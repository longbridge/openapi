use std::fmt::Display;

use jni::{
    JNIEnv,
    errors::Result,
    objects::{JObject, JThrowable, JValue},
};

use crate::{
    init::{LONG_CLASS, OPENAPI_EXCEPTION_CLASS},
    types::IntoJValue,
};

#[derive(Debug, thiserror::Error)]
pub(crate) enum JniError {
    #[error(transparent)]
    Jni(#[from] jni::errors::Error),
    #[error(transparent)]
    OpenApi(#[from] Box<longbridge::Error>),
    #[error("{0}")]
    Other(String),
}

impl From<longbridge::Error> for JniError {
    fn from(e: longbridge::Error) -> Self {
        JniError::OpenApi(Box::new(e))
    }
}

impl From<longbridge::oauth::OAuthError> for JniError {
    fn from(e: longbridge::oauth::OAuthError) -> Self {
        JniError::Other(e.to_string())
    }
}

impl JniError {
    fn into_runtime_error_object<'a>(
        env: &mut JNIEnv<'a>,
        err: impl Display,
    ) -> Result<JObject<'a>> {
        let jmsg: JObject = env.new_string(err.to_string())?.into();
        env.new_object(
            "java/lang/RuntimeException",
            "(Ljava/lang/String;)V",
            &[JValue::from(&jmsg)],
        )
    }

    fn throw_runtime_error(env: &mut JNIEnv, err: impl Display) -> Result<()> {
        let err = JThrowable::from(Self::into_runtime_error_object(env, err)?);
        env.throw(err)?;
        Ok(())
    }

    fn into_openapi_error_object<'a>(
        env: &mut JNIEnv<'a>,
        err: longbridge::Error,
    ) -> Result<JObject<'a>> {
        let exception_cls = OPENAPI_EXCEPTION_CLASS.get().unwrap();
        let err = err.into_simple_error();

        let kind = err.kind().into_jvalue(env)?;
        let code = match err.code() {
            Some(code) => {
                env.new_object(LONG_CLASS.get().unwrap(), "(J)V", &[JValue::from(code)])?
            }
            None => JObject::null(),
        };
        let message: JObject = env.new_string(err.message())?.into();

        env.new_object(
            exception_cls,
            "(Lcom/longbridge/ErrorKind;Ljava/lang/Long;Ljava/lang/String;)V",
            &[kind.borrow(), JValue::from(&code), JValue::from(&message)],
        )
    }

    fn throw_openapi_error(env: &mut JNIEnv, err: longbridge::Error) -> Result<()> {
        let err = JThrowable::from(Self::into_openapi_error_object(env, err)?);
        env.throw(err)?;
        Ok(())
    }

    pub(crate) fn into_error_object<'a>(self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        let res = match self {
            JniError::Jni(err) => Self::into_runtime_error_object(env, err),
            JniError::OpenApi(err) => Self::into_openapi_error_object(env, *err),
            JniError::Other(err) => Self::into_runtime_error_object(env, err),
        };
        match res {
            Ok(obj) => obj,
            Err(_) => {
                // Building the error object failed (e.g. an exception is already
                // pending). Fall back to a null error rather than panicking:
                // this runs on a background callback thread, where a panic would
                // unwind across the FFI boundary and abort the whole JVM.
                let _ = env.exception_clear();
                JObject::null()
            }
        }
    }

    fn throw(self, env: &mut JNIEnv) {
        let res = match self {
            JniError::Jni(err) => Self::throw_runtime_error(env, err),
            JniError::OpenApi(err) => Self::throw_openapi_error(env, *err),
            JniError::Other(err) => Self::throw_runtime_error(env, err),
        };
        if let Err(err) = res {
            // Mapping the error to a Java exception failed. Fall back to a plain
            // RuntimeException instead of `fatal_error`, which would abort the
            // JVM outright.
            if !env.exception_check().unwrap_or(false) {
                let _ = env.throw_new("java/lang/RuntimeException", err.to_string());
            }
        }
    }
}

pub(crate) fn jni_result<'a, F, T>(env: &'a mut JNIEnv, err_value: T, f: F) -> T
where
    F: FnOnce(&mut JNIEnv) -> std::result::Result<T, JniError> + 'a,
{
    match f(env) {
        Ok(value) => value,
        Err(err) => {
            // If a Java exception is already pending (e.g. thrown directly by a
            // `from_jvalue` conversion via `throw_illegal_argument`, or raised by
            // a Java method we called), keep it instead of throwing a second one.
            if !env.exception_check().unwrap_or(false) {
                err.throw(env);
            }
            err_value
        }
    }
}

/// Throw a `java.lang.IllegalArgumentException` with the given message.
///
/// Used by the JNI value conversions to reject invalid input (null where a
/// value is required, an unrecognized enum constant, an out-of-range date, …)
/// as a catchable Java exception instead of panicking across the FFI boundary
/// (which would abort the JVM). It is best-effort: if a Java exception is
/// already pending it is left untouched.
pub(crate) fn throw_illegal_argument(env: &mut JNIEnv, msg: impl AsRef<str>) {
    if !env.exception_check().unwrap_or(false) {
        let _ = env.throw_new("java/lang/IllegalArgumentException", msg.as_ref());
    }
}

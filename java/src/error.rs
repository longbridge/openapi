use std::fmt::Display;

use jni::{
    JNIEnv,
    errors::Result,
    objects::{JObject, JThrowable, JValue},
};

use crate::init::{LONG_CLASS, OPENAPI_EXCEPTION_CLASS};

#[derive(Debug, thiserror::Error)]
pub(crate) enum JniError {
    #[error(transparent)]
    Jni(#[from] jni::errors::Error),
    #[error(transparent)]
    OpenApi(#[from] longport::Error),
    #[error("{0}")]
    Other(String),
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
        err: longport::Error,
    ) -> Result<JObject<'a>> {
        let exception_cls = OPENAPI_EXCEPTION_CLASS.get().unwrap();
        let err = err.into_simple_error();

        let code = match err.code() {
            Some(code) => {
                env.new_object(LONG_CLASS.get().unwrap(), "(J)V", &[JValue::from(code)])?
            }
            None => JObject::null(),
        };
        let message: JObject = env.new_string(err.message())?.into();

        env.new_object(
            exception_cls,
            "(Ljava/lang/Long;Ljava/lang/String;)V",
            &[JValue::from(&code), JValue::from(&message)],
        )
    }

    fn throw_openapi_error(env: &mut JNIEnv, err: longport::Error) -> Result<()> {
        let err = JThrowable::from(Self::into_openapi_error_object(env, err)?);
        env.throw(err)?;
        Ok(())
    }

    pub(crate) fn into_error_object<'a>(self, env: &mut JNIEnv<'a>) -> JObject<'a> {
        match self {
            JniError::Jni(err) => Self::into_runtime_error_object(env, err),
            JniError::OpenApi(err) => Self::into_openapi_error_object(env, err),
            JniError::Other(err) => Self::into_runtime_error_object(env, err),
        }
        .expect("to error object")
    }

    fn throw(self, env: &mut JNIEnv) {
        let res = match self {
            JniError::Jni(err) => Self::throw_runtime_error(env, err),
            JniError::OpenApi(err) => Self::throw_openapi_error(env, err),
            JniError::Other(err) => Self::throw_runtime_error(env, err),
        };
        if let Err(err) = res {
            env.fatal_error(err.to_string());
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
            err.throw(env);
            err_value
        }
    }
}

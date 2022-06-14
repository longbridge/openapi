use std::borrow::Cow;

use jni::{
    errors::Result,
    objects::{JObject, JValue},
    JNIEnv,
};

use crate::types::{FromJValue, IntoJValue, JClassName, JSignature};

impl<T: JClassName> JClassName for Option<T> {
    const CLASSNAME: &'static str = T::CLASSNAME;
}

impl<T: JSignature> JSignature for Option<T> {
    #[inline]
    fn signature() -> Cow<'static, str> {
        T::signature()
    }
}

impl<T: IntoJValue> IntoJValue for Option<T> {
    fn into_jvalue<'a>(self, env: &JNIEnv<'a>) -> Result<JValue<'a>> {
        match self {
            Some(value) => value.into_jvalue(env),
            None => Ok(JValue::from(JObject::null())),
        }
    }
}

impl<T: FromJValue> FromJValue for Option<T> {
    fn from_jvalue(env: &JNIEnv, value: JValue) -> Result<Self> {
        let obj = value.l()?;
        if !obj.is_null() {
            Ok(Some(T::from_jvalue(env, obj.into())?))
        } else {
            Ok(None)
        }
    }
}

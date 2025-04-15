use std::borrow::Cow;

use jni::{
    JNIEnv,
    errors::Result,
    objects::{JObject, JValueOwned},
};

use crate::types::{ClassLoader, FromJValue, IntoJValue, JSignature};

impl<T: ClassLoader> ClassLoader for Option<T> {
    fn init(env: &mut JNIEnv) {
        T::init(env)
    }

    fn class_ref() -> jni::objects::GlobalRef {
        T::class_ref()
    }
}

impl<T: JSignature> JSignature for Option<T> {
    #[inline]
    fn signature() -> Cow<'static, str> {
        T::signature()
    }
}

impl<T: IntoJValue> IntoJValue for Option<T> {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        match self {
            Some(value) => value.into_jvalue(env),
            None => Ok(JValueOwned::from(JObject::null())),
        }
    }
}

impl<T: FromJValue> FromJValue for Option<T> {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        let obj = value.l()?;
        if !obj.is_null() {
            Ok(Some(T::from_jvalue(env, obj.into())?))
        } else {
            Ok(None)
        }
    }
}

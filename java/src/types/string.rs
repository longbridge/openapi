use jni::{
    JNIEnv,
    errors::Result,
    objects::{JString, JValueOwned},
};

use crate::{
    init::STRING_CLASS,
    types::{ClassLoader, FromJValue, IntoJValue, JSignature},
};

impl ClassLoader for String {
    fn init(_env: &mut JNIEnv) {}

    fn class_ref() -> jni::objects::GlobalRef {
        STRING_CLASS.get().cloned().unwrap()
    }
}

impl JSignature for String {
    fn signature() -> std::borrow::Cow<'static, str> {
        "Ljava/lang/String;".into()
    }
}

impl FromJValue for String {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        let obj = value.l()?;
        if obj.is_null() {
            crate::error::throw_illegal_argument(env, "String argument must not be null");
            return Err(jni::errors::Error::JavaException);
        }
        let s = JString::from(obj);
        let str = env.get_string(&s)?;
        match str.to_str() {
            Ok(str) => Ok(str.to_string()),
            Err(_) => {
                crate::error::throw_illegal_argument(env, "string is not valid UTF-8");
                Err(jni::errors::Error::JavaException)
            }
        }
    }
}

impl IntoJValue for String {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        env.new_string(self).map(JValueOwned::from)
    }
}

impl crate::types::ClassLoader for serde_json::Value {
    fn init(_env: &mut JNIEnv) {}
    fn class_ref() -> jni::objects::GlobalRef {
        STRING_CLASS.get().cloned().unwrap()
    }
}

impl crate::types::JSignature for serde_json::Value {
    fn signature() -> std::borrow::Cow<'static, str> {
        "Ljava/lang/String;".into()
    }
}

impl IntoJValue for serde_json::Value {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        let s = serde_json::to_string(&self).unwrap_or_default();
        env.new_string(s).map(JValueOwned::from)
    }
}

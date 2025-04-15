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
        let s = JString::from(obj);
        let str = env.get_string(&s)?;
        Ok(str
            .to_str()
            .map(ToString::to_string)
            .expect("valid utf8 string"))
    }
}

impl IntoJValue for String {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        env.new_string(self).map(JValueOwned::from)
    }
}

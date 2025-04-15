use std::borrow::Cow;

use jni::{JNIEnv, errors::Result, objects::JValueOwned};
use longport::quote::SubFlags;

use crate::types::{IntoJValue, JSignature};

impl JSignature for SubFlags {
    fn signature() -> Cow<'static, str> {
        "I".into()
    }
}

impl IntoJValue for SubFlags {
    fn into_jvalue<'a>(self, _env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        Ok(JValueOwned::from(self.bits() as i32))
    }
}

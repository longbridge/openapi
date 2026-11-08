use std::borrow::Cow;

use jni::{JNIEnv, errors::Result, objects::JValueOwned};
use longbridge::Decimal;

use crate::{
    init::DECIMAL_CLASS,
    types::{ClassLoader, FromJValue, IntoJValue, JSignature},
};

impl ClassLoader for Decimal {
    fn init(_env: &mut JNIEnv) {}

    fn class_ref() -> jni::objects::GlobalRef {
        DECIMAL_CLASS.get().cloned().unwrap()
    }
}

impl JSignature for Decimal {
    fn signature() -> Cow<'static, str> {
        "Ljava/math/BigDecimal;".into()
    }
}

impl FromJValue for Decimal {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        let obj = value.l()?;
        if obj.is_null() {
            crate::error::throw_illegal_argument(env, "BigDecimal argument must not be null");
            return Err(jni::errors::Error::JavaException);
        }
        let value = env.call_method(obj, "toString", "()Ljava/lang/String;", &[])?;
        let value = String::from_jvalue(env, value)?;
        match value.parse() {
            Ok(decimal) => Ok(decimal),
            Err(_) => {
                crate::error::throw_illegal_argument(
                    env,
                    format!("value \"{value}\" is not a valid / representable decimal"),
                );
                Err(jni::errors::Error::JavaException)
            }
        }
    }
}

impl IntoJValue for Decimal {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        let str = env.new_string(self.to_string())?;
        let obj = env.new_object(
            DECIMAL_CLASS.get().unwrap(),
            "(Ljava/lang/String;)V",
            &[JValueOwned::from(str).borrow()],
        )?;
        Ok(JValueOwned::from(obj))
    }
}

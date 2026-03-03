use std::borrow::Cow;

use jni::{
    JNIEnv,
    errors::Result,
    objects::{JValueOwned, ReleaseMode},
    sys::{jboolean, jint, jlong},
};

use crate::types::{FromJValue, IntoJValue, JSignature};

pub(crate) struct PrimaryArray<T>(pub(crate) Vec<T>);

impl<T: JSignature> JSignature for PrimaryArray<T> {
    fn signature() -> Cow<'static, str> {
        format!("[{}", T::signature()).into()
    }
}

impl FromJValue for PrimaryArray<i32> {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        unsafe {
            let value = value.l()?;
            if value.is_null() {
                return Ok(PrimaryArray(Vec::new()));
            }
            let value = value.into();
            let array = env.get_array_elements::<jint>(&value, ReleaseMode::CopyBack)?;
            Ok(PrimaryArray(
                std::slice::from_raw_parts(array.as_ptr(), array.len()).to_vec(),
            ))
        }
    }
}

impl IntoJValue for PrimaryArray<i32> {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        let array = env.new_int_array(self.0.len() as i32)?;
        env.set_int_array_region(&array, 0, &self.0)?;
        Ok(array.into())
    }
}

impl FromJValue for PrimaryArray<i64> {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        unsafe {
            let value = value.l()?;
            if value.is_null() {
                return Ok(PrimaryArray(Vec::new()));
            }
            let value = value.into();
            let array = env.get_array_elements::<jlong>(&value, ReleaseMode::CopyBack)?;
            Ok(PrimaryArray(
                std::slice::from_raw_parts(array.as_ptr(), array.len()).to_vec(),
            ))
        }
    }
}

impl IntoJValue for PrimaryArray<i64> {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        let array = env.new_long_array(self.0.len() as i32)?;
        env.set_long_array_region(&array, 0, &self.0)?;
        Ok(array.into())
    }
}

impl FromJValue for PrimaryArray<bool> {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        unsafe {
            let value = value.l()?;
            if value.is_null() {
                return Ok(PrimaryArray(Vec::new()));
            }
            let value = value.into();
            let array = env.get_array_elements::<jboolean>(&value, ReleaseMode::CopyBack)?;
            Ok(PrimaryArray(
                std::slice::from_raw_parts(array.as_ptr(), array.len())
                    .iter()
                    .copied()
                    .map(|value| value > 0)
                    .collect(),
            ))
        }
    }
}

impl IntoJValue for PrimaryArray<bool> {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        let array = env.new_boolean_array(self.0.len() as i32)?;
        let buf = self
            .0
            .into_iter()
            .map(|value| if value { 1u8 } else { 0 })
            .collect::<Vec<_>>();
        env.set_boolean_array_region(&array, 0, &buf)?;
        Ok(array.into())
    }
}

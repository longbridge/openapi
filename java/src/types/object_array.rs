use std::borrow::Cow;

use jni::{
    errors::Result,
    objects::{JObject, JValue},
    JNIEnv,
};

use crate::types::{FromJValue, IntoJValue, JClassName, JSignature};

pub(crate) struct ObjectArray<T>(pub(crate) Vec<T>);

impl<T: JSignature> JSignature for ObjectArray<T> {
    fn signature() -> Cow<'static, str> {
        format!("[{}", T::signature()).into()
    }
}

impl<T: FromJValue> FromJValue for ObjectArray<T> {
    fn from_jvalue(env: &JNIEnv, value: JValue) -> Result<Self> {
        let obj = value.l()?;
        if obj.is_null() {
            return Ok(ObjectArray(Vec::new()));
        }

        let array = obj.into_inner();
        let len = env.get_array_length(array)?;
        let mut res = Vec::with_capacity(len as usize);

        for i in 0..len {
            let obj = env.get_object_array_element(array, i)?;
            let value = T::from_jvalue(env, JValue::from(obj))?;
            res.push(value);
        }

        Ok(ObjectArray(res))
    }
}

impl<T: IntoJValue + JClassName> IntoJValue for ObjectArray<T> {
    fn into_jvalue<'a>(self, env: &JNIEnv<'a>) -> Result<JValue<'a>> {
        let array = env.new_object_array(self.0.len() as i32, T::CLASSNAME, JObject::null())?;
        for (i, obj) in self.0.into_iter().enumerate() {
            env.set_object_array_element(array, i as i32, obj.into_jvalue(env)?.l()?)?;
        }
        Ok(array.into())
    }
}

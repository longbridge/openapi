use std::borrow::Cow;

use jni::{
    JNIEnv,
    errors::Result,
    objects::{JObject, JObjectArray, JValueOwned},
};

use crate::types::{ClassLoader, FromJValue, IntoJValue, JSignature};

pub(crate) struct ObjectArray<T>(pub(crate) Vec<T>);

impl<T: JSignature> JSignature for ObjectArray<T> {
    fn signature() -> Cow<'static, str> {
        format!("[{}", T::signature()).into()
    }
}

impl<T: FromJValue> FromJValue for ObjectArray<T> {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        let obj = value.l()?;
        if obj.is_null() {
            return Ok(ObjectArray(Vec::new()));
        }

        let array: JObjectArray = obj.into();
        let len = env.get_array_length(&array)?;
        let mut res = Vec::with_capacity(len as usize);

        for i in 0..len {
            let obj = env.get_object_array_element(&array, i)?;
            let value = T::from_jvalue(env, JValueOwned::from(obj))?;
            res.push(value);
        }

        Ok(ObjectArray(res))
    }
}

impl<T: IntoJValue + ClassLoader> IntoJValue for ObjectArray<T> {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        let array = env.new_object_array(self.0.len() as i32, &T::class_ref(), JObject::null())?;
        for (i, obj) in self.0.into_iter().enumerate() {
            let value = obj.into_jvalue(env)?.l()?;
            env.set_object_array_element(&array, i as i32, value)?;
        }
        Ok(array.into())
    }
}

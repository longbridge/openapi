mod classes;
mod datetime;
mod decimal;
mod enum_types;
mod object_array;
mod optional;
mod primary_array;
mod primary_types;
mod string;
mod sub_flags;

use std::borrow::Cow;

use jni::{
    errors::Result,
    objects::{JObject, JValue},
    strings::JNIString,
    JNIEnv,
};

pub(crate) use self::{object_array::ObjectArray, primary_array::PrimaryArray};

pub(crate) trait FromJValue: Sized {
    fn from_jvalue(env: &JNIEnv, value: JValue) -> Result<Self>;
}

pub(crate) trait IntoJValue {
    fn into_jvalue<'a>(self, env: &JNIEnv<'a>) -> Result<JValue<'a>>;
}

impl IntoJValue for () {
    #[inline]
    fn into_jvalue<'a>(self, _env: &JNIEnv<'a>) -> Result<JValue<'a>> {
        Ok(JValue::from(JObject::null()))
    }
}

pub(crate) trait JClassName {
    const CLASSNAME: &'static str;
}

pub(crate) trait JSignature {
    fn signature() -> Cow<'static, str>;
}

#[inline]
pub(crate) fn set_field<'a, O, N, T>(env: &JNIEnv<'a>, obj: O, name: N, value: T) -> Result<()>
where
    O: Into<JObject<'a>>,
    N: Into<JNIString>,
    T: IntoJValue + JSignature,
{
    env.set_field(obj, name, T::signature(), value.into_jvalue(env)?)
}

#[inline]
pub(crate) fn get_field<'a, O, N, T>(env: &JNIEnv<'a>, obj: O, name: N) -> Result<T>
where
    O: Into<JObject<'a>>,
    N: Into<JNIString>,
    T: FromJValue + JSignature,
{
    T::from_jvalue(env, env.get_field(obj, name, T::signature())?)
}

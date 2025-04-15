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
    JNIEnv,
    errors::Result,
    objects::{GlobalRef, JObject, JValueOwned},
    strings::JNIString,
};

pub(crate) use self::{
    classes::{
        CreateWatchlistGroupResponse, SecurityCalcIndex, StockPosition, StockPositionChannel,
        StockPositionsResponse,
    },
    object_array::ObjectArray,
    primary_array::PrimaryArray,
};

pub(crate) trait ClassLoader {
    fn init(env: &mut JNIEnv);
    fn class_ref() -> GlobalRef;
}

pub(crate) trait FromJValue: Sized {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self>;
}

pub(crate) trait IntoJValue {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>>;
}

impl IntoJValue for () {
    #[inline]
    fn into_jvalue<'a>(self, _env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        Ok(JValueOwned::from(JObject::null()))
    }
}

pub(crate) trait JSignature {
    fn signature() -> Cow<'static, str>;
}

#[inline]
pub(crate) fn set_field<'a, O, N, T>(env: &mut JNIEnv<'a>, obj: O, name: N, value: T) -> Result<()>
where
    O: AsRef<JObject<'a>>,
    N: Into<JNIString>,
    T: IntoJValue + JSignature,
{
    let value = value.into_jvalue(env)?;
    env.set_field(obj, name, T::signature(), value.borrow())
}

#[inline]
pub(crate) fn get_field<'a, O, N, T>(env: &mut JNIEnv<'_>, obj: O, name: N) -> Result<T>
where
    O: AsRef<JObject<'a>>,
    N: Into<JNIString>,
    T: FromJValue + JSignature,
{
    let value = env.get_field(obj, name, T::signature())?;
    T::from_jvalue(env, value)
}

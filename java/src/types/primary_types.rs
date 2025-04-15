use std::borrow::Cow;

use jni::{JNIEnv, errors::Result, objects::JValueOwned};

use crate::types::{FromJValue, IntoJValue, JSignature};

impl JSignature for i32 {
    fn signature() -> Cow<'static, str> {
        "I".into()
    }
}

impl FromJValue for i32 {
    #[inline]
    fn from_jvalue(_env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        value.i()
    }
}

impl IntoJValue for i32 {
    #[inline]
    fn into_jvalue<'a>(self, _env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        Ok(JValueOwned::from(self))
    }
}

impl JSignature for i64 {
    fn signature() -> Cow<'static, str> {
        "J".into()
    }
}

impl FromJValue for i64 {
    #[inline]
    fn from_jvalue(_env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        value.j()
    }
}

impl IntoJValue for i64 {
    #[inline]
    fn into_jvalue<'a>(self, _env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        Ok(JValueOwned::from(self))
    }
}

impl JSignature for bool {
    fn signature() -> Cow<'static, str> {
        "Z".into()
    }
}

impl FromJValue for bool {
    #[inline]
    fn from_jvalue(_env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        value.z()
    }
}

impl IntoJValue for bool {
    #[inline]
    fn into_jvalue<'a>(self, _env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        Ok(JValueOwned::from(self))
    }
}

impl JSignature for f64 {
    fn signature() -> Cow<'static, str> {
        "D".into()
    }
}

impl FromJValue for f64 {
    #[inline]
    fn from_jvalue(_env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        value.d()
    }
}

impl IntoJValue for f64 {
    #[inline]
    fn into_jvalue<'a>(self, _env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        Ok(JValueOwned::from(self))
    }
}

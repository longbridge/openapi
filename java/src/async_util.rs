use std::future::Future;

use jni::{
    JNIEnv,
    errors::Result,
    objects::{JObject, JValue},
};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("create tokio runtime")
});

use crate::{error::JniError, types::IntoJValue};

fn async_complete<'a, T, V>(env: &mut JNIEnv<'a>, callback: T, value: V) -> Result<()>
where
    T: AsRef<JObject<'a>>,
    V: AsRef<JObject<'a>>,
{
    env.call_method(
        callback,
        "callback",
        "(Ljava/lang/Object;Ljava/lang/Object;)V",
        &[JValue::from(&JObject::null()), JValue::from(value.as_ref())],
    )
    .map(|_| ())
}

fn async_error<'a, T, V>(env: &mut JNIEnv<'a>, callback: T, err: V) -> Result<()>
where
    T: AsRef<JObject<'a>>,
    V: AsRef<JObject<'a>>,
{
    env.call_method(
        callback,
        "callback",
        "(Ljava/lang/Object;Ljava/lang/Object;)V",
        &[JValue::from(err.as_ref()), JValue::from(&JObject::null())],
    )
    .map(|_| ())
}

pub(crate) fn execute<T, F>(env: &JNIEnv<'_>, callback: JObject, fut: F) -> Result<()>
where
    T: IntoJValue,
    F: Future<Output = ::std::result::Result<T, JniError>> + Send + 'static,
{
    let jvm = env.get_java_vm()?;
    let callback = env.new_global_ref(callback)?;

    let _guard = RUNTIME.enter();
    tokio::spawn(async move {
        let res = fut.await;
        let mut env = jvm.attach_current_thread().unwrap();
        match res {
            Ok(value) => {
                let value = value.into_jvalue(&mut env).unwrap().l().unwrap();
                let _ = async_complete(&mut env, &*callback, value);
            }
            Err(err) => {
                let err = err.into_error_object(&mut env);
                let _ = async_error(&mut env, &*callback, err);
            }
        }
    });

    Ok(())
}

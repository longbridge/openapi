use jni::{
    JNIEnv,
    objects::{JClass, JObject, JString},
    sys::{jint, jlong},
};
use longbridge::oauth::{OAuth, OAuthBuilder};

use crate::{async_util, error::jni_result, types::JavaLong};

// ── OAuth native handle
// ───────────────────────────────────────────────────────

/// Asynchronously build an OAuth client via OAuthBuilder.
///
/// `callback_port == 0` means "use the default (60355)".
/// `openUrlCallback` must be a `java.util.function.Consumer<String>`.
/// On success the async `callback` receives a heap-allocated `*mut OAuth`
/// cast to `jlong`; the caller is responsible for calling `freeOAuth`.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_oauthBuild(
    mut env: JNIEnv,
    _class: JClass,
    client_id: JString,
    callback_port: jint,
    open_url_callback: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        use crate::types::FromJValue;
        let client_id = String::from_jvalue(env, client_id.into())?;
        let jvm = env.get_java_vm()?;
        let open_url_callback = env.new_global_ref(open_url_callback)?;

        async_util::execute(env, callback, async move {
            let mut builder = OAuthBuilder::new(client_id);
            if callback_port > 0 {
                builder = builder.callback_port(callback_port as u16);
            }
            let oauth = builder
                .build(move |url| {
                    if let Ok(mut env) = jvm.attach_current_thread()
                        && let Ok(j_url) = env.new_string(url)
                    {
                        let _ = env.call_method(
                            open_url_callback.as_obj(),
                            "accept",
                            "(Ljava/lang/Object;)V",
                            &[jni::objects::JValue::from(&j_url)],
                        );
                    }
                })
                .await
                .map_err(|e| crate::error::JniError::Other(e.to_string()))?;
            Ok(JavaLong::from(Box::into_raw(Box::new(oauth)) as jlong))
        })?;
        Ok(())
    })
}

/// Free an OAuth pointer.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeOAuth(
    _env: JNIEnv,
    _class: JClass,
    oauth: jlong,
) {
    drop(Box::from_raw(oauth as *mut OAuth));
}

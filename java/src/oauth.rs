use jni::{
    JNIEnv,
    objects::{JClass, JObject, JString},
    sys::jlong,
};
use longport::oauth::OAuth;

use crate::{async_util, error::jni_result, types::set_field};

// ── OAuthToken native handle
// ──────────────────────────────────────────────────

struct OAuthTokenPtr(*mut longport::oauth::OAuthToken);

impl crate::types::IntoJValue for OAuthTokenPtr {
    fn into_jvalue<'a>(
        self,
        env: &mut JNIEnv<'a>,
    ) -> jni::errors::Result<jni::objects::JValueOwned<'a>> {
        let obj = env.new_object(crate::init::OAUTH_TOKEN_CLASS.get().unwrap(), "()V", &[])?;
        set_field(env, &obj, "raw", self.0 as i64)?;
        Ok(jni::objects::JValueOwned::from(obj))
    }
}

fn into_token_ptr(token: longport::oauth::OAuthToken) -> OAuthTokenPtr {
    OAuthTokenPtr(Box::into_raw(Box::new(token)))
}

// ── OAuth native handle
// ───────────────────────────────────────────────────────

/// Create a new OAuth 2.0 client, returning an opaque pointer stored as `long`
/// in `com.longport.OAuth.raw`.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_newOAuth(
    mut env: JNIEnv,
    _class: JClass,
    client_id: JString,
) -> jlong {
    jni_result(&mut env, 0, |env| {
        use crate::types::FromJValue;
        let client_id = String::from_jvalue(env, client_id.into())?;
        Ok(Box::into_raw(Box::new(OAuth::new(client_id))) as jlong)
    })
}

/// Free an OAuth pointer.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_freeOAuth(
    _env: JNIEnv,
    _class: JClass,
    oauth: jlong,
) {
    drop(Box::from_raw(oauth as *mut OAuth));
}

// ── OAuthToken native methods
// ─────────────────────────────────────────────────

/// Free an OAuthToken pointer.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_freeOAuthToken(
    _env: JNIEnv,
    _class: JClass,
    token: jlong,
) {
    drop(Box::from_raw(token as *mut longport::oauth::OAuthToken));
}

/// Returns true if the token has expired.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_oauthTokenIsExpired(
    _env: JNIEnv,
    _class: JClass,
    token: jlong,
) -> bool {
    let token = &*(token as *const longport::oauth::OAuthToken);
    token.is_expired()
}

/// Returns true if the token will expire within 1 hour.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_oauthTokenExpiresSoon(
    _env: JNIEnv,
    _class: JClass,
    token: jlong,
) -> bool {
    let token = &*(token as *const longport::oauth::OAuthToken);
    token.expires_soon()
}

// ── OAuth native methods
// ──────────────────────────────────────────────────────

/// Start the OAuth 2.0 authorization flow (async).
///
/// `openUrlCallback` must be a `java.util.function.Consumer<String>` — its
/// `accept(String)` method is called with the authorization URL.
/// On success the async `callback` receives a `com/longport/OAuthToken` object.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_oauthAuthorize(
    mut env: JNIEnv,
    _class: JClass,
    oauth: jlong,
    open_url_callback: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let client_id = (*(oauth as *const OAuth)).client_id().to_string();
        let jvm = env.get_java_vm()?;
        let open_url_callback = env.new_global_ref(open_url_callback)?;

        async_util::execute(env, callback, async move {
            let token = OAuth::new(client_id)
                .authorize(move |url| {
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
                .await?;
            Ok(into_token_ptr(token))
        })?;
        Ok(())
    })
}

/// Refresh an access token (async).
///
/// On success the async `callback` receives a new `com/longport/OAuthToken`.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longport_SdkNative_oauthRefresh(
    mut env: JNIEnv,
    _class: JClass,
    oauth: jlong,
    token: jlong,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let client_id = (*(oauth as *const OAuth)).client_id().to_string();
        let existing_token = (*(token as *const longport::oauth::OAuthToken)).clone();

        async_util::execute(env, callback, async move {
            let new_token = OAuth::new(client_id).refresh(&existing_token).await?;
            Ok(into_token_ptr(new_token))
        })?;
        Ok(())
    })
}

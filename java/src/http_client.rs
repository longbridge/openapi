use std::collections::HashMap;

use jni::{
    JNIEnv,
    objects::{JClass, JObject, JString},
    sys::jlong,
};
use longbridge::httpclient::{HttpClient, HttpClientConfig, Json, Method};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    async_util,
    error::{JniError, jni_result},
    types::FromJValue,
};

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newHttpClientFromApikey(
    mut env: JNIEnv,
    _class: JClass,
    app_key: JString,
    app_secret: JString,
    access_token: JString,
    http_url: JString,
) -> jlong {
    jni_result(&mut env, 0, |env| {
        let app_key = String::from_jvalue(env, app_key.into())?;
        let app_secret = String::from_jvalue(env, app_secret.into())?;
        let access_token = String::from_jvalue(env, access_token.into())?;
        let mut config = HttpClientConfig::from_apikey(app_key, app_secret, access_token);
        if !http_url.is_null() {
            config = config.http_url(String::from_jvalue(env, http_url.into())?);
        }
        Ok(Box::into_raw(Box::new(HttpClient::new(config))) as jlong)
    })
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_longbridge_SdkNative_newHttpClientFromApikeyEnv(
    mut env: JNIEnv,
    _class: JClass,
) -> jlong {
    jni_result(&mut env, 0, |_env| {
        let config = HttpClient::new(
            longbridge::httpclient::HttpClientConfig::from_apikey_env()
                .map_err(longbridge::Error::HttpClient)?,
        );
        Ok(Box::into_raw(Box::new(config)) as jlong)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newHttpClientFromOauth(
    mut env: JNIEnv,
    _class: JClass,
    oauth: jlong,
    http_url: JString,
) -> jlong {
    jni_result(&mut env, 0, |env| {
        let oauth = &*(oauth as *const longbridge::oauth::OAuth);
        let mut config = HttpClientConfig::from_oauth(oauth.clone());
        if !http_url.is_null() {
            config = config.http_url(String::from_jvalue(env, http_url.into())?);
        }
        Ok(Box::into_raw(Box::new(HttpClient::new(config))) as jlong)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeHttpClient(
    _env: JNIEnv,
    _class: JClass,
    http_client: i64,
) {
    let _ = Box::from_raw(http_client as *mut HttpClient);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_httpClientRequest(
    mut env: JNIEnv,
    _class: JClass,
    http_client: i64,
    request: JString,
    callback: JObject,
) {
    #[derive(Debug, Deserialize)]
    struct Request {
        method: String,
        path: String,
        data: Option<Value>,
        headers: Option<HashMap<String, String>>,
    }

    jni_result(&mut env, (), |env| {
        let http_client = &*(http_client as *const HttpClient);
        let request = String::from_jvalue(env, request.into())?;
        let request: Request =
            serde_json::from_str(&request).map_err(|err| JniError::Other(err.to_string()))?;

        async_util::execute(env, callback, async move {
            let mut req = http_client
                .request(
                    request
                        .method
                        .to_uppercase()
                        .parse::<Method>()
                        .map_err(|err| JniError::Other(err.to_string()))?,
                    request.path,
                )
                .response::<String>();

            if let Some(headers) = request.headers {
                for (key, value) in headers {
                    req = req.header(&key, &value);
                }
            }

            let resp = match request.data {
                Some(req_data) => req
                    .body(Json(req_data))
                    .send()
                    .await
                    .map_err(|err| JniError::from(longbridge::Error::HttpClient(err)))?,
                None => req
                    .send()
                    .await
                    .map_err(|err| JniError::from(longbridge::Error::HttpClient(err)))?,
            };

            Ok(resp)
        })?;
        Ok(())
    })
}

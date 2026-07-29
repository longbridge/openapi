use std::sync::Arc;

use jni::{
    JNIEnv, JavaVM,
    objects::{GlobalRef, JClass, JObject, JString, JValue},
};
use longbridge::{
    AgentContext, Config,
    agent::{self, AnswersByToolCall, ConversationStreamEvent, GetAgentsOptions},
};

use crate::{
    async_util,
    error::{JniError, jni_result},
    init::CONVERSATION_STREAM_SUBSCRIPTION_CLASS,
    types::{FromJValue, IntoJValue, JavaInteger, get_field},
};

struct ContextObj {
    ctx: AgentContext,
}

/// Everything needed to deliver `Flow.Subscriber` callbacks (`onNext` /
/// `onError` / `onComplete`) from a background tokio task back into the JVM.
/// Shared (via `Arc`) between the three closures handed to
/// [`agent::ConversationStreamSubscription::spawn`], since they all target
/// the same `Subscriber` instance.
struct SubscriberSink {
    jvm: JavaVM,
    subscriber: GlobalRef,
}

fn deliver_on_next(sink: &SubscriberSink, event: ConversationStreamEvent) {
    let Ok(mut env) = sink.jvm.attach_current_thread() else {
        return;
    };
    if let Ok(value) = event.into_jvalue(&mut env) {
        // `Flow.Subscriber<T>.onNext(T)` is generic; the JVM always exposes a
        // bridge method with the erased `(Ljava/lang/Object;)V` descriptor
        // for any concrete implementation, so this is the correct signature
        // to target regardless of what `T` the caller's `Subscriber` names.
        let _ = env.call_method(
            &sink.subscriber,
            "onNext",
            "(Ljava/lang/Object;)V",
            &[value.borrow()],
        );
    }
}

fn deliver_on_error(sink: &SubscriberSink, err: longbridge::Error) {
    let Ok(mut env) = sink.jvm.attach_current_thread() else {
        return;
    };
    let err_obj = JniError::from(err).into_error_object(&mut env);
    let _ = env.call_method(
        &sink.subscriber,
        "onError",
        "(Ljava/lang/Throwable;)V",
        &[JValue::from(&err_obj)],
    );
}

fn deliver_on_complete(sink: &SubscriberSink) {
    let Ok(mut env) = sink.jvm.attach_current_thread() else {
        return;
    };
    let _ = env.call_method(&sink.subscriber, "onComplete", "()V", &[]);
}

fn new_subscription_object<'a>(
    env: &mut JNIEnv<'a>,
    handle: i64,
) -> jni::errors::Result<JObject<'a>> {
    let cls = CONVERSATION_STREAM_SUBSCRIPTION_CLASS.get().unwrap();
    env.new_object(cls, "(J)V", &[JValue::from(handle)])
}

/// Finish a successful `subscribe()`: box up `subscription` behind an opaque
/// handle, wrap it in a Java `ConversationStreamSubscription`, and deliver it
/// via `onSubscribe`. If attaching to the JVM or constructing the Java object
/// fails, the boxed subscription is freed immediately so it isn't leaked (the
/// subscriber will simply never hear back, which is the best we can do at
/// that point).
fn finish_subscribe_success(
    sink: Arc<SubscriberSink>,
    subscription: agent::ConversationStreamSubscription,
) {
    let handle = Box::into_raw(Box::new(subscription)) as i64;

    let Ok(mut env) = sink.jvm.attach_current_thread() else {
        unsafe {
            let _ = Box::from_raw(handle as *mut agent::ConversationStreamSubscription);
        }
        return;
    };

    match new_subscription_object(&mut env, handle) {
        Ok(obj) => {
            let _ = env.call_method(
                &sink.subscriber,
                "onSubscribe",
                "(Ljava/util/concurrent/Flow$Subscription;)V",
                &[JValue::from(&obj)],
            );
        }
        Err(_) => unsafe {
            let _ = Box::from_raw(handle as *mut agent::ConversationStreamSubscription);
        },
    }
}

fn read_get_agents_options(
    env: &mut JNIEnv,
    opts: &JObject,
) -> jni::errors::Result<Option<GetAgentsOptions>> {
    if opts.is_null() {
        return Ok(None);
    }

    let mut new_opts = GetAgentsOptions::new();
    let page: Option<JavaInteger> = get_field(env, opts, "page")?;
    if let Some(page) = page {
        new_opts = new_opts.page(page.into());
    }
    let limit: Option<JavaInteger> = get_field(env, opts, "limit")?;
    if let Some(limit) = limit {
        new_opts = new_opts.limit(limit.into());
    }
    let name: Option<String> = get_field(env, opts, "name")?;
    if let Some(name) = name {
        new_opts = new_opts.name(name);
    }
    Ok(Some(new_opts))
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_newAgentContext(
    mut env: JNIEnv,
    _class: JClass,
    config: i64,
) -> i64 {
    jni_result(&mut env, 0i64, |_env| {
        let config = Arc::new((*(config as *const Config)).clone());
        Ok(Box::into_raw(Box::new(ContextObj {
            ctx: AgentContext::new(config),
        })) as i64)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeAgentContext(
    _env: JNIEnv,
    _class: JClass,
    ctx: i64,
) {
    let _ = Box::from_raw(ctx as *mut ContextObj);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_agentContextWorkspaces(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        async_util::execute(
            env,
            callback,
            async move { Ok(context.ctx.workspaces().await?) },
        )?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_agentContextAgents(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    workspace_id: JString,
    opts: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let workspace_id: String = FromJValue::from_jvalue(env, workspace_id.into())?;
        let opts = read_get_agents_options(env, &opts)?;
        async_util::execute(env, callback, async move {
            Ok(context.ctx.agents(workspace_id, opts).await?)
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_agentContextConversation(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    agent_id: JString,
    query: JString,
    chat_uid: JObject,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let agent_id: String = FromJValue::from_jvalue(env, agent_id.into())?;
        let query: String = FromJValue::from_jvalue(env, query.into())?;
        let chat_uid: Option<String> = FromJValue::from_jvalue(env, chat_uid.into())?;
        async_util::execute(env, callback, async move {
            Ok(crate::types::ConversationResponse::from(
                context.ctx.conversation(agent_id, query, chat_uid).await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_agentContextContinueConversation(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    agent_id: JString,
    chat_uid: JString,
    message_id: JString,
    answers_json: JString,
    callback: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let agent_id: String = FromJValue::from_jvalue(env, agent_id.into())?;
        let chat_uid: String = FromJValue::from_jvalue(env, chat_uid.into())?;
        let message_id: String = FromJValue::from_jvalue(env, message_id.into())?;
        let answers_json: String = FromJValue::from_jvalue(env, answers_json.into())?;
        let answers: AnswersByToolCall = serde_json::from_str(&answers_json).unwrap_or_default();
        async_util::execute(env, callback, async move {
            Ok(crate::types::ConversationResponse::from(
                context
                    .ctx
                    .continue_conversation(agent_id, chat_uid, message_id, answers)
                    .await?,
            ))
        })?;
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_agentContextConversationStreamSubscribe(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    agent_id: JString,
    query: JString,
    chat_uid: JObject,
    subscriber: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let ctx = context.ctx.clone();
        let agent_id: String = FromJValue::from_jvalue(env, agent_id.into())?;
        let query: String = FromJValue::from_jvalue(env, query.into())?;
        let chat_uid: Option<String> = FromJValue::from_jvalue(env, chat_uid.into())?;
        let jvm = env.get_java_vm()?;
        let subscriber = env.new_global_ref(subscriber)?;

        // `subscribe()` must return immediately without doing I/O (cold
        // Publisher semantics) — the actual connect happens on the shared
        // runtime, and `onSubscribe`/`onError` are delivered asynchronously
        // once it resolves.
        longbridge::runtime_handle().spawn(async move {
            let sink = Arc::new(SubscriberSink { jvm, subscriber });
            match ctx.conversation_streamed(agent_id, query, chat_uid).await {
                Ok(stream) => {
                    let (sink_next, sink_err, sink_complete) =
                        (sink.clone(), sink.clone(), sink.clone());
                    let subscription = agent::ConversationStreamSubscription::spawn(
                        stream,
                        move |event| deliver_on_next(&sink_next, event),
                        move |err| deliver_on_error(&sink_err, err),
                        move || deliver_on_complete(&sink_complete),
                    );
                    finish_subscribe_success(sink, subscription);
                }
                Err(err) => deliver_on_error(&sink, err),
            }
        });
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_agentContextContinueConversationStreamSubscribe(
    mut env: JNIEnv,
    _class: JClass,
    context: i64,
    agent_id: JString,
    chat_uid: JString,
    message_id: JString,
    answers_json: JString,
    subscriber: JObject,
) {
    jni_result(&mut env, (), |env| {
        let context = &*(context as *const ContextObj);
        let ctx = context.ctx.clone();
        let agent_id: String = FromJValue::from_jvalue(env, agent_id.into())?;
        let chat_uid: String = FromJValue::from_jvalue(env, chat_uid.into())?;
        let message_id: String = FromJValue::from_jvalue(env, message_id.into())?;
        let answers_json: String = FromJValue::from_jvalue(env, answers_json.into())?;
        let answers: AnswersByToolCall = serde_json::from_str(&answers_json).unwrap_or_default();
        let jvm = env.get_java_vm()?;
        let subscriber = env.new_global_ref(subscriber)?;

        longbridge::runtime_handle().spawn(async move {
            let sink = Arc::new(SubscriberSink { jvm, subscriber });
            match ctx
                .continue_conversation_streamed(agent_id, chat_uid, message_id, answers)
                .await
            {
                Ok(stream) => {
                    let (sink_next, sink_err, sink_complete) =
                        (sink.clone(), sink.clone(), sink.clone());
                    let subscription = agent::ConversationStreamSubscription::spawn(
                        stream,
                        move |event| deliver_on_next(&sink_next, event),
                        move |err| deliver_on_error(&sink_err, err),
                        move || deliver_on_complete(&sink_complete),
                    );
                    finish_subscribe_success(sink, subscription);
                }
                Err(err) => deliver_on_error(&sink, err),
            }
        });
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_conversationStreamSubscriptionRequest(
    _env: JNIEnv,
    _class: JClass,
    handle: i64,
    n: i64,
) {
    let subscription = &*(handle as *const agent::ConversationStreamSubscription);
    subscription.request(n.max(0) as u64);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_conversationStreamSubscriptionCancel(
    _env: JNIEnv,
    _class: JClass,
    handle: i64,
) {
    let subscription = &*(handle as *const agent::ConversationStreamSubscription);
    subscription.cancel();
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_com_longbridge_SdkNative_freeConversationStreamSubscription(
    _env: JNIEnv,
    _class: JClass,
    handle: i64,
) {
    let _ = Box::from_raw(handle as *mut agent::ConversationStreamSubscription);
}

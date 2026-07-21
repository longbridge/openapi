use std::{collections::HashMap, ffi::c_void, os::raw::c_char, sync::Arc};

use longbridge::agent::{
    AgentContext, AnswersByToolCall, GetAgentsOptions, drive_conversation_stream,
};

use crate::{
    agent_context::types::{
        CAgentsResponseOwned, CAnswersByToolCallEntry, CConversationResponseOwned,
        CConversationStreamEvent, CConversationStreamEventOwned, CGetAgentsOptions,
        CWorkspacesResponseOwned,
    },
    async_call::{CAsyncCallback, execute_async},
    callback::CFreeUserDataFunc,
    config::CConfig,
    types::{CCow, ToFFI, cstr_to_rust},
};

/// AI Agent conversation context
pub struct CAgentContext {
    ctx: AgentContext,
}

/// Called once for every event observed while streaming a conversation. See
/// `lb_agent_context_conversation_streamed`/
/// `lb_agent_context_continue_conversation_streamed`.
///
/// Unlike the `lb_xxx_context_set_on_xxx` push callbacks, this callback is
/// scoped to a single streamed call — it's supplied directly as an argument
/// and is never stored on the context.
pub type COnConversationEventCallback =
    extern "C" fn(*const CAgentContext, *const CConversationStreamEvent, *mut c_void);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_agent_context_new(config: *const CConfig) -> *const CAgentContext {
    Arc::into_raw(Arc::new(CAgentContext {
        ctx: AgentContext::new(Arc::new((*config).0.clone())),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_agent_context_retain(ctx: *const CAgentContext) {
    Arc::increment_strong_count(ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_agent_context_release(ctx: *const CAgentContext) {
    let _ = Arc::from_raw(ctx);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_agent_context_ref_count(ctx: *const CAgentContext) -> usize {
    Arc::increment_strong_count(ctx);
    let ctx = Arc::from_raw(ctx);
    Arc::strong_count(&ctx)
}

/// Flatten the C representation of `answers_by_tool_call` (an array of
/// `(tool_call_id, [(question, answer)])` entries) back into the nested-map
/// shape (`HashMap<String, HashMap<String, String>>`) the Rust core expects.
/// See the doc comment on `CAnswersByToolCallEntry` for why the C side is
/// shaped this way.
unsafe fn answers_from_ffi(
    answers: *const CAnswersByToolCallEntry,
    num_answers: usize,
) -> AnswersByToolCall {
    let mut map: AnswersByToolCall = HashMap::new();
    for entry in std::slice::from_raw_parts(answers, num_answers) {
        let tool_call_id = cstr_to_rust(entry.tool_call_id);
        let mut questions = HashMap::new();
        for qa in std::slice::from_raw_parts(entry.answers, entry.num_answers) {
            questions.insert(cstr_to_rust(qa.question), cstr_to_rust(qa.answer));
        }
        map.insert(tool_call_id, questions);
    }
    map
}

/// List the Workspaces the current account belongs to. Returns
/// `CWorkspacesResponse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_agent_context_workspaces(
    ctx: *const CAgentContext,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CWorkspacesResponseOwned> = CCow::new(ctx_inner.workspaces().await?);
        Ok(resp)
    });
}

/// List the Agents in the specified Workspace. Returns `CAgentsResponse`.
///
/// @param[in] opts Options for get agents request (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_agent_context_agents(
    ctx: *const CAgentContext,
    workspace_id: *const c_char,
    opts: *const CGetAgentsOptions,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let workspace_id = cstr_to_rust(workspace_id);
    let mut opts2 = GetAgentsOptions::new();
    if !opts.is_null() {
        if !(*opts).page.is_null() {
            opts2 = opts2.page(*(*opts).page);
        }
        if !(*opts).limit.is_null() {
            opts2 = opts2.limit(*(*opts).limit);
        }
        if !(*opts).name.is_null() {
            opts2 = opts2.name(cstr_to_rust((*opts).name));
        }
    }
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CAgentsResponseOwned> =
            CCow::new(ctx_inner.agents(workspace_id, opts2).await?);
        Ok(resp)
    });
}

/// Start a conversation with the specified Agent, blocking until the run
/// succeeds, is interrupted, or fails. Returns `CConversationResponse`.
///
/// @param[in] chat_uid Existing conversation identifier to continue within
///                      (can be null to start a brand-new conversation)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_agent_context_conversation(
    ctx: *const CAgentContext,
    agent_id: *const c_char,
    query: *const c_char,
    chat_uid: *const c_char,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let agent_id = cstr_to_rust(agent_id);
    let query = cstr_to_rust(query);
    let chat_uid = (!chat_uid.is_null()).then(|| cstr_to_rust(chat_uid));
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CConversationResponseOwned> =
            CCow::new(ctx_inner.conversation(agent_id, query, chat_uid).await?);
        Ok(resp)
    });
}

/// Resume an interrupted conversation, blocking until the run succeeds, is
/// interrupted again, or fails. Returns `CConversationResponse`.
///
/// @param[in] answers      Answers keyed by `tool_call_id`, see
///                         `CAnswersByToolCallEntry` (can be null if
///                         `num_answers` is 0)
/// @param[in] num_answers  Number of entries in `answers`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_agent_context_continue_conversation(
    ctx: *const CAgentContext,
    agent_id: *const c_char,
    chat_uid: *const c_char,
    message_id: *const c_char,
    answers: *const CAnswersByToolCallEntry,
    num_answers: usize,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let agent_id = cstr_to_rust(agent_id);
    let chat_uid = cstr_to_rust(chat_uid);
    let message_id = cstr_to_rust(message_id);
    let answers = answers_from_ffi(answers, num_answers);
    execute_async(callback, ctx, userdata, async move {
        let resp: CCow<CConversationResponseOwned> = CCow::new(
            ctx_inner
                .continue_conversation(agent_id, chat_uid, message_id, answers)
                .await?,
        );
        Ok(resp)
    });
}

/// Start a conversation with the specified Agent, calling `event_callback`
/// for every run-progress event observed over SSE. A `WorkflowFinished`
/// event carries the run's outcome, but isn't necessarily the last one seen —
/// the server may still emit a few more housekeeping events (e.g. a
/// `ChatTitleUpdated`) before actually closing the connection. Once the
/// stream truly ends, `callback` is invoked with the final
/// `CConversationResponse` — same as `lb_agent_context_conversation`, just
/// arrived at via the streamed path.
///
/// @param[in] chat_uid            Existing conversation identifier to
///                                continue within (can be null to start a
///                                brand-new conversation)
/// @param[in] event_callback      Called once per stream event, on an
///                                internal worker thread
/// @param[in] event_userdata      Opaque pointer forwarded to
///                                `event_callback`
/// @param[in] event_free_userdata Called exactly once, after the stream ends
///                                (successfully or not), to free
///                                `event_userdata` (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_agent_context_conversation_streamed(
    ctx: *const CAgentContext,
    agent_id: *const c_char,
    query: *const c_char,
    chat_uid: *const c_char,
    event_callback: COnConversationEventCallback,
    event_userdata: *mut c_void,
    event_free_userdata: CFreeUserDataFunc,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let agent_id = cstr_to_rust(agent_id);
    let query = cstr_to_rust(query);
    let chat_uid = (!chat_uid.is_null()).then(|| cstr_to_rust(chat_uid));
    // Raw pointers aren't `Send`, so thread them through the future as plain
    // addresses (mirroring how `execute_async` itself carries `ctx`/
    // `userdata` across the `spawn` boundary) and only reconstitute them
    // inside the synchronous `on_event` closure below.
    let ctx_addr = ctx as usize;
    let event_userdata_addr = event_userdata as usize;
    execute_async(callback, ctx, userdata, async move {
        let stream = Box::pin(
            ctx_inner
                .conversation_streamed(agent_id, query, chat_uid)
                .await?,
        );
        let result = drive_conversation_stream(stream, move |event| {
            let event_owned: CConversationStreamEventOwned = event.into();
            event_callback(
                ctx_addr as *const CAgentContext,
                &event_owned.to_ffi_type(),
                event_userdata_addr as *mut c_void,
            );
        })
        .await;
        if let Some(free_userdata) = event_free_userdata {
            free_userdata(event_userdata_addr as *mut c_void);
        }
        let resp: CCow<CConversationResponseOwned> = CCow::new(result?);
        Ok(resp)
    });
}

/// Resume an interrupted conversation, calling `event_callback` for every
/// run-progress event observed over SSE, then `callback` with the final
/// `CConversationResponse` once the stream ends — same shape as
/// `lb_agent_context_conversation_streamed`.
///
/// @param[in] answers             Answers keyed by `tool_call_id`, see
///                                `CAnswersByToolCallEntry` (can be null if
///                                `num_answers` is 0)
/// @param[in] num_answers         Number of entries in `answers`
/// @param[in] event_callback      Called once per stream event, on an
///                                internal worker thread
/// @param[in] event_userdata      Opaque pointer forwarded to
///                                `event_callback`
/// @param[in] event_free_userdata Called exactly once, after the stream ends
///                                (successfully or not), to free
///                                `event_userdata` (can be null)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn lb_agent_context_continue_conversation_streamed(
    ctx: *const CAgentContext,
    agent_id: *const c_char,
    chat_uid: *const c_char,
    message_id: *const c_char,
    answers: *const CAnswersByToolCallEntry,
    num_answers: usize,
    event_callback: COnConversationEventCallback,
    event_userdata: *mut c_void,
    event_free_userdata: CFreeUserDataFunc,
    callback: CAsyncCallback,
    userdata: *mut c_void,
) {
    let ctx_inner = (*ctx).ctx.clone();
    let agent_id = cstr_to_rust(agent_id);
    let chat_uid = cstr_to_rust(chat_uid);
    let message_id = cstr_to_rust(message_id);
    let answers = answers_from_ffi(answers, num_answers);
    let ctx_addr = ctx as usize;
    let event_userdata_addr = event_userdata as usize;
    execute_async(callback, ctx, userdata, async move {
        let stream = Box::pin(
            ctx_inner
                .continue_conversation_streamed(agent_id, chat_uid, message_id, answers)
                .await?,
        );
        let result = drive_conversation_stream(stream, move |event| {
            let event_owned: CConversationStreamEventOwned = event.into();
            event_callback(
                ctx_addr as *const CAgentContext,
                &event_owned.to_ffi_type(),
                event_userdata_addr as *mut c_void,
            );
        })
        .await;
        if let Some(free_userdata) = event_free_userdata {
            free_userdata(event_userdata_addr as *mut c_void);
        }
        let resp: CCow<CConversationResponseOwned> = CCow::new(result?);
        Ok(resp)
    });
}

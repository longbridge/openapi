#include "agent_context.hpp"
#include "convert.hpp"

namespace longbridge {
namespace agent {

using longbridge::convert::convert;

namespace {

/// Flattened storage for an `AnswersByToolCall`, matching the C layer's
/// array-of-entries shape (`lb_answers_by_tool_call_entry_t` /
/// `lb_answer_question_t`). The `entries`/`per_entry_answers` vectors are
/// reserved up front so pointers into already-pushed elements stay valid
/// while more are appended; all `const char*`s borrow from the original
/// `AnswersByToolCall`, so this must not outlive it.
struct AnswersFFI
{
  std::vector<lb_answers_by_tool_call_entry_t> entries;
  std::vector<std::vector<lb_answer_question_t>> per_entry_answers;
};

AnswersFFI
build_answers_ffi(const AnswersByToolCall& answers)
{
  AnswersFFI ffi;
  ffi.entries.reserve(answers.size());
  ffi.per_entry_answers.reserve(answers.size());
  for (const auto& [tool_call_id, qa] : answers) {
    std::vector<lb_answer_question_t> qs;
    qs.reserve(qa.size());
    for (const auto& [question, answer] : qa) {
      qs.push_back(lb_answer_question_t{ question.c_str(), answer.c_str() });
    }
    ffi.per_entry_answers.push_back(std::move(qs));
    const auto& stored = ffi.per_entry_answers.back();
    ffi.entries.push_back(lb_answers_by_tool_call_entry_t{
      tool_call_id.c_str(), stored.data(), stored.size() });
  }
  return ffi;
}

} // namespace

AgentContext::AgentContext()
  : ctx_(nullptr)
{
}

AgentContext::AgentContext(const lb_agent_context_t* ctx)
{
  ctx_ = ctx;
  if (ctx_) {
    lb_agent_context_retain(ctx_);
  }
}

AgentContext::AgentContext(const AgentContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_agent_context_retain(ctx_);
  }
}

AgentContext::AgentContext(AgentContext&& ctx)
{
  ctx_ = ctx.ctx_;
  ctx.ctx_ = nullptr;
}

AgentContext::~AgentContext()
{
  if (ctx_) {
    lb_agent_context_release(ctx_);
  }
}

AgentContext&
AgentContext::operator=(const AgentContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_agent_context_retain(ctx_);
  }
  return *this;
}

AgentContext
AgentContext::create(const Config& config)
{
  auto* ctx_ptr = lb_agent_context_new(config);
  AgentContext ctx(ctx_ptr);
  if (ctx_ptr) {
    lb_agent_context_release(ctx_ptr);
  }
  return ctx;
}

void
AgentContext::workspaces(
  AsyncCallback<AgentContext, WorkspacesResponse> callback) const
{
  lb_agent_context_workspaces(
    ctx_,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<AgentContext, WorkspacesResponse>(
          res->userdata);
      AgentContext ctx((const lb_agent_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        WorkspacesResponse resp =
          convert((const lb_workspaces_response_t*)res->data);
        (*callback_ptr)(AsyncResult<AgentContext, WorkspacesResponse>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<AgentContext, WorkspacesResponse>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<AgentContext, WorkspacesResponse>(callback));
}

void
AgentContext::agents(
  const std::string& workspace_id,
  const std::optional<GetAgentsOptions>& opts,
  AsyncCallback<AgentContext, AgentsResponse> callback) const
{
  lb_get_agents_options_t opts2 = { nullptr, nullptr, nullptr };
  if (opts) {
    opts2.page = opts->page ? &opts->page.value() : nullptr;
    opts2.limit = opts->limit ? &opts->limit.value() : nullptr;
    opts2.name = opts->name ? opts->name->c_str() : nullptr;
  }

  lb_agent_context_agents(
    ctx_,
    workspace_id.c_str(),
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<AgentContext, AgentsResponse>(
          res->userdata);
      AgentContext ctx((const lb_agent_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        AgentsResponse resp = convert((const lb_agents_response_t*)res->data);
        (*callback_ptr)(AsyncResult<AgentContext, AgentsResponse>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<AgentContext, AgentsResponse>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<AgentContext, AgentsResponse>(callback));
}

void
AgentContext::conversation(
  const std::string& agent_id,
  const std::string& query,
  const std::optional<std::string>& chat_uid,
  AsyncCallback<AgentContext, ConversationResponse> callback) const
{
  lb_agent_context_conversation(
    ctx_,
    agent_id.c_str(),
    query.c_str(),
    chat_uid ? chat_uid->c_str() : nullptr,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<AgentContext, ConversationResponse>(
          res->userdata);
      AgentContext ctx((const lb_agent_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        ConversationResponse resp =
          convert((const lb_conversation_response_t*)res->data);
        (*callback_ptr)(AsyncResult<AgentContext, ConversationResponse>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<AgentContext, ConversationResponse>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<AgentContext, ConversationResponse>(callback));
}

void
AgentContext::continue_conversation(
  const std::string& agent_id,
  const std::string& chat_uid,
  const std::string& message_id,
  const AnswersByToolCall& answers,
  AsyncCallback<AgentContext, ConversationResponse> callback) const
{
  AnswersFFI ffi = build_answers_ffi(answers);

  lb_agent_context_continue_conversation(
    ctx_,
    agent_id.c_str(),
    chat_uid.c_str(),
    message_id.c_str(),
    ffi.entries.empty() ? nullptr : ffi.entries.data(),
    ffi.entries.size(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<AgentContext, ConversationResponse>(
          res->userdata);
      AgentContext ctx((const lb_agent_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        ConversationResponse resp =
          convert((const lb_conversation_response_t*)res->data);
        (*callback_ptr)(AsyncResult<AgentContext, ConversationResponse>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<AgentContext, ConversationResponse>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<AgentContext, ConversationResponse>(callback));
}

void
AgentContext::conversation_streamed(
  const std::string& agent_id,
  const std::string& query,
  const std::optional<std::string>& chat_uid,
  PushCallback<AgentContext, ConversationStreamEvent> on_event,
  AsyncCallback<AgentContext, ConversationResponse> callback) const
{
  lb_agent_context_conversation_streamed(
    ctx_,
    agent_id.c_str(),
    query.c_str(),
    chat_uid ? chat_uid->c_str() : nullptr,
    [](auto ctx, auto event, auto userdata) {
      auto cb =
        callback::get_push_callback<AgentContext, ConversationStreamEvent>(
          userdata);
      ConversationStreamEvent event2 = convert(event);
      (*cb)(PushEvent<AgentContext, ConversationStreamEvent>(
        AgentContext(ctx), &event2));
    },
    new PushCallback<AgentContext, ConversationStreamEvent>(on_event),
    [](auto p) {
      delete (PushCallback<AgentContext, ConversationStreamEvent>*)p;
    },
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<AgentContext, ConversationResponse>(
          res->userdata);
      AgentContext ctx((const lb_agent_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        ConversationResponse resp =
          convert((const lb_conversation_response_t*)res->data);
        (*callback_ptr)(AsyncResult<AgentContext, ConversationResponse>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<AgentContext, ConversationResponse>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<AgentContext, ConversationResponse>(callback));
}

void
AgentContext::continue_conversation_streamed(
  const std::string& agent_id,
  const std::string& chat_uid,
  const std::string& message_id,
  const AnswersByToolCall& answers,
  PushCallback<AgentContext, ConversationStreamEvent> on_event,
  AsyncCallback<AgentContext, ConversationResponse> callback) const
{
  AnswersFFI ffi = build_answers_ffi(answers);

  lb_agent_context_continue_conversation_streamed(
    ctx_,
    agent_id.c_str(),
    chat_uid.c_str(),
    message_id.c_str(),
    ffi.entries.empty() ? nullptr : ffi.entries.data(),
    ffi.entries.size(),
    [](auto ctx, auto event, auto userdata) {
      auto cb =
        callback::get_push_callback<AgentContext, ConversationStreamEvent>(
          userdata);
      ConversationStreamEvent event2 = convert(event);
      (*cb)(PushEvent<AgentContext, ConversationStreamEvent>(
        AgentContext(ctx), &event2));
    },
    new PushCallback<AgentContext, ConversationStreamEvent>(on_event),
    [](auto p) {
      delete (PushCallback<AgentContext, ConversationStreamEvent>*)p;
    },
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<AgentContext, ConversationResponse>(
          res->userdata);
      AgentContext ctx((const lb_agent_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        ConversationResponse resp =
          convert((const lb_conversation_response_t*)res->data);
        (*callback_ptr)(AsyncResult<AgentContext, ConversationResponse>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<AgentContext, ConversationResponse>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<AgentContext, ConversationResponse>(callback));
}

} // namespace agent
} // namespace longbridge

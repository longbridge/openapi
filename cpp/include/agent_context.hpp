#pragma once

#include <map>

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "push.hpp"
#include "types.hpp"

typedef struct lb_agent_context_t lb_agent_context_t;

namespace longbridge {
namespace agent {

/// Answers keyed by `tool_call_id`, then by question text — mirrors the
/// Rust core's `HashMap<String, HashMap<String, String>>` (see the C
/// layer's `lb_answers_by_tool_call_entry_t`/`lb_answer_question_t`, which
/// flatten this nested map into arrays since C has no native map type).
using AnswersByToolCall = std::map<std::string, std::map<std::string, std::string>>;

/// AI Agent conversation context.
class AgentContext
{
private:
  const lb_agent_context_t* ctx_;

public:
  AgentContext();
  AgentContext(const lb_agent_context_t* ctx);
  AgentContext(const AgentContext& ctx);
  AgentContext(AgentContext&& ctx);
  ~AgentContext();

  AgentContext& operator=(const AgentContext& ctx);

  /// Create an AgentContext from a Config.
  static AgentContext create(const Config& config);

  /// List the Workspaces the current account belongs to.
  void workspaces(AsyncCallback<AgentContext, WorkspacesResponse> callback) const;

  /// List the Agents in the specified Workspace.
  void agents(const std::string& workspace_id,
             const std::optional<GetAgentsOptions>& opts,
             AsyncCallback<AgentContext, AgentsResponse> callback) const;

  /// Start a conversation with the specified Agent, blocking until the run
  /// succeeds, is interrupted, or fails.
  ///
  /// @param chat_uid Existing conversation identifier to continue within
  ///                 (`std::nullopt` to start a brand-new conversation)
  void conversation(const std::string& agent_id,
                    const std::string& query,
                    const std::optional<std::string>& chat_uid,
                    AsyncCallback<AgentContext, ConversationResponse> callback) const;

  /// Resume an interrupted conversation, blocking until the run succeeds,
  /// is interrupted again, or fails.
  ///
  /// @param answers Answers keyed by `tool_call_id`, see AnswersByToolCall
  void continue_conversation(const std::string& agent_id,
                             const std::string& chat_uid,
                             const std::string& message_id,
                             const AnswersByToolCall& answers,
                             AsyncCallback<AgentContext, ConversationResponse> callback) const;

  /// Start a conversation with the specified Agent, calling `on_event` for
  /// every run-progress event observed over SSE. A `WorkflowFinished` event
  /// carries the run's outcome, but isn't necessarily the last one seen —
  /// the server may still emit a few more housekeeping events (e.g.
  /// `ChatTitleUpdated`) before actually closing the connection. Once the
  /// stream truly ends, `callback` is invoked with the final
  /// ConversationResponse — same as `conversation`, just arrived at via the
  /// streamed path.
  ///
  /// @param chat_uid Existing conversation identifier to continue within
  ///                 (`std::nullopt` to start a brand-new conversation)
  void conversation_streamed(
    const std::string& agent_id,
    const std::string& query,
    const std::optional<std::string>& chat_uid,
    PushCallback<AgentContext, ConversationStreamEvent> on_event,
    AsyncCallback<AgentContext, ConversationResponse> callback) const;

  /// Resume an interrupted conversation, calling `on_event` for every
  /// run-progress event observed over SSE, then `callback` with the final
  /// ConversationResponse once the stream ends — same shape as
  /// `conversation_streamed`.
  ///
  /// @param answers Answers keyed by `tool_call_id`, see AnswersByToolCall
  void continue_conversation_streamed(
    const std::string& agent_id,
    const std::string& chat_uid,
    const std::string& message_id,
    const AnswersByToolCall& answers,
    PushCallback<AgentContext, ConversationStreamEvent> on_event,
    AsyncCallback<AgentContext, ConversationResponse> callback) const;
};

} // namespace agent
} // namespace longbridge

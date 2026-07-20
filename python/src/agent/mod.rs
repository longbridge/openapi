mod context;
mod context_async;
mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    parent.add_class::<types::Workspace>()?;
    parent.add_class::<types::WorkspacesResponse>()?;
    parent.add_class::<types::Agent>()?;
    parent.add_class::<types::AgentsResponse>()?;
    parent.add_class::<types::ConversationStatus>()?;
    parent.add_class::<types::Reference>()?;
    parent.add_class::<types::QuestionOption>()?;
    parent.add_class::<types::Question>()?;
    parent.add_class::<types::Interrupt>()?;
    parent.add_class::<types::AgentError>()?;
    parent.add_class::<types::ConversationResponse>()?;
    parent.add_class::<types::ChatStartedPayload>()?;
    parent.add_class::<types::MessagePayload>()?;
    parent.add_class::<types::ConversationStreamEvent>()?;

    parent.add_class::<context::AgentContext>()?;
    parent.add_class::<context::ConversationStreamIter>()?;
    parent.add_class::<context_async::AsyncAgentContext>()?;
    parent.add_class::<context_async::AsyncConversationStreamIter>()?;
    Ok(())
}

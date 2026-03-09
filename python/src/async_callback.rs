//! Invoke a Python callback and, if it returns a coroutine and an event loop
//! is provided, schedule the coroutine on that loop (for use from non-asyncio
//! threads).

use pyo3::prelude::*;

/// If `result` is a coroutine and `event_loop` is `Some`, schedules it via
/// `loop.call_soon_thread_safe(loop.create_task, coro)`. Otherwise a no-op.
pub(crate) fn schedule_coro_if_needed(
    result: &Bound<PyAny>,
    event_loop: Option<&Bound<PyAny>>,
    py: Python<'_>,
) -> PyResult<()> {
    let Some(loop_ref) = event_loop else {
        return Ok(());
    };
    if result.is_none() {
        return Ok(());
    }
    let asyncio = py.import("asyncio")?;
    let is_coro = asyncio
        .getattr("iscoroutine")?
        .call1((result,))?
        .extract::<bool>()?;
    if is_coro {
        let create_task = loop_ref.getattr("create_task")?;
        loop_ref.call_method("call_soon_thread_safe", (create_task, result), None)?;
    }
    Ok(())
}

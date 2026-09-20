use longbridge::trade::{PushEvent, PushGridOrderChanged, PushOrderChanged};
use pyo3::prelude::*;

use crate::{async_callback, trade::context::Callbacks};

pub(crate) fn handle_push_event(
    callbacks: &Callbacks,
    event: PushEvent,
    event_loop: Option<&Bound<PyAny>>,
) {
    match event {
        PushEvent::OrderChanged(order_changed) => {
            handle_order_changed(callbacks, order_changed, event_loop)
        }
        PushEvent::GridOrderChanged(grid_order_changed) => {
            handle_grid_order_changed(callbacks, grid_order_changed, event_loop)
        }
    }
}

fn handle_order_changed(
    callbacks: &Callbacks,
    order_changed: PushOrderChanged,
    event_loop: Option<&Bound<PyAny>>,
) {
    if let Some(callback) = &callbacks.order_changed {
        let _ = Python::attach(|py| {
            let args = (crate::trade::types::PushOrderChanged::try_from(
                order_changed,
            )?,);
            let result = callback.bind(py).call(args, None)?;
            async_callback::schedule_coro_if_needed(&result, event_loop, py)
        });
    }
}

fn handle_grid_order_changed(
    callbacks: &Callbacks,
    grid_order_changed: PushGridOrderChanged,
    event_loop: Option<&Bound<PyAny>>,
) {
    if let Some(callback) = &callbacks.grid_order_changed {
        let _ = Python::attach(|py| {
            let args = (crate::trade::types::PushGridOrderChanged::try_from(
                grid_order_changed,
            )?,);
            let result = callback.bind(py).call(args, None)?;
            async_callback::schedule_coro_if_needed(&result, event_loop, py)
        });
    }
}

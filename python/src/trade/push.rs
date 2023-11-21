use longport::trade::{PushEvent, PushOrderChanged};
use pyo3::Python;

use crate::trade::context::Callbacks;

pub(crate) fn handle_push_event(callbacks: &Callbacks, event: PushEvent) {
    match event {
        PushEvent::OrderChanged(order_changed) => handle_order_changed(callbacks, order_changed),
    }
}

fn handle_order_changed(callbacks: &Callbacks, order_changed: PushOrderChanged) {
    if let Some(callback) = &callbacks.order_changed {
        let _ = Python::with_gil(|py| {
            callback.call1(
                py,
                (crate::trade::types::PushOrderChanged::try_from(
                    order_changed,
                )?,),
            )
        });
    }
}

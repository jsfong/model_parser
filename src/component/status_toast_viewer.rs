use leptos::prelude::*;
use leptos_use::{use_timeout_fn, UseTimeoutFnReturn};

use crate::app::StatusMsg;

const STATUS_BAR_TIMEOUT: f64 = 5000.0;

#[component]
pub fn StatusToastViewer(
    status: ReadSignal<StatusMsg>,
    set_status: WriteSignal<StatusMsg>,
) -> impl IntoView {

    // Use timeout fn to reset status
    let UseTimeoutFnReturn {
        start: clear_status, is_pending, ..
    } = use_timeout_fn(
        move |_: ()| {
            set_status.set(StatusMsg::Empty);
        },
        STATUS_BAR_TIMEOUT,
    );
 

    view! {
        <div
            id="statusbar"
            class=move || {
                let style = match status.get() {
                    StatusMsg::Empty => "hide",
                    _ => "show",
                };
                if style == "show" {
                    clear_status(());
                }
                style
            }
        >
            {move || status.get().get_msg().to_string()}

        </div>
    }
}

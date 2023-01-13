mod counter;

use leptos::*;

use counter::*;
pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <div class={"container"}>
                <header class={"d-flex flex-wrap justify-content-center py-3 mb-4 border-bottom"}>
                <span class={"d-flex align-items-center mb-3 mb-md-0 me-md-auto text-dark text-decoration-none fs-2"}>{"Best Before"}</span>
                </header>
            </div>
            <div class={"container"}><SimpleCounter
                initial_value=0
                step=1
            /></div>
        }
    })
}

#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    hot_reload_init!();
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let mut counter = use_state(cx, || 0);

    cx.render(rsx! {
        label {
            "{counter}"
        }
        button {
            onclick: move |_| counter+= 1,
            "Count"
        }
    })
}

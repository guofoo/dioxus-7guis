#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    hot_reload_init!();
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let start_date: &UseState<String> = use_state(cx, String::new);
    let end_date: &UseState<String> = use_state(cx, String::new);

    let hide_end = use_state(cx, || true);

    cx.render(rsx! {
        form {
            select {
                onchange: move |evt| {
                //   println!("select evt: {:?}", evt.data.value);
                  if evt.data.value != "1" {
                    println!("Return Trip");
                    hide_end.set(false);
                  } else {
                    println!("One-Way Trip");
                    hide_end.set(true);
                  }
                },
                id: "",
                name: "trip_type",
                option {
                    value: "1", "One-Way Flight"
                }
                option {
                    value: "2", "Round Trip"
                }
                option {
                    // selected: true,
                    value: "3", "Multi-City"
                }
            }
            div {
                input {
                    value: "{start_date}",
                    r#type: "date",
                    placeholder: "Start",
                    oninput: move |e| start_date.set(e.value.clone())
                }
            }

            div {
                input {
                    value: "{end_date}",
                    r#type: "date",
                    placeholder: "Return",
                    disabled: "{hide_end}",
                    oninput: move |e| end_date.set(e.value.clone())
                }
            }
            div {
                width: "100%",
                height: "30px",
                button {
                    onclick: move |event| println!("Booked! Event: {event:?}"),
                    "Book",
                }
            }
        }
    })
}

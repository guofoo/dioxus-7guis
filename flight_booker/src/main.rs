#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    hot_reload_init!();
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let start_date = use_state(cx, || 0);
    let end_date = use_state(cx, || 0);

    cx.render(rsx! {
        div {
            // width: "50%",
            // height: "20px",
            // background_color: "red",
            // justify_content: "center",
            // align_items: "center",
            input {
                value: "{start_date}",
                onchange: move |evt| {
                    match evt.value.clone().parse::<i32>() {
                        Ok(number) => {
                            println!("F={}", number);
                        }
                        Err(_) => {
                            println!("Invalid input. Please enter an integer.");
                        }
                    }
                }
            }
        }
        div {
            input {
                value: "{end_date}",
                onchange: move |evt| {
                    match evt.value.clone().parse::<i32>() {
                        Ok(number) => {
                            println!("C={}", number);
                        }
                        Err(_) => {
                            println!("Invalid input. Please enter an integer.");
                        }
                    }
                }
            }
        }
        div {
            width: "50%",
            height: "20px",
            button {
                "Book",

            }
        }
    })
}

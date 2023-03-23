#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    hot_reload_init!();
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let celsius = use_state(cx, || 0);
    let fahrenheit = use_state(cx, || 32);

    cx.render(rsx! {
        input {
            value: "{celsius}",
            onchange: move |evt| {
                match evt.value.clone().parse::<i32>() {
                    Ok(number) => {
                        celsius.set(number);
                        let f_value = number * 9 / 5 + 32;
                        // println!("F={}", f_value);
                        fahrenheit.set(f_value);
                    }
                    Err(_) => {
                        println!("Invalid input. Please enter an integer.");
                    }
                }
            }
        }
        label {
            "Celsius ="
        }
        input {
            value: "{fahrenheit}",
            onchange: move |evt| {
                match evt.value.clone().parse::<i32>() {
                    Ok(number) => {
                        fahrenheit.set(number);
                        let c_value = (number-32) * 5/9;
                        // println!("C={}", c_value);
                        celsius.set(c_value);
                    }
                    Err(_) => {
                        println!("Invalid input. Please enter an integer.");
                    }
                }
            }
        }
        label {
            "Fahrenheit"
        }
    })
}

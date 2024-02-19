use dioxus::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        div { img { src: mg!(file("examples/assets/logo.png")) } }
    }
}

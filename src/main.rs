
mod backend;
use crate::backend::save_dog;

mod components;
use crate::components::label::Label;
use crate::components::input::Input;

use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        let router = dioxus::server::router(App);

        Ok(router)
    })
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        Bolinao {}
        Zambal {}
        Bisaya {}
        English {}
    }
}

#[component]
pub fn Bolinao() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "column", gap: ".5rem",
            Label { html_for: "bolinao", "Bolinao" }

            Input { id: "bolinao", placeholder: "Bolinao translation" }
        }
    }
}

#[component]
pub fn Zambal() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "column", gap: ".5rem",
            Label { html_for: "zambal", "Zambal" }

            Input { id: "zambal", placeholder: "Zambal translation" }
        }
    }
}

#[component]
pub fn Bisaya() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "column", gap: ".5rem",
            Label { html_for: "bisaya", "Bisaya" }

            Input { id: "bisaya", placeholder: "Bisaya translation" }
        }
    }
}

#[component]
pub fn English() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "column", gap: ".5rem",
            Label { html_for: "english", "English" }

            Input { id: "english", placeholder: "Enter English word/phrase" }
        }
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "Cultural Preservation! 🌭" }
        }
    }
}



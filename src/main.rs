
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
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0",
        }
        Content {}

    }
}

#[component]
pub fn TempNavigation() -> Element {
    rsx! {
        div { id: "navigation", class: "grow-1 bg-green-500" }
    }
}

#[component]
pub fn Content() -> Element {
    rsx! {
        div { id: "content", class: "flex flex-col min-h-screen bg-blue-500",
            Title {}
            Languages {}
            TempNavigation {}
        }
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title", class: "grow-3 bg-red-500",
            h1 { "for the culture" }
        }
    }
}

#[component]
pub fn Languages() -> Element {
    rsx! {
        div {
            id: "languages",
            class: "flex flex-col grow-6 justify-end mt-2 ml-6 mr-6 mb-2 bg-red-500 space-y-13",
            Bolinao {}

            Zambal {}

            Bisaya {}

            English {}
        }
    }
}

#[component]
pub fn Bolinao() -> Element {
    rsx! {
        div { class: "bg-white m-3",
            Label { html_for: "bolinao", "Bolinao" }

            Input { id: "bolinao", placeholder: "Bolinao translation" }
        }
    }
}

#[component]
pub fn Zambal() -> Element {
    rsx! {
        div { class: "bg-white m-3",
            Label { html_for: "zambal", "Zambal" }

            Input { id: "zambal", placeholder: "Zambal translation" }
        }
    }
}

#[component]
pub fn Bisaya() -> Element {
    rsx! {
        div { class: "bg-white m-3",
            Label { html_for: "bisaya", "Bisaya" }

            Input { id: "bisaya", placeholder: "Bisaya translation" }
        }
    }
}

#[component]
pub fn English() -> Element {
    rsx! {
        div { class: "bg-white m-3",
            Label { html_for: "english", "English" }

            Input { id: "english", placeholder: "Enter English word/phrase" }
        }
    }
}



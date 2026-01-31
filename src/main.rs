
mod backend;

mod components;
use crate::components::label::Label;
use crate::components::input::Input;
use crate::components::dropdown_menu::*;

use dioxus::prelude::*;
use strum::*;

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
pub fn Navigation() -> Element {
    rsx! {
        div {
            id: "navigation",
            class: "fixed bottom-0 left-0 w-full h-16 z-50 bg-green-500",
            AppMenu {}
        }
    }
}

#[derive(Clone, Copy, strum::Display, strum::EnumIter, PartialEq)]
enum Operation {
    Main,
    Review,
    Dictionary,
}

#[component]
pub fn AppMenu() -> Element {
    let mut selected_operation = use_signal(|| None);

    let operations = Operation::iter().enumerate().map(|(i, o)| {
        rsx! {
            DropdownMenuItem::<Operation> {
                class: "dropdown-menu-item",
                value: o,
                index: i,
                on_select: move |value| {
                    selected_operation.set(Some(value));
                },
                {o.to_string()}
            }
        }
    });

    rsx! {
        DropdownMenu { class: "dropdown-menu", default_open: false,
            DropdownMenuTrigger { class: "dropdown-menu-trigger", "Options" }
            DropdownMenuContent { class: "dropdown-menu-content", {operations} }
        }
        // if let Some(op) = selected_operation() {
        //     "Selected: {op}"
        // }
    }
}

#[component]
pub fn Content() -> Element {
    rsx! {
        div {
            id: "content",
            class: "flex flex-col h-[calc(100vh-4rem)] bg-blue-500",
            Title {}
            Languages {}
            Navigation {}
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



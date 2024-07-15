#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div {
            class: "bg-slate-600",
            ul {
                li {
                    Link {
                        to: "",
                        "直播录制"
                    }
                }
                li {
                    class: "",
                    "设置"
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            Sidebar{

            }
        }
    }
}

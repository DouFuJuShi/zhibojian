#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/setting")]
    Setting {},

    #[route("/signin")]
    Signin {},
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
    let s: &str = "中文";
    println!("{s:?}");

    rsx! {
        div { class: "bg-gray-100 w-1/6 h-dvh float-left border-spacing-y-4 border-r-8",
            ul {
                li {
                    Link { to: Route::Home {}, "直播录制" }
                }

                li {
                    Link { to: Route::Setting {}, "设置"}

                }

                li {
                    Link { to: Route::Setting {},  "登录"}
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        Sidebar{

       }

        div {
            class: "bg-red-50 w-5/6 h-full float-right",

            button {
                class: "float-left border-x-2 border-y-2 bg-white hover:bg-gray-300 h-[30px] w-fit",
                onclick: move |event| println!("{:?}", event),
                "添加直播间"
            }
        }
    }
}

#[component]
fn Setting() -> Element {
    rsx! {
        Sidebar{

        }
    }
}

#[component]
fn Signin() -> Element {
    rsx! {
        Sidebar{

        }
    }
}

#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct DownloadArgs<'a> {
    id: &'a str,
}

pub fn App() -> Element {
    let mut id = use_signal(|| String::new());
    let mut download_msg = use_signal(|| String::new());

    let download_video = move |_: FormEvent| async move {
        if id.read().is_empty() {
            download_msg.set("Please enter a valid video ID.".into());
            return;
        }

        let video_id = id.read();
        let args = serde_wasm_bindgen::to_value(&DownloadArgs { id: &*video_id }).unwrap();

        // Call the backend download function
        let response = invoke("download", args).await.as_string().unwrap_or_else(|| "Error: Could not start download".into());
        download_msg.set(response);
    };

    rsx! {
        link { rel: "stylesheet", href: "styles.css" }
        main {
            class: "container",
            h1 { "YouTube Video Downloader" }

            form {
                class: "row",
                onsubmit: download_video,
                input {
                    id: "video-id-input",
                    placeholder: "Enter video ID...",
                    value: "{id}",
                    oninput: move |event| id.set(event.value())
                }
                button { r#type: "submit", "Download" }
            }
            p { "{download_msg}" }
        }
    }
}

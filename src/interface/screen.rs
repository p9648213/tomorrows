use crate::core::{api::get_desktop_files, entity::FileNode};
use dioxus::prelude::*;

#[component]
pub fn Screen() -> Element {
    let files_resource = use_resource(get_desktop_files);

    rsx! {
        div { class: "w-full h-screen bg-slate-900 text-white p-4",
            match &*files_resource.read_unchecked() {
                Some(Ok(nodes)) => rsx! {
                    div { class: "grid grid-cols-4 gap-4",
                        for node in nodes {
                            FileIcon { node: node.clone() }
                        }
                    }
                },
                Some(Err(e)) => rsx! { div { class: "text-red-500", "System Error: {e}" } },
                None => rsx! { div { "Booting..." } }
            }
        }
    }
}

#[component]
fn FileIcon(node: FileNode) -> Element {
    rsx! {
        div { class: "flex flex-col items-center p-2 hover:bg-white/10 rounded cursor-pointer transition",
            div { class: "text-4xl mb-2",
                if node.kind == "folder" { "📁" } else { "📄" }
            }
            span { class: "text-sm text-center select-none", "{node.name}" }
        }
    }
}

use dioxus::prelude::*;

#[component]
pub fn FolderIconFilled(size: Option<u32>) -> Element {
    let size = size.unwrap_or(64);

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            width: "{size}",
            height: "{size}",
            path {
                fill: "#FFA000",
                d: "M10 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-2-2z"
            }
            path {
                fill: "#FFCA28",
                d: "M2 8v10c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8H2z"
            }
        }
    }
}

#[component]
pub fn FileIconFilled(size: Option<u32>) -> Element {
    let size = size.unwrap_or(64);

    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            width: "{size}",
            height: "{size}",
            path {
                fill: "#90CAF9",
                d: "M14 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V8l-6-6z"
            }
            path {
                fill: "#42A5F5",
                d: "M14 2v6h6l-6-6z"
            }
            rect { fill: "#FFFFFF", x: "8", y: "12", width: "8", height: "2", rx: "1" }
            rect { fill: "#FFFFFF", x: "8", y: "16", width: "8", height: "2", rx: "1" }
        }
    }
}

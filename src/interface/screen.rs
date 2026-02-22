use crate::{
    core::{
        api::{create_new_folder, get_desktop_files, move_file},
        entity::FileNode,
    },
    interface::icon::{FileIconFilled, FolderIconFilled},
};
use dioxus::prelude::*;

const GRID_SIZE: i32 = 96;

#[derive(Clone, Copy, Debug)]
struct ContextMenuState {
    x: i32,
    y: i32,
    visible: bool,
}

#[component]
pub fn Screen() -> Element {
    let mut files = use_signal(Vec::<FileNode>::new);
    let mut dragging_id = use_signal(|| Option::<String>::None);
    let mut drag_offset = use_signal(|| (0, 0));

    let mut context_menu = use_signal(|| ContextMenuState {
        x: 0,
        y: 0,
        visible: false,
    });

    use_resource(move || async move {
        if let Ok(data) = get_desktop_files().await {
            files.set(data);
        }
    });

    let snap_to_grid = |raw_x: i32, raw_y: i32| -> (i32, i32) {
        let col = (raw_x as f32 / GRID_SIZE as f32).round() as i32;
        let row = (raw_y as f32 / GRID_SIZE as f32).round() as i32;
        (col * GRID_SIZE, row * GRID_SIZE)
    };

    let handle_mouse_move = move |evt: MouseEvent| {
        if let Some(id) = dragging_id() {
            let mut current_files = files();
            if let Some(idx) = current_files.iter().position(|f| f.id == id) {
                let (off_x, off_y) = drag_offset();
                current_files[idx].x = evt.client_coordinates().x as i32 - off_x;
                current_files[idx].y = evt.client_coordinates().y as i32 - off_y;
                files.set(current_files);
            }
        }
    };

    let handle_mouse_up = move |_| {
        if let Some(id) = dragging_id() {
            let mut current_files = files();
            if let Some(idx) = current_files.iter().position(|f| f.id == id) {
                let raw_x = current_files[idx].x;
                let raw_y = current_files[idx].y;
                let (final_x, final_y) = snap_to_grid(raw_x, raw_y);

                current_files[idx].x = final_x;
                current_files[idx].y = final_y;

                let id_clone = id.clone();

                spawn(async move {
                    let _ = move_file(id_clone, final_x, final_y).await;
                });
            }
            files.set(current_files);
            dragging_id.set(None);
        }
    };

    let handle_context_menu = move |evt: MouseEvent| {
        evt.prevent_default();
        context_menu.set(ContextMenuState {
            x: evt.client_coordinates().x as i32,
            y: evt.client_coordinates().y as i32,
            visible: true,
        });
    };

    let handle_background_click = move |_| {
        if context_menu().visible {
            context_menu.set(ContextMenuState {
                x: 0,
                y: 0,
                visible: false,
            });
        }
    };

    let mut create_folder_action = move || {
        let state = context_menu();
        context_menu.set(ContextMenuState {
            x: 0,
            y: 0,
            visible: false,
        });

        let (x, y) = snap_to_grid(state.x, state.y);

        spawn(async move {
            if let Ok(new_node) = create_new_folder("New Folder".to_string(), x, y).await {
                files.with_mut(|f| f.push(new_node));
            }
        });
    };

    rsx! {
        div {
            class: "w-full h-screen bg-slate-900 text-white relative overflow-hidden",
            onmousemove: handle_mouse_move,
            onmouseup: handle_mouse_up,
            oncontextmenu: handle_context_menu,
            onclick: handle_background_click,

            for node in files() {
                FileIcon {
                    key: "{node.id}",
                    node: node.clone(),
                    on_drag_start: move |evt: MouseEvent| {
                        let rect_x = node.x;
                        let rect_y = node.y;
                        let mouse_x = evt.client_coordinates().x as i32;
                        let mouse_y = evt.client_coordinates().y as i32;

                        drag_offset.set((mouse_x - rect_x, mouse_y - rect_y));
                        dragging_id.set(Some(node.id.clone()));
                    }
                }
            }

            if context_menu().visible {
                div {
                    class: "absolute bg-slate-800 border border-slate-600 shadow-lg rounded py-1 z-50 w-40 flex flex-col",
                    style: "left: {context_menu().x}px; top: {context_menu().y}px;",

                    button {
                        class: "px-4 py-2 hover:bg-slate-700 text-left text-sm",
                        onclick: move |e| { e.stop_propagation(); create_folder_action(); },
                        "New Folder"
                    }
                }
            }
        }
    }
}

#[component]
fn FileIcon(node: FileNode, on_drag_start: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div {
            class: "absolute flex flex-col gap-1.5 items-center justify-center cursor-pointer select-none p-1 hover:bg-white/10 rounded active:bg-white/20",
            style: "left: {node.x}px; top: {node.y}px; width: {GRID_SIZE}px; height: {GRID_SIZE}px;",
            onmousedown: move |e| on_drag_start.call(e),

            if node.kind == "folder" {
                FolderIconFilled { size: 40 }
            } else {
                FileIconFilled { size: 40 }
            }

            span {
                class: "text-xs text-center leading-tight text-ellipsis line-clamp-2 w-full px-1 drop-shadow-md",
                "{node.name}"
            }
        }
    }
}

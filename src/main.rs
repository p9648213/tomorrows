mod core;
mod interface;

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use crate::core::{database, file_system::FileSystem};
        use dioxus::server::axum::Extension;

        database::init();

        let file_system = FileSystem::new();

        let router = dioxus::server::router(interface::application::Application).layer(Extension(file_system));

        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(interface::application::Application);
}

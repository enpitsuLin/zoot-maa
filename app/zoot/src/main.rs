use assets::Assets;
use global::constants::{APP_ID, APP_NAME};
#[cfg(target_os = "macos")]
use gpui::KeyBinding;
use gpui::{
    actions, px, size, AnyView, App, AppContext, Application, Bounds, Menu, MenuItem, WindowBounds,
    WindowKind, WindowOptions,
};
use gpui::{SharedString, WindowBackgroundAppearance, WindowDecorations};
use gpui_component::{Root, Theme};
use reqwest_client::ReqwestClient;

use std::sync::Arc;

use crate::views::app::ZootApp;

mod assets;
mod logger;
mod views;
mod components;
mod layouts;

actions!(maa, [About, Setting, Quit]);

fn main() {
    let _guard = logger::init_logger();

    let app = Application::new()
        .with_assets(Assets)
        .with_http_client(Arc::new(ReqwestClient::new()));

    app.run(|cx| {
        // Register the `quit` function
        cx.on_action(quit);
        cx.on_action(setting);
        cx.on_action(about);

        // Register the `quit` function with CMD+Q (macOS only)
        #[cfg(target_os = "macos")]
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        // Set menu items
        cx.set_menus(vec![Menu {
            name: SharedString::new_static(APP_NAME),
            items: vec![
                MenuItem::action("Settings", Setting),
                MenuItem::action("About", About),
                MenuItem::separator(),
                MenuItem::action("Quit", Quit),
            ],
        }]);

        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                tracing::info!("Gracefully quitting the application . . .");
                cx.quit();
            }
        })
        .detach();

        let mut window_size = size(px(1280.0), px(700.0));
        if let Some(display) = cx.primary_display() {
            let display_size = display.bounds().size;
            window_size.width = window_size.width.min(display_size.width * 0.85);
            window_size.height = window_size.height.min(display_size.height * 0.85);
        }
        let window_bounds = Bounds::centered(None, window_size, cx);

        let opts = WindowOptions {
            #[cfg(not(target_os = "linux"))]
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::new_static(APP_NAME)),
                traffic_light_position: Some(point(px(10.0), px(10.0))),
                appears_transparent: true,
            }),
            window_bounds: Some(WindowBounds::Windowed(window_bounds)),
            window_min_size: Some(size(px(1000.0), px(588.0))),
            #[cfg(target_os = "linux")]
            window_background: WindowBackgroundAppearance::Transparent,
            #[cfg(target_os = "linux")]
            window_decorations: Some(WindowDecorations::Client),
            kind: WindowKind::Normal,
            app_id: Some(APP_ID.to_owned()),
            ..Default::default()
        };

        // Open a window with default options
        cx.open_window(opts, |window, cx| {
            tracing::info!("Starting application...");

            // Automatically sync theme with system appearance
            window
                .observe_window_appearance(|window, cx| {
                    Theme::sync_system_appearance(Some(window), cx);
                })
                .detach();

            // Root Entity
            cx.new(|cx| {
                cx.activate(true);

                // Initialize settings
                settings::init(cx);
                // Initialize components
                gpui_component::init(cx);
                gpui_tokio::init(cx);

                route::init(cx);

                let app_view = cx.new(|_| ZootApp {});

                Root::new(AnyView::from(app_view), window, cx)
            })
        })
        .expect("Failed to open window. Please restart the application.");
    });
}

fn setting(_: &Setting, _cx: &mut App) {
    tracing::debug!("Opening settings");
}

fn about(_: &About, _cx: &mut App) {
    tracing::debug!("Opening about");
}

fn quit(_: &Quit, cx: &mut App) {
    tracing::info!("Gracefully quitting the application . . .");
    cx.quit();
}

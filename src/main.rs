mod app;
mod audio;
mod model;
mod store;

fn main() -> eframe::Result {
    let viewport = eframe::egui::ViewportBuilder::default()
        .with_title("Français Pas à Pas")
        .with_inner_size([1280.0, 860.0])
        .with_min_inner_size([860.0, 640.0]);
    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "com.frenchapp.course",
        native_options,
        Box::new(|creation| {
            app::FrenchApp::new(&creation.egui_ctx)
                .map(|app| Box::new(app) as Box<dyn eframe::App>)
                .map_err(|error| {
                    Box::new(std::io::Error::other(error))
                        as Box<dyn std::error::Error + Send + Sync>
                })
        }),
    )
}

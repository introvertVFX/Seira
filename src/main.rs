mod app;
mod inet;
mod fs;
mod user;

fn main() -> Result<(), eframe::Error>
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000f32, 600f32]),
        ..Default::default()
    };

     eframe::run_native(
        "Seira",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<app::Seira>::default()
        }),
    )
}

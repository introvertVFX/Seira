pub struct Seira
{
}


impl Default for Seira
{
    fn default() -> Self {
        Self{

        }
    }
}


impl eframe::App for Seira
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

        });
        egui::SidePanel::left("")
            .resizable(false)
            .min_width(25f32)
            .max_width(25f32)
            .show(ctx, |ui|{
            ui.vertical(|ui|{
                let filetree = egui::Button::new("F");
                let utils    = egui::Button::new("U");
                let canvas   = egui::Button::new("C");
                let kanban   = egui::Button::new("K");
                if ui.add_sized([ui.available_width(), ui.available_width()], filetree).clicked() {
                    todo!()
                }
                if ui.add_sized([ui.available_width(), ui.available_width()], utils).clicked() {
                    todo!()
                }
                if ui.add_sized([ui.available_width(), ui.available_width()], canvas).clicked() {
                    todo!()
                }
                if ui.add_sized([ui.available_width(), ui.available_width()], kanban).clicked() {
                   todo!()
                }
            });
        });
    }
    
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        ()
    }
}


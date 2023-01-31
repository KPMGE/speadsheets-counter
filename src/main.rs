use eframe::{
  run_native, 
  epi::App,
  egui::CentralPanel, 
  NativeOptions, 
  egui::{Vec2, Color32}, 
  egui::{Button},
};

struct SpreadSheets {
  name: String
}

impl Default for SpreadSheets {
  fn default() -> Self {
    Self { name: "".to_string() }
  }
}

impl App for SpreadSheets {
  fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
    CentralPanel::default().show(ctx, |ui| {
      const RED:Color32 = Color32::from_rgb(217, 9, 36);
      const BLUE:Color32 = Color32::from_rgb(9, 85, 217);

      ui.horizontal(|ui| {
        ui.label("Your name: ");
        ui.text_edit_singleline(&mut self.name);
      });

      let start_button = ui.add(Button::new("Start counting").fill(BLUE));
      let stop_button = ui.add(Button::new("Stop counting").fill(RED));

      if start_button.clicked() {
        println!("hey now we're counting the time!");
      }

      if stop_button.clicked() {
        println!("hey now we're done counting the time!");
      }
    });
  }

  fn name(&self) -> &str {
    "SpreadSheets"
  }
}

fn main() {
  let app = SpreadSheets::default();
  let window_options = NativeOptions{
    initial_window_size: Some(Vec2 { x: 540f32, y: 600f32 }),
    ..Default::default()
  };
  run_native(Box::new(app), window_options);
}

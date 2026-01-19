use eframe::egui;
mod backup;
use backup::test;

fn main() {
        let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([800.0, 600.0])
            .with_transparent(true),
        ..Default::default()
    };
    let ui = eframe::run_native(
        "test app",
        native_options,
        Box::new(|cc| Ok(Box::new(test_gui::new(cc)))),
    );
}

#[derive(Default)]
struct test_gui {
    data: Vec<test>,
    temp_log: String,
    temp_pass: String,
    logged_in: bool,
    current_user: String,
    current_sub_expires: String,
}

impl test_gui {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
    egui_extras::install_image_loaders(&cc.egui_ctx);
    let data = backup::load_users("data.txt");
    Self { 
        data,
        temp_log: String::new(),
        temp_pass: String::new(),
        logged_in: false,
        current_sub_expires: String::new(),
        current_user: String::new(),
        }
    }
}

impl eframe::App for test_gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::top("title_bar")
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgb(30, 30, 35))
                 .inner_margin(egui::Margin {
                    left: 8,
                    right: 8,
                    top: 4,
                    bottom: 4,
                })
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("test")
                            .size(16.0)
                            .color(egui::Color32::WHITE)
                    );
                    
                    let title_bar_rect = ui.available_rect_before_wrap();
                    let response = ui.interact(
                        title_bar_rect,
                        ui.id().with("title_bar"),
                        egui::Sense::click_and_drag()
                    );
                    
                    if response.dragged() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(egui::RichText::new("+").color(egui::Color32::WHITE)).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        
                        if ui.button(egui::RichText::new("-").color(egui::Color32::WHITE)).clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                        }
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.logged_in {
                ui.label(format!("Welcome back, {}", self.current_user));
                ui.add(
                    egui::Image::new(egui::include_image!("test.png"))
                    .max_size(egui::vec2(64.0, 64.0))
                    .rounding(5.0),
                );
                ui.add(egui::Hyperlink::from_label_and_url("GitHub", "https://yougame.biz/"));
                ui.label(format!("Days to expires: {}", self.current_sub_expires));
                if ui.button("Logout").clicked() {
                    self.logged_in = false;
                    self.current_user.clear();
                }
            } else {
                ui.label("Username:");
                ui.text_edit_singleline(&mut self.temp_log);
                
                ui.label("Password:");
                ui.text_edit_singleline(&mut self.temp_pass);

                if ui.button("Login").clicked() {
                    for users_data in &self.data {
                        if users_data.user == self.temp_log && users_data.password == self.temp_pass {
                            self.logged_in = true;
                            self.current_user = users_data.user.clone();
                            self.current_sub_expires = users_data.sub_expires.clone();
                            self.temp_log.clear();
                            self.temp_pass.clear();
                            break;
                        }
                    }
                }
            }
        });
    }
}
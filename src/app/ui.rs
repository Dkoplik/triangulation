use crate::app::AthenianApp;

// --------------------------------------------------
// Построение UI приложения
// --------------------------------------------------

impl eframe::App for AthenianApp {
    /// Главный цикл UI.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_top_panel(ctx);
        self.show_left_panel(ctx);
        self.show_bottom_panel(ctx);
        self.show_cental_panel(ctx);
    }
}

impl AthenianApp {
    /// Показать верхную панель приложения.
    fn show_top_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });
    }

    /// Показать левую панель приложения.
    fn show_left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    if ui.button("Стереть всё").clicked() {
                        self.clear_canvas();
                    }

                    ui.separator();

                    ui.label("Инструменты:");

                    if ui.button("Выполнить 1 шаг").clicked() {
                        self.do_triangulation_step();
                    }

                    if ui.button("Завершить полностью").clicked() {
                        self.do_full_triangulation();
                    }
                });
            });
    }

    /// Показать нижнюю панель приложения.
    fn show_bottom_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("триангуляция завершена?: {}", self.state.is_triangulation_completed().to_string()));

                ui.separator();

                ui.label(format!("размер холста: {:.1} x {:.1}", self.painter_width, self.painter_height));
            });
        });
    }

    /// Показать центральную (основную) панель приложения.
    fn show_cental_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Resize::default()
                .default_size(egui::Vec2 {x: 900.0, y: 600.0})
                .show(ui, |ui| {
                    let (response, painter) = self.allocate_painter(ui);
                    self.draw_canvas(&painter);
                    self.handle_input(&response);
                });
        });
    }
}

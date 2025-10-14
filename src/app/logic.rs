use crate::app::{
    AthenianApp,
    logic::triangulation::{TriangulationState, init_triangulation, step_triangulation},
};
use egui::{Color32, Painter, Response, Ui};

pub mod polygon;
pub mod triangulation;

// --------------------------------------------------
// Обработка области рисования (холст)
// --------------------------------------------------

impl AthenianApp {
    /// Выделить egui::painter на всю свободную область указанного UI элемента.
    pub fn allocate_painter(&mut self, ui: &mut Ui) -> (Response, Painter) {
        let available_size = ui.available_size();
        self.painter_width = available_size.x;
        self.painter_height = available_size.y;

        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(self.painter_width, self.painter_height),
            egui::Sense::click_and_drag(),
        );

        // цвет холста
        painter.rect_filled(response.rect, 0.0, Color32::WHITE);
        // границы
        // painter.rect_stroke(
        //     response.rect,
        //     0,
        //     Stroke::new(1.0, Color32::GRAY),
        //     StrokeKind::Inside,
        // );

        (response, painter)
    }

    /// Очистить холст.
    pub fn clear_canvas(&mut self) {
        self.state = TriangulationState::default();
    }

    /// Нарисовать холст.
    pub fn draw_canvas(&mut self, painter: &Painter) {
        self.state.draw(painter);
    }
}

// --------------------------------------------------
// Обработка управления
// --------------------------------------------------

impl AthenianApp {
    /// Обработать взаимодействие с холстом.
    pub fn handle_input(&mut self, response: &Response) {
        self.handle_click(response);
    }

    /// Обработать клики по холсту.
    fn handle_click(&mut self, response: &Response) {
        if response.clicked_by(egui::PointerButton::Primary) {
            let pos = response.hover_pos().unwrap();
            self.state.points.push(pos);
        }
    }

    pub fn do_triangulation_step(&mut self) {
        if self.state.is_triangulation_completed() {
            return;
        }

        if !self.state.is_triangulation_initialized() {
            init_triangulation(&mut self.state);
            return;
        }

        step_triangulation(&mut self.state);
    }

    pub fn do_full_triangulation(&mut self) {
        while !self.state.is_triangulation_completed() {
            self.do_triangulation_step();
        }
    }
}

use crate::app::{
    AthenianApp,
    polygon::{Polygon, PolygonStyle, transform2d::Transform2D},
};
use egui::{Color32, Painter, Pos2, Response, Ui};

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

    /// Очистить холст от полигонов.
    pub fn clear_canvas(&mut self) {
        self.polygons.clear();
        self.selected_polygon_index = None;
        self.selected_polygon_anchor = None;
        self.selected_point = None;
    }

    /// Нарисовать текущий якорь.
    fn draw_anchor(&self, painter: &Painter) {
        if let Some(anchor) = self.selected_polygon_anchor {
            painter.circle_filled(anchor, 5.0, Color32::RED);
        }
    }

    /// Нарисовать выбранную точку.
    fn draw_point(&self, painter: &Painter) {
        if let Some(anchor) = self.selected_point {
            painter.circle_filled(anchor, 5.0, Color32::GREEN);
        }
    }

    /// Нарисовать холст.
    pub fn draw_canvas(&mut self, painter: &Painter) {
        for i in 0..self.polygons.len() {
            if self.selected_polygon_index.is_some() && i == self.selected_polygon_index.unwrap() {
                self.polygons[i].draw(&painter, &PolygonStyle::selected(), self.selected_point);
            } else {
                self.polygons[i].draw(&painter, &PolygonStyle::standard(), self.selected_point);
            }
        }
        self.draw_anchor(painter);
        self.draw_point(painter);
    }
}

// --------------------------------------------------
// Обработка управления
// --------------------------------------------------

impl AthenianApp {
    /// Обработать взаимодействие с холстом.
    pub fn handle_input(&mut self, response: &Response) {
        self.handle_click(response);
        self.handle_drag(response);
    }

    /// Обработать клики по холсту.
    fn handle_click(&mut self, response: &Response) {
        if response.clicked_by(egui::PointerButton::Primary) {
            let pos = response.hover_pos().unwrap();
            match &self.instrument {
                Instrument::AddVertex => self.add_vertex_to_selected_polygon(pos),
                Instrument::Select => self.select_polygon(pos),
                Instrument::SetAnchor => self.change_anchor(pos),
                Instrument::SetPoint => self.change_point(pos),
                _ => (),
            }
        }
    }

    /// Обработать перетаскивание по холсту.
    fn handle_drag(&mut self, response: &Response) {
        if response.drag_stopped_by(egui::PointerButton::Primary) {
            self.drag_prev_pos = None;
            return;
        }

        if !response.dragged_by(egui::PointerButton::Primary) {
            return;
        }

        if let Some(drag_start) = self.drag_prev_pos
            && let Some(drag_cur) = response.hover_pos()
        {
            match &self.instrument {
                Instrument::Drag => self.drag_selected_polygon(drag_start, drag_cur),
                Instrument::Rotate => self.rotate_selected_polygon(drag_start, drag_cur),
                Instrument::Scale => self.scale_selected_polygon(drag_start, drag_cur),
                _ => (),
            }
        }

        self.drag_prev_pos = response.hover_pos();
    }
}

// --------------------------------------------------
// Взаимодействие с полигонами
// --------------------------------------------------

impl AthenianApp {
    /// Добавить новую вершину к текущему полигону.
    fn add_vertex_to_selected_polygon(&mut self, pos: Pos2) {
        if let Some(index) = self.selected_polygon_index {
            let polygon = &mut self.polygons[index];
            polygon.add_vertex_pos(pos);
        }
        // Новый полигон
        else {
            let polygon = Polygon::from_pos(pos);
            self.polygons.push(polygon);
            self.selected_polygon_index = Some(self.polygons.len() - 1);
        }
    }

    /// Выбрать полигон в указанной точке.
    fn select_polygon(&mut self, pos: Pos2) {
        // обнулить прошлый якорь
        self.selected_polygon_anchor = None;

        for i in 0..self.polygons.len() {
            if self.polygons[i].contains_pos(pos) {
                self.selected_polygon_index = Some(i);
                return;
            }
        }
        self.selected_polygon_index = None;
    }

    /// Выбрать якорь для операций над полигоном.
    fn change_anchor(&mut self, pos: Pos2) {
        self.selected_polygon_anchor = Some(pos);
    }

    /// Выбрать точку для проверки положения относительно ребёр.
    fn change_point(&mut self, pos: Pos2) {
        self.selected_point = Some(pos);
    }

    /// Переместить выбранный полигон параллельно координатным осям.
    fn drag_selected_polygon(&mut self, start: Pos2, end: Pos2) {
        if let Some(index) = self.selected_polygon_index {
            let delta = end - start;
            let polygon = &mut self.polygons[index];
            polygon.apply_transform(Transform2D::translation(delta.x, delta.y));

            #[cfg(debug_assertions)]
            println!("drag with start {:#?} end {:#?}", start, end);
        }
    }

    /// Повернуть выбранный полигон через вектор смещения.
    fn rotate_selected_polygon(&mut self, start: Pos2, end: Pos2) {
        if let Some(index) = self.selected_polygon_index {
            let polygon = &mut self.polygons[index];

            // Задан якорь для вращения
            if let Some(anchor) = self.selected_polygon_anchor {
                let angle = calculate_rotation_angle(anchor, start, end);
                polygon.apply_transform(Transform2D::rotation_around_pos(angle, anchor));

                #[cfg(debug_assertions)]
                println!("rotate relative to {:#?} with angle {:#?}", anchor, angle);
            }
            // Просто повернуть относительно центра
            else {
                let center = polygon.get_center();
                let angle = calculate_rotation_angle(center, start, end);
                polygon.apply_transform(Transform2D::rotation_around_pos(angle, center));

                #[cfg(debug_assertions)]
                println!(
                    "rotate relative to center {:#?} with angle {:#?}",
                    center, angle
                );
            }
        }
    }

    /// Изменить размер полигона через вектор смещения.
    fn scale_selected_polygon(&mut self, start: Pos2, end: Pos2) {
        if let Some(index) = self.selected_polygon_index {
            let polygon = &mut self.polygons[index];

            // Задан якорь для изменения размера
            if let Some(anchor) = self.selected_polygon_anchor {
                let (sx, sy) = calculate_scale(anchor, start, end);
                polygon.apply_transform(Transform2D::scaling_around_pos(sx, sy, anchor));

                #[cfg(debug_assertions)]
                println!(
                    "scale relative to {:#?} with scale x:{} y:{}",
                    anchor, sx, sy
                );
            }
            // Просто растянуть относительно центра
            else {
                let center = polygon.get_center();
                let (sx, sy) = calculate_scale(center, start, end);
                polygon.apply_transform(Transform2D::scaling_around_pos(sx, sy, center));

                #[cfg(debug_assertions)]
                println!(
                    "scale relative to center {:#?} with scale x:{} y:{}",
                    center, sx, sy
                );
            }
        }
    }
}

#[derive(Default)]
pub enum Instrument {
    #[default]
    AddVertex,
    Select,
    SetAnchor,
    SetPoint,
    Drag,
    Rotate,
    Scale,
}

impl ToString for Instrument {
    fn to_string(&self) -> String {
        match self {
            Self::AddVertex => String::from("добавить вершину"),
            Self::Select => String::from("выбрать полигон"),
            Self::SetAnchor => String::from("изменить якорь полигона"),
            Self::SetPoint => String::from("изменить точку"),
            Self::Drag => String::from("перетащить полигон"),
            Self::Rotate => String::from("повернуть полигон"),
            Self::Scale => String::from("изменить размер полигона"),
        }
    }
}

/// Считает угол поворота в раиданах на основе смещения относительно какого-то центра.
fn calculate_rotation_angle(center: Pos2, start: Pos2, end: Pos2) -> f32 {
    let start_vec = (start.x - center.x, start.y - center.y);
    let end_vec = (end.x - center.x, end.y - center.y);

    let start_angle = start_vec.1.atan2(start_vec.0);
    let end_angle = end_vec.1.atan2(end_vec.0);

    let mut angle = start_angle - end_angle;
    let pi = std::f32::consts::PI;
    while angle > pi {
        angle -= 2.0 * pi;
    }
    while angle < -pi {
        angle += 2.0 * pi;
    }

    angle
}

/// Считает растяжение на основе смещения относительно какого-то центра.
fn calculate_scale(center: Pos2, start: Pos2, end: Pos2) -> (f32, f32) {
    let start_vec = start - center;
    let end_vec = end - center;

    let scale_x = if start_vec.x.abs() < f32::EPSILON {
        1.0
    } else {
        end_vec.x / start_vec.x
    };

    let scale_y = if start_vec.y.abs() < f32::EPSILON {
        1.0
    } else {
        end_vec.y / start_vec.y
    };

    (scale_x, scale_y)
}

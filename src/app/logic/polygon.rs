// --------------------------------------------------
// Реализация полигона
// --------------------------------------------------

/// Представление полигона. Точка и вектор тоже считаются полигонами.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Polygon {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

// --------------------------------------------------
// Конструкторы
// --------------------------------------------------
impl Polygon {
    /// Создание полигона из набора точек.
    pub fn from_poses(mut poses: [usize; 3]) -> Self {
        poses.sort();
        Self {
            a: poses[0],
            b: poses[1],
            c: poses[2],
        }
    }
}

/// Настройка рисования полигона
pub struct PolygonStyle {
    /// Цвет вершины полигона
    pub vertex_color: egui::Color32,
    /// Радиус вершины полигона
    pub vertex_radius: f32,

    /// Цвет пересечения полигона
    pub intersection_color: egui::Color32,
    /// Радиус пересечения полигона
    pub intersection_radius: f32,

    /// Цвет ребра полигона
    pub edge_color: egui::Color32,
    /// Толщина ребра полигона
    pub edge_width: f32,

    /// Цвет стрелки
    pub arrow_color: egui::Color32,
    /// Ширина стрелки
    pub arrow_width: f32,
}

impl PolygonStyle {
    /// Стандартный стиль полигона
    pub fn dead() -> Self {
        PolygonStyle {
            vertex_color: egui::Color32::BLACK,
            vertex_radius: 7.0,
            intersection_color: egui::Color32::LIGHT_GRAY,
            intersection_radius: 3.0,
            edge_color: egui::Color32::BLACK,
            edge_width: 5.0,
            arrow_color: egui::Color32::LIGHT_BLUE,
            arrow_width: 1.0,
        }
    }

    /// Стиль выбранного полигона
    pub fn alive() -> Self {
        PolygonStyle {
            vertex_color: egui::Color32::LIGHT_BLUE,
            vertex_radius: 10.0,
            intersection_color: egui::Color32::DARK_BLUE,
            intersection_radius: 7.0,
            edge_color: egui::Color32::LIGHT_BLUE,
            edge_width: 7.0,
            arrow_color: egui::Color32::DARK_BLUE,
            arrow_width: 1.0,
        }
    }
}

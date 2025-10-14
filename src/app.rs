pub mod logic;
pub mod polygon;
pub mod ui;

// --------------------------------------------------
// Базовое определение приложения
// --------------------------------------------------

/// Приложение-демонстрация аффинных преобразований.
#[derive(Default)]
pub struct AthenianApp {
    /// Текущие полигоны на холсте
    polygons: Vec<polygon::Polygon>,
    /// Индекс выбранного полигона
    selected_polygon_index: Option<usize>,
    /// Якорь для операций над полигоном
    selected_polygon_anchor: Option<egui::Pos2>,
    /// Точка (слева/справа от ребра)
    selected_point: Option<egui::Pos2>,

    /// Текущий инструмент
    instrument: logic::Instrument,

    /// Начальная позиция перетаскивания
    drag_prev_pos: Option<egui::Pos2>,

    // Размеры холста
    painter_width: f32,
    painter_height: f32,
}

impl AthenianApp {
    /// Инициализация приложения.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // белая тема
        cc.egui_ctx.set_theme(egui::Theme::Light);
        Self::default()
    }
}

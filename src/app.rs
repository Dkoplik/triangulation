pub mod logic;
pub mod ui;

use logic::triangulation::TriangulationState;

// --------------------------------------------------
// Базовое определение приложения
// --------------------------------------------------

/// Приложение-демонстрация аффинных преобразований.
#[derive(Default)]
pub struct AthenianApp {
    /// Состояние триангуляции.
    state: TriangulationState,

    // Размеры холста.
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

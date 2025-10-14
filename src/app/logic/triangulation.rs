use egui::Pos2;
use std::{collections::HashSet, hash::Hash};

use crate::app::logic::polygon::{Polygon, PolygonStyle};

/// Текущее состояние триангуляции Делоне.
#[derive(Debug, Default)]
pub struct TriangulationState {
    /// Набор точек (вершин) для построение полигона.
    pub points: Vec<Pos2>,
    /// Полученные полигоны.
    pub triangles: Vec<Polygon>,
    /// "Живые" рёбра.
    pub alive_edges: HashSet<Edge>,
    /// "Мёртвые" рёбра.
    pub dead_edges: HashSet<Edge>,
}

impl TriangulationState {
    fn draw_triangles(&self, painter: &egui::Painter, style: &PolygonStyle) {
        self.triangles.iter().for_each(|triangle| {
            triangle.draw(painter, style);
        });
    }

    fn draw_points(&self, painter: &egui::Painter, style: &PolygonStyle) {
        self.points.iter().for_each(|point_pos| {
            painter.circle_filled(*point_pos, style.vertex_radius, style.vertex_color);
        });
    }

    fn draw_alive_edges(&self, painter: &egui::Painter, style: &PolygonStyle) {
        self.alive_edges.iter().for_each(|edge| {
            painter.line_segment(
                [self.points[edge.0], self.points[edge.1]],
                egui::epaint::Stroke::new(style.edge_width, style.edge_color),
            );
        });
    }

    fn draw_dead_edges(&self, painter: &egui::Painter, style: &PolygonStyle) {
        self.dead_edges.iter().for_each(|edge| {
            painter.line_segment(
                [self.points[edge.0], self.points[edge.1]],
                egui::epaint::Stroke::new(style.edge_width, style.edge_color),
            );
        });
    }

    pub fn draw(&self, painter: &egui::Painter) {
        self.draw_triangles(painter, &PolygonStyle::dead());
        self.draw_points(painter, &PolygonStyle::dead());
        self.draw_dead_edges(painter, &PolygonStyle::dead());
        self.draw_alive_edges(painter, &PolygonStyle::alive());
    }

    pub fn is_triangulation_initialized(&self) -> bool {
        return !self.alive_edges.is_empty() || !self.dead_edges.is_empty();
    }

    pub fn is_triangulation_completed(&self) -> bool {
        return self.alive_edges.is_empty() && !self.dead_edges.is_empty();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge(usize, usize);

impl Edge {
    fn new(a: usize, b: usize) -> Self {
        if a < b { Edge(a, b) } else { Edge(b, a) }
    }
}

/// Инициализировать триангуляцию вместе с выбором первого ребра.
pub fn init_triangulation(state: &mut TriangulationState) {
    if state.points.len() < 3 {
        return;
    }

    state.triangles = Vec::new();
    state.alive_edges = HashSet::new();
    state.dead_edges = HashSet::new();

    state.alive_edges.insert(find_initial_edge(&state.points));
}

/// Выполнить шаг триангуляции.
pub fn step_triangulation(state: &mut TriangulationState) {
    let mut current_edge;
    let mut right_point;
    // поиск живой вершины
    loop {
        // алгоритм завершён
        if state.alive_edges.is_empty() {
            return;
        }

        current_edge = *state.alive_edges.iter().next().unwrap();
        state.alive_edges.remove(&current_edge);

        // ребро уже было рассмотрено
        if state.dead_edges.contains(&current_edge) {
            continue;
        }

        right_point = find_right_conjugate_point(&state.points, current_edge);
        // нет правой сопряжённой точки => ребро принадлежит границе
        if right_point.is_none() {
            continue;
        }
        break;
    }

    let best_point = right_point.unwrap();
    let new_triangle = Polygon::from_poses(vec![
        state.points[current_edge.0],
        state.points[current_edge.1],
        state.points[best_point],
    ]);
    state.triangles.push(new_triangle);

    // рассмотрение новых рёбер
    let edges_to_add = [
        if is_point_left(
            state.points[current_edge.1],
            state.points[current_edge.0],
            state.points[best_point],
        ) {
            Edge::new(current_edge.0, best_point)
        } else {
            Edge::new(best_point, current_edge.0)
        },
        if is_point_left(
            state.points[current_edge.0],
            state.points[current_edge.1],
            state.points[best_point],
        ) {
            Edge::new(current_edge.1, best_point)
        } else {
            Edge::new(best_point, current_edge.1)
        },
    ];
    for edge in edges_to_add {
        if !state.dead_edges.contains(&edge) && !state.alive_edges.contains(&edge) {
            state.alive_edges.insert(edge);
        } else if state.alive_edges.contains(&edge) {
            state.alive_edges.remove(&edge);
            state.dead_edges.insert(edge);
        }
    }
    state.dead_edges.insert(current_edge);
}

/// Нахождение начального ребра для триангуляции Делоне.
fn find_initial_edge(points: &[Pos2]) -> Edge {
    // Нужна самая левая точка
    let mut leftmost_idx = 0;
    for i in 1..points.len() {
        if points[i].x < points[leftmost_idx].x
            || (points[i].x == points[leftmost_idx].x && points[i].y < points[leftmost_idx].y)
        {
            leftmost_idx = i;
        }
    }

    // 2-ая точка ребра (по минимальному углу)
    let mut best_idx = (leftmost_idx + 1) % points.len();
    for i in 0..points.len() {
        if i == leftmost_idx {
            continue;
        }
        let current_angle = angle_with_horizontal(&points[leftmost_idx], &points[i]);
        let best_angle = angle_with_horizontal(&points[leftmost_idx], &points[best_idx]);

        if current_angle < best_angle {
            best_idx = i;
        }
    }

    Edge::new(leftmost_idx, best_idx)
}

/// Угол ребра к горизонтали
fn angle_with_horizontal(p1: &Pos2, p2: &Pos2) -> f32 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    dy.atan2(dx)
}

/// Нахождение правой сопряжённой точки
fn find_right_conjugate_point(points: &[Pos2], edge: Edge) -> Option<usize> {
    let p1 = points[edge.0];
    let p2 = points[edge.1];

    let mut best_point = None;
    let mut best_distance = f32::INFINITY;

    for i in 0..points.len() {
        if i == edge.0 || i == edge.1 {
            continue;
        }

        let p3 = points[i];
        // точка должна быть справа от ребра
        if !is_point_right(p1, p2, p3) {
            continue;
        }

        // расстояние до центра описанной
        if let Some(center) = calculate_center(p1, p2, p3) {
            let mid_edge = Pos2::new((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0);
            let distance =
                ((mid_edge.x - center.x).powi(2) + (mid_edge.y - center.y).powi(2)).sqrt();

            if distance < best_distance {
                best_distance = distance;
                best_point = Some(i);
            }
        }
    }

    best_point
}

/// Нахождение центра окружности, проходящей через точки a, b, c.
fn calculate_center(a: Pos2, b: Pos2, c: Pos2) -> Option<Pos2> {
    // срединный перпендикуляр к ab
    let ab = b - a;
    let n_ab_start = Pos2::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0);
    let n_ab_end = Pos2::new(n_ab_start.x + ab.y, n_ab_start.y - ab.x);

    // срединный перпендикуляр к ac
    let ac = c - a;
    let n_ac_start = Pos2::new((a.x + c.x) / 2.0, (a.y + c.y) / 2.0);
    let n_ac_end = Pos2::new(n_ac_start.x + ac.y, n_ac_start.y - ac.x);

    lines_intersect(n_ab_start, n_ab_end, n_ac_start, n_ac_end)
}

fn is_point_right(point: Pos2, start: Pos2, end: Pos2) -> bool {
    let segment_vector = end - start;
    let point_vector = point - start;

    // векторное произведение
    let cross_product = segment_vector.x * point_vector.y - segment_vector.y * point_vector.x;
    cross_product < 0.0
}

fn is_point_left(point: Pos2, start: Pos2, end: Pos2) -> bool {
    let segment_vector = end - start;
    let point_vector = point - start;

    // векторное произведение
    let cross_product = segment_vector.x * point_vector.y - segment_vector.y * point_vector.x;
    cross_product > 0.0
}

/// Проверка пересечения двух отрезков ab и cd
fn lines_intersect(a: Pos2, b: Pos2, c: Pos2, d: Pos2) -> Option<Pos2> {
    let denominator = (a.x - b.x) * (c.y - d.y) - (a.y - b.y) * (c.x - d.x);
    if denominator.abs() < f32::EPSILON {
        return None;
    }

    let numerator_x = (a.x * b.y - a.y * b.x) * (c.x - d.x) - (a.x - b.x) * (c.x * d.y - c.y * d.x);
    let numerator_y = (a.x * b.y - a.y * b.x) * (c.y - d.y) - (a.y - b.y) * (c.x * d.y - c.y * d.x);

    let x = numerator_x / denominator;
    let y = numerator_y / denominator;

    Some(Pos2::new(x, y))
}

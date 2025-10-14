use egui::Pos2;
use std::{collections::HashSet, hash::Hash};

use crate::app::logic::polygon::Polygon;

/// Текущее состояние триангуляции Делоне.
#[derive(Debug)]
struct TriangulationState {
    /// Набор точек (вершин) для построение полигона.
    points: Vec<Pos2>,
    /// Полученные полигоны.
    triangles: Vec<Polygon>,
    /// "Живые" рёбра.
    alive_edges: HashSet<Edge>,
    /// "Мёртвые" рёбра.
    dead_edges: HashSet<Edge>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge(usize, usize);

impl Edge {
    fn new(a: usize, b: usize) -> Self {
        if a < b { Edge(a, b) } else { Edge(b, a) }
    }

    fn reversed(&self) -> Edge {
        Edge(self.1, self.0)
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
    let mut current_edge: Edge;

    // поиск живой вершины
    loop {
        // алгоритм завершён
        if state.alive_edges.is_empty() {
            return;
        }

        current_edge = *state.alive_edges.iter().next().unwrap();
        state.alive_edges.remove(&current_edge);

        // ребро уже было рассмотрено
        if !state.dead_edges.contains(&current_edge) {
            continue;
        }

        let right_point = find_right_conjugate_point(
            &state.points,
            current_edge,
            &state.alive_edges,
            &state.dead_edges,
        );
        // нет правой сопряжённой точки => ребро принадлежит границе
        if right_point.is_none() {
            continue;
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
            Edge::new(current_edge.0, best_point),
            Edge::new(current_edge.1, best_point),
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
fn find_right_conjugate_point(
    points: &[Pos2],
    edge: Edge,
    alive_edges: &HashSet<Edge>,
    dead_edges: &HashSet<Edge>,
) -> Option<usize> {
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
        if let Some(circumcenter) = calculate_circumcenter(p1, p2, p3) {
            // Вектор от середины ребра к центру окружности
            let mid_point = Pos2::new((p1.x + p2.x) * 0.5, (p1.y + p2.y) * 0.5);
            let to_center = Pos2::new(circumcenter.x - mid_point.x, circumcenter.y - mid_point.y);

            // Перпендикуляр к ребру (направлен в правую сторону)
            let edge_vec = Pos2::new(p2.x - p1.x, p2.y - p1.y);
            let perpendicular = Pos2::new(-edge_vec.y, edge_vec.x);
            let distance = to_center.x * perpendicular.x + to_center.y * perpendicular.y;

            if distance < best_distance {
                best_distance = distance;
                best_point = Some(i);
            }
        }
    }

    best_point
}

/// Нахождение центра окружности, проходящей через точки a, b, c.
fn calculate_circumcenter(a: Pos2, b: Pos2, c: Pos2) -> Option<Pos2> {
    let d = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));

    if d.abs() < 1e-10 {
        return None; // Точки коллинеарны
    }

    let a_sq = a.x * a.x + a.y * a.y;
    let b_sq = b.x * b.x + b.y * b.y;
    let c_sq = c.x * c.x + c.y * c.y;

    let x = (a_sq * (b.y - c.y) + b_sq * (c.y - a.y) + c_sq * (a.y - b.y)) / d;
    let y = (a_sq * (c.x - b.x) + b_sq * (a.x - c.x) + c_sq * (b.x - a.x)) / d;

    Some(Pos2::new(x, y))
}

/// Проверяет, находится ли точка point слева от отрезка [start, end]
fn is_point_left(point: Pos2, start: Pos2, end: Pos2) -> bool {
    let segment_vector = end - start;
    let point_vector = point - start;

    // векторное произведение
    let cross_product = segment_vector.x * point_vector.y - segment_vector.y * point_vector.x;
    cross_product > 0.0
}

fn is_point_right(point: Pos2, start: Pos2, end: Pos2) -> bool {
    !is_point_left(point, start, end)
}

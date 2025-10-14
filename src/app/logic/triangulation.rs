use std::collections::HashSet;
use egui::Pos2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge(usize, usize);

impl Edge {
    fn new(a: usize, b: usize) -> Self {
        if a < b {
            Edge(a, b)
        } else {
            Edge(b, a)
        }
    }
    
    fn reversed(&self) -> Edge {
        Edge(self.1, self.0)
    }
}

pub fn delaunay_triangulation(points: Vec<Pos2>) -> Vec<[usize; 3]> {
    if points.len() < 3 {
        return Vec::new();
    }
    
    let mut triangles = Vec::new();
    let mut alive_edges = HashSet::new();
    let mut dead_edges = HashSet::new();
    
    // Шаг 1: Находим начальное ребро с помощью алгоритма заворачивания подарков
    let initial_edge = find_initial_edge(&points);
    
    // Добавляем начальное ребро в список живых
    alive_edges.insert(initial_edge);
    
    // Шаг 2: Обрабатываем живые ребра
    while let Some(&current_edge) = alive_edges.iter().next().cloned() {
        alive_edges.remove(&current_edge);
        
        if dead_edges.contains(&current_edge) {
            continue;
        }
        
        // Ищем правую сопряженную точку
        if let Some(best_point) = find_right_conjugate_point(&points, current_edge, &alive_edges, &dead_edges) {
            // Создаем новый треугольник
            let new_triangle = Triangle(current_edge.0, current_edge.1, best_point);
            triangles.push(new_triangle);
            
            // Добавляем новые ребра
            let edges_to_add = [
                Edge::new(current_edge.0, best_point),
                Edge::new(current_edge.1, best_point),
            ];
            
            for edge in edges_to_add {
                if !dead_edges.contains(&edge) && !alive_edges.contains(&edge) {
                    alive_edges.insert(edge);
                } else if alive_edges.contains(&edge) {
                    alive_edges.remove(&edge);
                    dead_edges.insert(edge);
                }
            }
            
            dead_edges.insert(current_edge);
        } else {
            // Нет правой сопряженной точки - ребро принадлежит границе
            dead_edges.insert(current_edge);
        }
    }
    
    // Конвертируем в формат результата
    triangles.iter().map(|t| [t.0, t.1, t.2]).collect()
}

fn find_initial_edge(points: &[Pos2]) -> Edge {
    // Находим самую левую точку
    let mut leftmost_idx = 0;
    for i in 1..points.len() {
        if points[i].x < points[leftmost_idx].x || 
           (points[i].x == points[leftmost_idx].x && points[i].y < points[leftmost_idx].y) {
            leftmost_idx = i;
        }
    }
    
    // Находим точку, образующую минимальный угол с горизонталью
    let mut best_idx = (leftmost_idx + 1) % points.len();
    for i in 0..points.len() {
        if i != leftmost_idx {
            let current_angle = angle_with_horizontal(&points[leftmost_idx], &points[i]);
            let best_angle = angle_with_horizontal(&points[leftmost_idx], &points[best_idx]);
            
            if current_angle < best_angle {
                best_idx = i;
            }
        }
    }
    
    Edge::new(leftmost_idx, best_idx)
}

fn angle_with_horizontal(p1: &Pos2, p2: &Pos2) -> f32 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    dy.atan2(dx)
}

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
        
        // Проверяем, что точка находится справа от ребра
        if !is_point_on_right_side(p1, p2, p3) {
            continue;
        }
        
        // Вычисляем расстояние до центра описанной окружности
        if let Some(circumcenter) = calculate_circumcenter(p1, p2, p3) {
            // Вектор от середины ребра к центру окружности
            let mid_point = Pos2::new((p1.x + p2.x) * 0.5, (p1.y + p2.y) * 0.5);
            let to_center = Pos2::new(circumcenter.x - mid_point.x, circumcenter.y - mid_point.y);
            
            // Перпендикуляр к ребру (направлен в правую сторону)
            let edge_vec = Pos2::new(p2.x - p1.x, p2.y - p1.y);
            let perpendicular = Pos2::new(-edge_vec.y, edge_vec.x);
            
            // Расстояние со знаком (отрицательное, если центр слева)
            let distance = to_center.x * perpendicular.x + to_center.y * perpendicular.y;
            
            if distance < best_distance {
                best_distance = distance;
                best_point = Some(i);
            }
        }
    }
    
    best_point
}

fn is_point_on_right_side(p1: Pos2, p2: Pos2, p: Pos2) -> bool {
    let edge_vec = (p2.x - p1.x, p2.y - p1.y);
    let point_vec = (p.x - p1.x, p.y - p1.y);
    
    // Векторное произведение (2D cross product)
    let cross = edge_vec.0 * point_vec.1 - edge_vec.1 * point_vec.0;
    
    cross > 0.0 // Положительное значение означает правую сторону
}

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
use crate::point::*;

pub struct Polygon {
    points: Vec<Point>,
    area: Option<f64>,
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Polygon {
        // TODO: implement validity check
        // TODO: add first point to end of vec
        return Polygon { points, area: None };
    }
    pub fn getcentroid(&self) -> Point {
        return;
    }
    pub fn getarea(&self) -> Option<f64> {
        return self.area;
    }
}

pub fn calcarea(polygon: &Polygon) -> f64 {
    return;
}

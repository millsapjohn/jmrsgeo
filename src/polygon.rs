use crate::point::*;

pub struct Polygon {
    points: Vec<Point>,
    area: Option<f64>,
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Polygon {
        // TODO: implement type casting
        // TODO: implement validity check
        // TODO: add first point to end of vec
        return Polygon { points, area: None };
    }
}

// TODO: setarea method
// TODO: area function

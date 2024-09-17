use crate::point::*;

pub struct Polygon2d {
    points: Vec<Pointxy>,
    area: Option<f64>,
}

impl Polygon2d {
    pub fn new(points: Vec<&impl Point2d>) -> Polygon2d {
        // TODO: implement type casting
        // TODO: implement validity check
        // TODO: add first point to end of vec
        return Polygon2d { points, area: None };
    }
}

pub struct Polygon3d {
    points: Vec<Pointz>,
    area: Option<f64>,
}

impl Polygon3d {
    pub fn new(points: Vec<&impl Point3d>) -> Polygon3d {
        // TODO: implement type casting
        // TODO: implement validity check
        // TODO: add first point to end of vec
        return Polygon3d { points, area: None };
    }
}

// TODO: setarea method
// TODO: area function

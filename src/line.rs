use crate::point::*;

pub struct Segment {
    start: Point,
    end: Point,
    a_coeff: f64,
    b_coeff: Option<f64>,
    c_coeff: f64,
    len2d: f64,
}

impl Segment {
    pub fn new(start: &Point, end: &Point) -> Segment {
        let coeffs: (f64, Option<f64>, f64) = get_linear_eq(start, end);
        let a_coeff: f64 = coeffs.0;
        let b_coeff: Option<f64> = coeffs.1;
        let c_coeff: f64 = coeffs.2;
        return Segment {
            start: *start,
            end: *start,
            a_coeff,
            b_coeff,
            c_coeff,
            len2d: dist2d(start, end),
        };
    }
    pub fn get_start(&self) -> Point {
        return self.start;
    }
    pub fn get_end(&self) -> Point {
        return self.end;
    }
    pub fn geta(&self) -> f64 {
        return self.a_coeff;
    }
    pub fn getb(&self) -> Option<f64> {
        return self.b_coeff;
    }
    pub fn getc(&self) -> f64 {
        return self.c_coeff;
    }
    pub fn getlen2d(&self) -> f64 {
        return self.len2d;
    }
}

pub fn get_linear_eq(point1: &Point, point2: &Point) -> (f64, Option<f64>, f64) {
    match equal2d(point1, point2) {
        true => panic!("points are coincident, cannot create segment"),
        false => {
            let slope = if point1.getx() == point2.getx() {
                None
            } else {
                Some(point2.gety() - point1.gety() / (point2.getx() - point1.getx()))
            };
            let c_coeff: f64 = point1.gety() - (slope.unwrap() * point1.getx());
            return (1.0, slope, c_coeff);
        }
    }
}

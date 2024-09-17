use crate::point::*;

pub trait Segment2d {
    fn get_start(&self) -> Pointxy;
    fn get_end(&self) -> Pointxy;
    fn geta(&self) -> f64;
    fn getb(&self) -> f64;
    fn getc(&self) -> f64;
    fn getlen2d(&self) -> f64;
}

#[derive(Copy, Clone)]
pub struct Segmentxy {
    start: Pointxy,
    end: Pointxy,
    a_coeff: f64,
    b_coeff: f64,
    c_coeff: f64,
    len2d: f64,
}

impl Segmentxy {
    pub fn new(start: &impl Point2d, end: &impl Point2d) -> Segmentxy {
        let coeffs: (f64, f64, f64) = get_linear_eq(start, end);
        let a_coeff: f64 = coeffs.0;
        let b_coeff: f64 = coeffs.1;
        let c_coeff: f64 = coeffs.2;
        return Segmentxy {
            start: Pointxy::new(start.getx(), start.gety()),
            end: Pointxy::new(end.getx(), end.gety()),
            a_coeff,
            b_coeff,
            c_coeff,
            len2d: dist2d(start, end),
        };
    }
}

impl Segment2d for Segmentxy {
    fn get_start(&self) -> Pointxy {
        return self.start;
    }
    fn get_end(&self) -> Pointxy {
        return self.end;
    }
    fn geta(&self) -> f64 {
        return self.a_coeff;
    }
    fn getb(&self) -> f64 {
        return self.b_coeff;
    }
    fn getc(&self) -> f64 {
        return self.c_coeff;
    }
    fn getlen2d(&self) -> f64 {
        return self.len2d;
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Segmentz {
    start: Pointz,
    end: Pointz,
    a_coeff: f64,
    b_coeff: f64,
    c_coeff: f64,
    len2d: f64,
    len3d: f64,
}

impl Segmentz {
    pub fn new(start: &(impl Point3d + Point2d), end: &(impl Point3d + Point2d)) -> Segmentz {
        let coeffs: (f64, f64, f64) = get_linear_eq(start, end);
        let a_coeff = coeffs.0;
        let b_coeff = coeffs.1;
        let c_coeff = coeffs.2;
        return Segmentz {
            start: Pointz::new(
                Point3d::getx(start),
                Point3d::gety(start),
                Point3d::getz(start),
            ),
            end: Pointz::new(Point3d::getx(end), Point3d::gety(end), Point3d::getz(end)),
            a_coeff,
            b_coeff,
            c_coeff,
            len2d: dist2d(start, end),
            len3d: dist3d(start, end),
        };
    }
}

impl Segment2d for Segmentz {
    fn get_start(&self) -> Pointxy {
        return Pointxy::new(Point3d::getx(&self.start), Point3d::gety(&self.start));
    }
    fn get_end(&self) -> Pointxy {
        return Pointxy::new(Point3d::getx(&self.end), Point3d::gety(&self.end));
    }
    fn geta(&self) -> f64 {
        return self.a_coeff;
    }
    fn getb(&self) -> f64 {
        return self.b_coeff;
    }
    fn getc(&self) -> f64 {
        return self.c_coeff;
    }
    fn getlen2d(&self) -> f64 {
        return self.len2d;
    }
}

pub fn get_linear_eq(point1: &impl Point2d, point2: &impl Point2d) -> (f64, f64, f64) {
    match equal2d(point1, point2) {
        true => return (0.0, 0.0, 0.0),
        false => {
            let slope: f64 = (point2.gety() - point1.gety()) / (point2.getx() - point1.getx());
            let c_coeff: f64 = point1.gety() - (slope * point1.getx());
            return (1.0, slope, c_coeff);
        }
    }
}

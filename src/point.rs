use crate::line::*;

// TODO: refactor to use a single struct with optional fields

#[derive(Copy, Clone)]
pub struct Point {
    x_coord: f64,
    y_coord: f64,
    z_coord: Option<f64>,
    m_coord: Option<f64>,
}
impl Point {
    pub fn new(x_coord: f64, y_coord: f64, z_coord: Option<f64>, m_coord: Option<f64>) -> Point {
        Point {
            x_coord,
            y_coord,
            z_coord,
            m_coord,
        }
    }
    pub fn getx(&self) -> f64 {
        return self.x_coord;
    }
    pub fn gety(&self) -> f64 {
        return self.y_coord;
    }
    pub fn getz(&self) -> f64 {
        return self.z_coord.expect("no z coordinate");
    }
    pub fn getm(&self) -> f64 {
        return self.m_coord.expect("no m coordinate");
    }
}

pub fn dist2d(point1: &Point, point2: &Point) -> f64 {
    ((point1.getx() - point2.getx()).powi(2) + (point1.gety() - point2.gety()).powi(2)).sqrt()
}
pub fn dist3d(point1: &Point, point2: &Point) -> f64 {
    ((point1.getx() - point2.getx()).powi(2)
        + (point1.gety() - point2.gety()).powi(2)
        + (point1.getz() - point2.getz()).powi(2))
    .sqrt()
}
pub fn equal2d(point1: &Point, point2: &Point) -> bool {
    if point1.getx() == point2.getx() && point1.gety() == point2.gety() {
        true
    } else {
        false
    }
}

pub fn equal3d(point1: &Point, point2: &Point) -> bool {
    if point1.getx() == point2.getx()
        && point1.gety() == point2.gety()
        && point1.getz() == point2.getz()
    {
        true
    } else {
        false
    }
}

pub fn compare2d(point1: &Point, point2: &Point) -> i64 {
    if point1.getx() > point2.getx() {
        return 0;
    } else if point1.getx() < point2.getx() {
        return 1;
    } else {
        if point1.gety() > point2.gety() {
            return 0;
        } else if point1.gety() < point2.gety() {
            return 1;
        } else {
            return 2;
        }
    }
}

pub fn compare3d(point1: &Point, point2: &Point) -> i64 {
    if point1.getx() > point2.getx() {
        return 0;
    } else if point1.getx() < point2.getx() {
        return 1;
    } else {
        if point1.gety() > point2.gety() {
            return 0;
        } else if point1.gety() < point2.gety() {
            return 1;
        } else {
            if point1.getz() > point2.getz() {
                return 0;
            } else if point1.getz() < point2.getz() {
                return 1;
            } else {
                return 2;
            }
        }
    }
}

pub fn manhattan_dist2d(point1: &Point, point2: &Point) -> f64 {
    (point1.getx() - point2.getx()).abs() + (point1.gety() - point2.gety()).abs()
}

pub fn manhattan_dist3d(point1: &Point, point2: &Point) -> f64 {
    (point1.getx() - point2.getx()).abs()
        + (point1.gety() - point2.gety()).abs()
        + (point1.getz() - point2.getz()).abs()
}

pub fn dist2segment(point: &Point, segment: &Segment) -> f64 {
    let a_coeff: f64 = segment.geta();
    let b_coeff: Option<f64> = segment.getb();
    let c_coeff: f64 = segment.getc();
    let x_coord: f64 = point.getx();
    let y_coord: f64 = point.gety();
    let dist = if b_coeff == None {
        (x_coord - c_coeff).abs()
    } else {
        ((a_coeff * x_coord) + (b_coeff.unwrap() * y_coord) + c_coeff).abs()
            / (a_coeff.powi(2) + b_coeff.unwrap().powi(2)).sqrt()
    };
    return dist;
}

use crate::line::*;

pub trait Point2d {
    fn getx(&self) -> f64;
    fn gety(&self) -> f64;
}
pub trait Point3d {
    fn getx(&self) -> f64;
    fn gety(&self) -> f64;
    fn getz(&self) -> f64;
}

#[derive(Copy, Clone)]
pub struct Pointxy {
    x_coord: f64,
    y_coord: f64,
}
impl Pointxy {
    pub fn new(x_coord: f64, y_coord: f64) -> Pointxy {
        Pointxy { x_coord, y_coord }
    }
}
impl Point2d for Pointxy {
    fn getx(&self) -> f64 {
        return self.x_coord;
    }
    fn gety(&self) -> f64 {
        return self.y_coord;
    }
}

#[derive(Copy, Clone)]
pub struct Pointz {
    x_coord: f64,
    y_coord: f64,
    z_coord: f64,
}
impl Pointz {
    pub fn new(x_coord: f64, y_coord: f64, z_coord: f64) -> Pointz {
        Pointz {
            x_coord,
            y_coord,
            z_coord,
        }
    }
}
impl Point2d for Pointz {
    fn getx(&self) -> f64 {
        return self.x_coord;
    }
    fn gety(&self) -> f64 {
        return self.y_coord;
    }
}
impl Point3d for Pointz {
    fn getx(&self) -> f64 {
        return self.x_coord;
    }
    fn gety(&self) -> f64 {
        return self.y_coord;
    }
    fn getz(&self) -> f64 {
        return self.z_coord;
    }
}
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Pointm {
    x_coord: f64,
    y_coord: f64,
    m_coord: f64,
}
impl Pointm {
    pub fn new(x_coord: f64, y_coord: f64, m_coord: f64) -> Pointm {
        Pointm {
            x_coord,
            y_coord,
            m_coord,
        }
    }
}
impl Point2d for Pointm {
    fn getx(&self) -> f64 {
        return self.x_coord;
    }
    fn gety(&self) -> f64 {
        return self.y_coord;
    }
}
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Pointzm {
    x_coord: f64,
    y_coord: f64,
    z_coord: f64,
    m_coord: f64,
}
impl Pointzm {
    pub fn new(x_coord: f64, y_coord: f64, z_coord: f64, m_coord: f64) -> Pointzm {
        Pointzm {
            x_coord,
            y_coord,
            z_coord,
            m_coord,
        }
    }
}
impl Point2d for Pointzm {
    fn getx(&self) -> f64 {
        return self.x_coord;
    }
    fn gety(&self) -> f64 {
        return self.y_coord;
    }
}
impl Point3d for Pointzm {
    fn getx(&self) -> f64 {
        return self.x_coord;
    }
    fn gety(&self) -> f64 {
        return self.y_coord;
    }
    fn getz(&self) -> f64 {
        return self.z_coord;
    }
}

pub fn dist2d(point1: &impl Point2d, point2: &impl Point2d) -> f64 {
    ((point1.getx() - point2.getx()).powi(2) + (point1.gety() - point2.gety()).powi(2)).sqrt()
}
pub fn dist3d(point1: &impl Point3d, point2: &impl Point3d) -> f64 {
    ((point1.getx() - point2.getx()).powi(2)
        + (point1.gety() - point2.gety()).powi(2)
        + (point1.getz() - point2.getz()).powi(2))
    .sqrt()
}
pub fn equal2d(point1: &impl Point2d, point2: &impl Point2d) -> bool {
    if point1.getx() == point2.getx() && point1.gety() == point2.gety() {
        true
    } else {
        false
    }
}

pub fn equal3d(point1: &impl Point3d, point2: &impl Point3d) -> bool {
    if point1.getx() == point2.getx()
        && point1.gety() == point2.gety()
        && point1.getz() == point2.getz()
    {
        true
    } else {
        false
    }
}

pub fn compare2d(point1: &impl Point2d, point2: &impl Point2d) -> i64 {
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

pub fn compare3d(point1: &impl Point3d, point2: &impl Point3d) -> i64 {
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

pub fn manhattan_dist2d(point1: &impl Point2d, point2: &impl Point2d) -> f64 {
    (point1.getx() - point2.getx()).abs() + (point1.gety() - point2.gety()).abs()
}

pub fn manhattan_dist3d(point1: &impl Point3d, point2: &impl Point3d) -> f64 {
    (point1.getx() - point2.getx()).abs()
        + (point1.gety() - point2.gety()).abs()
        + (point1.getz() - point2.getz()).abs()
}

pub fn dist2segment(point: &impl Point2d, segment: &impl Segment2d) -> f64 {
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

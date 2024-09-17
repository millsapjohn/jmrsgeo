pub trait Point2d {
    fn getx(self) -> f64;
    fn gety(self) -> f64;
}
pub trait Point3d {
    fn getx(self) -> f64;
    fn gety(self) -> f64;
    fn getz(self) -> f64;
}

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
    fn getx(self) -> f64 {
        return self.x_coord;
    }
    fn gety(self) -> f64 {
        return self.y_coord;
    }
}
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
    fn getx(self) -> f64 {
        return self.x_coord;
    }
    fn gety(self) -> f64 {
        return self.y_coord;
    }
}
impl Point3d for Pointz {
    fn getx(self) -> f64 {
        return self.x_coord;
    }
    fn gety(self) -> f64 {
        return self.y_coord;
    }
    fn getz(self) -> f64 {
        return self.z_coord;
    }
}
#[allow(dead_code)]
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
    fn getx(self) -> f64 {
        return self.x_coord;
    }
    fn gety(self) -> f64 {
        return self.y_coord;
    }
}
#[allow(dead_code)]
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
    fn getx(self) -> f64 {
        return self.x_coord;
    }
    fn gety(self) -> f64 {
        return self.y_coord;
    }
}
impl Point3d for Pointzm {
    fn getx(self) -> f64 {
        return self.x_coord;
    }
    fn gety(self) -> f64 {
        return self.y_coord;
    }
    fn getz(self) -> f64 {
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

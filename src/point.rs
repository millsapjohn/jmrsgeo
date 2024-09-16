pub trait Point2d {
    fn cast_to_2d(self) -> Pointxy;
}
pub trait Point3d {
    fn cast_to_3d(self) -> Pointz;
}

pub struct Pointxy {
    x_coord: f64,
    y_coord: f64,
}
impl Pointxy {
    pub fn new(x_coord: f64, y_coord: f64) -> Pointxy {
        Pointxy { x_coord, y_coord }
    }
    pub fn dist2d(&self, point2: &Pointxy) -> f64 {
        ((self.x_coord - point2.x_coord).powi(2) + (self.y_coord - point2.y_coord).powi(2)).sqrt()
    }
}
impl Point2d for Pointxy {
    fn cast_to_2d(self) -> Pointxy {
        return self;
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
    pub fn dist3d(&self, point2: &Pointz) -> f64 {
        ((self.x_coord - point2.x_coord).powi(2)
            + (self.y_coord - point2.y_coord).powi(2)
            + (self.z_coord - point2.z_coord).powi(2))
        .sqrt()
    }
}
impl Point2d for Pointz {
    fn cast_to_2d(self) -> Pointxy {
        let new_point = Pointxy::new(self.x_coord, self.y_coord);
        return new_point;
    }
}
impl Point3d for Pointz {
    fn cast_to_3d(self) -> Pointz {
        return self;
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
    fn cast_to_2d(self) -> Pointxy {
        let new_point = Pointxy::new(self.x_coord, self.y_coord);
        return new_point;
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
    fn cast_to_2d(self) -> Pointxy {
        let new_point = Pointxy::new(self.x_coord, self.y_coord);
        return new_point;
    }
}
impl Point3d for Pointzm {
    fn cast_to_3d(self) -> Pointz {
        let new_point = Pointz::new(self.x_coord, self.y_coord, self.z_coord);
        return new_point;
    }
}

pub fn dist2d(point1: &Pointxy, point2: &Pointxy) -> f64 {
    ((point1.x_coord - point2.x_coord).powi(2) + (point1.y_coord - point2.y_coord).powi(2)).sqrt()
}
pub fn dist3d(point1: &Pointz, point2: &Pointz) -> f64 {
    ((point1.x_coord - point2.x_coord).powi(2)
        + (point1.y_coord - point2.y_coord).powi(2)
        + (point1.z_coord - point2.z_coord).powi(2))
    .sqrt()
}

pub mod Point {
    pub enum Point {
        Pointxy {
            x_coord: f64,
            y_coord: f64,
        },
        Pointz {
            x_coord: f64,
            y_coord: f64,
            z_coord: f64,
        },
        Pointm {
            x_coord: f64,
            y_coord: f64,
            m_coord: f64,
        },
        Pointzm {
            x_coord: f64,
            y_coord: f64,
            z_coord: f64,
            m_coord: f64,
        },
    }
    pub fn dist2d(point1: &Point, point2: &Point) -> f64 {
        let x_val = (point1.x_coord - point2.x_coord).powi(2);
        let y_val = (point1.y_coord - point2.y_coord).powi(2);
        let dist = (x_val + y_val).sqrt();
        return dist;
    }
}

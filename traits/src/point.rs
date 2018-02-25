use std;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

/*
impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
*/

/*
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
*/

/*
impl IntoIterator for Point {
    type Item = f32;
    type IntoIter = PointIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PointIntoIterator {
            point: self,
            index: 0,
        }
    }
}

pub struct PointIntoIterator {
    point: Point,
    index: usize,
}

impl std::iter::Iterator for PointIntoIterator {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let result = match self.index {
            0 => self.point.x,
            1 => self.point.y,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}
*/

/*
pub trait EuclideanDistance {
    fn dist_squared(&self, other: &Self) -> f32;
    fn dist(&self, other: &Self) -> f32 {
        self.dist_squared(other).sqrt()
    }
}

impl EuclideanDistance for Point {
    fn dist_squared(&self, other: &Point) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}
*/

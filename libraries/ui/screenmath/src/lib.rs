extern crate cgmath;

use cgmath::{Vector2, Point2, BaseNum};

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle<T> {
    pub min: Point2<T>,
    pub max: Point2<T>,
}

impl<S: BaseNum> Rectangle<S> {
    /// Creates a new rectangle.
    pub fn new(min: Point2<S>, max: Point2<S>) -> Self {
        Rectangle {
            min,
            max,
        }
    }

    /// Creates a new rectangle from a start coordinate and a size.
    pub fn start_size(min: Point2<S>, size: Vector2<S>) -> Self {
        Self::new(min, min + size)
    }

    /// Returns a new point with the start's X and the end's Y.
    pub fn min_max(&self) -> Point2<S> {
        Point2::new(self.min.x, self.max.y)
    }

    /// Returns a new point with the end's X and the start's Y.
    pub fn max_min(&self) -> Point2<S> {
        Point2::new(self.max.x, self.min.y)
    }

    pub fn size(&self) -> Vector2<S> {
        self.max - self.min
    }

    pub fn contains(&self, value: Point2<S>) -> bool {
        value.x >= self.min.x && value.y >= self.min.y &&
        value.x < self.max.x && value.y < self.max.y
    }
}

#[derive(Clone, Debug)]
pub struct Lrtb {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Lrtb {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Lrtb {
            left, right, top, bottom
        }
    }

    pub fn uniform(value: f32) -> Self {
        Self::new(value, value, value, value)
    }

    pub fn left_top(&self) -> Vector2<f32> {
        Vector2::new(self.left, self.top)
    }

    /// Takes the value and its own left value, then returns a new Lrtb with the maximum of the two
    /// left values.
    pub fn max_left(&self, value: f32) -> Self {
        let mut lrtb = self.clone();
        lrtb.left = f32::max(value, lrtb.left);
        lrtb
    }
}

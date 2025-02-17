// Copyright 2025 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::map::point::Point;

pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        let actual_top_left = Point::new(
            top_left.x.min(bottom_right.x),
            top_left.y.min(bottom_right.y),
        );
        let actual_bottom_right = Point::new(
            top_left.x.max(bottom_right.x),
            top_left.y.max(bottom_right.y),
        );
        Self {
            top_left: actual_top_left,
            bottom_right: actual_bottom_right,
        }
    }

    pub fn width(&self) -> i32 {
        self.bottom_right.x - self.top_left.x
    }

    pub fn height(&self) -> i32 {
        self.bottom_right.y - self.top_left.y
    }

    pub fn area(&self) -> i32 {
        self.width() * self.height()
    }

    pub fn contains(&self, point: Point) -> bool {
        self.top_left.x <= point.x
            && point.x < self.bottom_right.x
            && self.top_left.y <= point.y
            && point.y < self.bottom_right.y
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        !(self.bottom_right.x < other.top_left.x
            || other.bottom_right.x < self.top_left.x
            || self.bottom_right.y < other.top_left.y
            || other.bottom_right.y < self.top_left.y)
    }

    pub fn center(&self) -> Point {
        Point::new(
            self.top_left.x + self.width() / 2,
            self.top_left.y + self.height() / 2,
        )
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangles_are_created_correctly() {
        let top_left = Point::new(1, 2);
        let bottom_right = Point::new(3, 4);
        let rectangle = Rectangle::new(top_left, bottom_right);
        assert_eq!(rectangle.top_left, Point::new(1, 2));
        assert_eq!(rectangle.bottom_right, Point::new(3, 4));
        let rectangle = Rectangle::new(bottom_right, top_left);
        assert_eq!(rectangle.top_left, Point::new(1, 2));
        assert_eq!(rectangle.bottom_right, Point::new(3, 4));
    }

    #[test]
    fn rectange_width_height_and_area_are_calculated_correctly() {
        let top_left = Point::new(1, 2);
        let bottom_right = Point::new(3, 4);
        let rectangle = Rectangle::new(top_left, bottom_right);
        assert_eq!(rectangle.width(), 2);
        assert_eq!(rectangle.height(), 2);
        assert_eq!(rectangle.area(), 4);
    }

    #[test]
    fn rectangles_know_if_they_contain_a_point() {
        let top_left = Point::new(1, 2);
        let bottom_right = Point::new(3, 4);
        let rectangle = Rectangle::new(top_left, bottom_right);
        assert!(rectangle.contains(Point::new(1, 2)));
        assert!(rectangle.contains(Point::new(2, 3)));
        assert!(!rectangle.contains(Point::new(0, 2)));
        assert!(!rectangle.contains(Point::new(1, 1)));
        assert!(!rectangle.contains(Point::new(3, 4)));
        assert!(!rectangle.contains(Point::new(4, 3)));
    }

    #[test]
    fn rectangles_know_if_they_intersect() {
        let top_left = Point::new(1, 2);
        let bottom_right = Point::new(3, 4);
        let rectangle = Rectangle::new(top_left, bottom_right);
        let other = Rectangle::new(Point::new(2, 3), Point::new(4, 5));
        assert!(rectangle.intersects(&other));
        let other = Rectangle::new(Point::new(0, 1), Point::new(1, 2));
        assert!(rectangle.intersects(&other));
        let other = Rectangle::new(Point::new(4, 5), Point::new(5, 6));
        assert!(!rectangle.intersects(&other));
    }

    #[test]
    fn rectangles_know_their_center() {
        let top_left = Point::new(1, 2);
        let bottom_right = Point::new(3, 4);
        let rectangle = Rectangle::new(top_left, bottom_right);
        assert_eq!(rectangle.center(), Point::new(2, 3));
        let top_left = Point::new(1, 2);
        let bottom_right = Point::new(4, 5);
        let rectangle = Rectangle::new(top_left, bottom_right);
        assert_eq!(rectangle.center(), Point::new(2, 3));
        let top_left = Point::new(1, 2);
        let bottom_right = Point::new(5, 6);
        let rectangle = Rectangle::new(top_left, bottom_right);
        assert_eq!(rectangle.center(), Point::new(3, 4));
    }
}

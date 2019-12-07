use std::collections::HashMap;

// Point represents a point in the plane.
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn origin() -> Self {
        Point::new(0, 0)
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

// Wire represents the a wire, containing a Vec of all the points it traverses.
struct Wire {
    points: HashMap<Point, bool>,
}

impl Wire {
    fn new() -> Self {
        let mut wire = Wire {
            points: HashMap::new(),
        };

        let origin = Point::origin();

        wire.points.insert(origin, true);

        wire
    }

    fn add_point(&mut self, point: Point) {
        self.points.insert(point, true);
    }

    fn intersections(&self, other: &Wire) -> Vec<Point> {
        // TODO
    }
}

pub fn part_one() -> {
    let mut intersections: Vec<Point> = Vec::new();
}

pub fn load_instructions() -> Vec<String> {

}
use std::fmt;
use std::fmt::Formatter;
use std::time::SystemTime;
use rand::Rng;

fn main() {
    //let mut points: Vec<Point> = Vec::new();
    let start = SystemTime::now();
    let mut points_in_circle = 0;

    let zero_point = Point{ x: 0, y: 0 };

    let points_to_generate = 10000;

    for _ in 0..points_to_generate {
        let rx: i64 = rand::thread_rng().gen_range(-100..100);
        let ry: i64 = rand::thread_rng().gen_range(-100..100);
        let point = Point{x: rx, y: ry};
        // points.push(Point{ x: rx, y: ry });
        // points.push(Point{ x: 99, y: 99 });
        // println!("{}", points[a]);
        if point_distance(&point, &zero_point) < 100.0 {
            // println!("point was in circle!");
            points_in_circle = points_in_circle + 1;
        }
    }

    let pi_estimate: f64 = (points_in_circle as f64 / points_to_generate as f64) * 4.0;

    println!("{}", pi_estimate);
    let end = SystemTime::now();

    let diff = end.duration_since(start).unwrap();
    println!("{}", diff.as_secs_f32());
    // println!("len {}",points.len());
    // println!("points in circle {}", points_in_circle);

}

struct Point {
    x: i64,
    y: i64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Point: {}, {}", self.x, self.y)
    }
}

fn point_distance(p1: &Point, p2: &Point) -> f32 {
    let group1 = (p2.x - p1.x).pow(2);
    let group2 = (p2.y - p1.y).pow(2);
    let group3 = group2 + group1;
    (group3 as f32).sqrt()
}

impl Point {
    // fn get_x(&self) -> i64 {self.x}
    // fn get_y(&self) -> i64 {self.y}
}
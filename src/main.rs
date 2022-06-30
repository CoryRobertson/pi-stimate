#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt;
use std::fmt::{Formatter};
use eframe::egui;
use egui::{Color32, Pos2};
use rand::Rng;
use crate::egui::Vec2;

struct MyApp {
    points_in_circle: u64,
    points_generated: u64,
    pi_estimate: f64,
    points_per_frame:  i32,
    points: [Point; 10000],

}


impl Default for MyApp {
    fn default() -> Self {
        Self {
            points_in_circle: 0,
            points_generated: 0,
            pi_estimate: 0.0,
            points_per_frame: 1,
            points: [Point{ x: 0, y: 0, inside: true } ; 10000]
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();
            let pi_text = format!("Pi estimate: {}", &self.pi_estimate);
            let points_text = format!("Monte carlo points: {}", &self.points_generated);
            let points_in_circle_text = format!("Monte carlo points in circle: {}", &self.points_in_circle);
            ui.heading(pi_text);
            ui.heading(points_text);
            ui.heading(points_in_circle_text);

            ui.add(egui::Slider::new(&mut self.points_per_frame, 1..=1000));
            //let mut points: Vec<Point> = Vec::new();
            // let mut points_in_circle = 0;

            let zero_point = Point{ x: 0, y: 0, inside: false };

            //let points_to_generate_per_frame = 10;

            for _ in 0..self.points_per_frame {
                let rx: i64 = rand::thread_rng().gen_range(-100..100);
                let ry: i64 = rand::thread_rng().gen_range(-100..100);
                let mut point = Point{x: rx, y: ry, inside: false };
                self.points_generated = self.points_generated + 1;

                if point_distance(&point, &zero_point) < 100.0 {
                    // println!("point was in circle!");
                    self.points_in_circle = self.points_in_circle + 1;
                    point.inside = true;
                }
                let index = self.points_generated % self.points.len() as u64;
                self.points[index as usize] = point;

            }

            for point in self.points {
                let x: f32 = 250.0 + point.x as f32;
                let y: f32 = 300.0 + point.y as f32;
                let color = match point.inside {
                    true => {Color32::from_rgb(173,18,7)}
                    false => {Color32::from_rgb(125,125,125)}
                };

                ui.painter().circle_filled(Pos2::new(x, y), 3.5, color);

            }

            self.pi_estimate = (self.points_in_circle as f64 / self.points_generated as f64) * 4.0;

        });
    }
}

fn main() {

    let mut options = eframe::NativeOptions::default();

    options.resizable = false;
    options.initial_window_size = Option::from(Vec2::new(500.0, 500.0));

    eframe::run_native(
        "Pi-stimate",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );


}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    inside: bool,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Point: {}, {}, {}", self.x, self.y, self.inside)
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
extern crate physics;

use physics::harmonic_movement;
use std::io::{stdin, stdout, Write};
use std::fs::File;
use std::path::Path;

macro_rules! read {
    () => (
        {
            let mut input = String::new();
            stdout().flush().expect("Failed to flush stdout");
            stdin().read_line(&mut input).expect("Failed to read line from stdin");
            input
        }
    );
}

fn pendulum_ideal_simulation() {
    eprint!("angle, speed, length, gravity: ");
    let data = read!();
    let data: Vec<&str> = data.trim().split_whitespace().collect();
    if data.len() != 4 { panic!("Please input your fixed data"); }

    let angle: f64 = data[0].parse().unwrap();
    let speed: f64 = data[1].parse().unwrap();
    let length: f64 = data[2].parse().unwrap();
    let gravity: f64 = data[3].parse().unwrap();

    eprint!("steps, delta: ");
    let data = read!();
    let data: Vec<&str> = data.trim().split_whitespace().collect();
    if data.len() != 2 { panic!("Please input your fixed data"); }

    let steps: usize = data[0].parse().unwrap();
    let delta: f64 = data[1].parse().unwrap();

    eprint!("angle and speed output names: ");
    let data = read!();
    let data: Vec<&str> = data.trim().split_whitespace().collect();
    if data.len() != 2 { panic!("Please input your fixed data"); }

    let angle_file = data[0];
    let speed_file = data[1];

    let path = Path::new(angle_file);
    let mut angle_file = File::create(path).unwrap();

    let path = Path::new(speed_file);
    let mut speed_file = File::create(path).unwrap();

    let data = harmonic_movement::ideal::pendulum(speed, angle, length, gravity, steps, delta);

    for d in &data {
        angle_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.angle)).unwrap();
        speed_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.speed)).unwrap();
    }
}

fn pendulum_real_simulation() {
    eprint!("angle, speed, length, gravity, air constant: ");
    let data = read!();
    let data: Vec<&str> = data.trim().split_whitespace().collect();
    if data.len() != 5 { panic!("Please input your fixed data"); }

    let angle: f64 = data[0].parse().unwrap();
    let speed: f64 = data[1].parse().unwrap();
    let length: f64 = data[2].parse().unwrap();
    let gravity: f64 = data[3].parse().unwrap();
    let air_const: f64 = data[4].parse().unwrap();

    eprint!("steps, delta: ");
    let data = read!();
    let data: Vec<&str> = data.trim().split_whitespace().collect();
    if data.len() != 2 { panic!("Please input your fixed data"); }

    let steps: usize = data[0].parse().unwrap();
    let delta: f64 = data[1].parse().unwrap();

    eprint!("angle and speed output names: ");
    let data = read!();
    let data: Vec<&str> = data.trim().split_whitespace().collect();
    if data.len() != 2 { panic!("Please input your fixed data"); }

    let angle_file = data[0];
    let speed_file = data[1];

    let path = Path::new(angle_file);
    let mut angle_file = File::create(path).unwrap();

    let path = Path::new(speed_file);
    let mut speed_file = File::create(path).unwrap();

    let data = harmonic_movement::real::pendulum(speed, angle, length, gravity, air_const, steps, delta);

    for d in &data {
        angle_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.angle)).unwrap();
        speed_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.speed)).unwrap();
    }
}

fn spring_ideal_simulation() {
    eprint!("mass, k, position, speed: ");
    let data = read!();
    let data: Vec<&str> = data.trim().split_whitespace().collect();
    if data.len() != 4 { panic!("Please input your fixed data"); }

    let mass: f64 = data[0].parse().unwrap();
    let spring_k: f64 = data[1].parse().unwrap();
    let position: f64 = data[2].parse().unwrap();
    let speed: f64 = data[3].parse().unwrap();

    eprint!("steps, delta: ");
    let data = read!();
    let data: Vec<&str> = data.trim().split_whitespace().collect();
    if data.len() != 2 { panic!("Please input your fixed data"); }

    let steps: usize = data[0].parse().unwrap();
    let delta: f64 = data[1].parse().unwrap();

    eprint!("position and speed output names: ");
    let data = read!();
    let data: Vec<&str> = data.trim().split_whitespace().collect();
    if data.len() != 2 { panic!("Please input your fixed data"); }

    let position_file = data[0];
    let speed_file    = data[1];

    let path = Path::new(position_file);
    let mut position_file = File::create(path).unwrap();

    let path = Path::new(speed_file);
    let mut speed_file = File::create(path).unwrap();

    let data = harmonic_movement::ideal::spring(mass, spring_k, position, speed, steps, delta);

    for d in &data {
        position_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.position)).unwrap();
        speed_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.speed)).unwrap();
    }
}

fn spring_real_simulation() {
    eprint!("mass, k, position, speed, b: ");
    let data = read!();
    let data: Vec<&str> = data.trim().split_whitespace().collect();
    if data.len() != 5 { panic!("Please input your fixed data"); }

    let mass: f64 = data[0].parse().unwrap();
    let spring_k: f64 = data[1].parse().unwrap();
    let position: f64 = data[2].parse().unwrap();
    let speed: f64 = data[3].parse().unwrap();
    let b_const: f64 = data[4].parse().unwrap();

    eprint!("steps, delta: ");
    let data = read!();
    let data: Vec<&str> = data.trim().split_whitespace().collect();
    if data.len() != 2 { panic!("Please input your fixed data"); }

    let steps: usize = data[0].parse().unwrap();
    let delta: f64 = data[1].parse().unwrap();

    eprint!("position and speed output names: ");
    let data = read!();
    let data: Vec<&str> = data.trim().split_whitespace().collect();
    if data.len() != 2 { panic!("Please input your fixed data"); }

    let position_file = data[0];
    let speed_file    = data[1];

    let path = Path::new(position_file);
    let mut position_file = File::create(path).unwrap();

    let path = Path::new(speed_file);
    let mut speed_file = File::create(path).unwrap();

    let data = harmonic_movement::real::spring(mass, spring_k, position, speed, b_const, steps, delta);

    for d in &data {
        position_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.position)).unwrap();
        speed_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.speed)).unwrap();
    }
}

fn main() {
    println!("Choose method to simulate: ");
    println!("1) Pendulum without air resistance");
    println!("2) Pendulum with air resistance");
    println!("3) Spring without cushion");
    println!("4) Spring with cushion");
    print!("Option: ");
    let opt: i32 = read!().trim().parse().unwrap();
    match opt {
        1 => pendulum_ideal_simulation(),
        2 => pendulum_real_simulation(),
        3 => spring_ideal_simulation(),
        4 => spring_real_simulation(),
        _ => { panic!("Unknown option!!") }
    }
}

extern crate physics;
extern crate criterion_plot as plot;

#[macro_use]
extern crate text_io;

use std::{
  fs::File,
  io::{Write, stdout},
  path::Path
};

use physics::{
  lotka_volterra,
  harmonic_movement
};

use plot::prelude::*;

macro_rules! scan {
  () => ( { stdout().flush().expect("Failed to flush"); read!() } );
}

fn pendulum_ideal_simulation() {
  eprint!("angle, speed, length, gravity: ");
  let angle: f64 = read!();
  let speed: f64 = read!();
  let length: f64 = read!();
  let gravity: f64 = read!();

  eprint!("steps, delta: ");
  let steps: usize = read!();
  let delta: f64 = read!();

  eprint!("angle and speed output names: ");
  let angle_file: String = read!();
  let speed_file: String = read!();

  let path = Path::new(angle_file.as_str());
  let mut angle_file = File::create(path).unwrap();

  let path = Path::new(speed_file.as_str());
  let mut speed_file = File::create(path).unwrap();

  let data = harmonic_movement::ideal::pendulum(speed, angle, length, gravity, steps, delta);

  let mut angles = Vec::new();
  let mut speeds = Vec::new();
  let mut times  = Vec::new();

  for d in &data {
      angles.push(d.angle);
      speeds.push(d.speed);
      times.push(d.time);
      angle_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.angle)).unwrap();
      speed_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.speed)).unwrap();
  }

  let mut figure = Figure::new();
  figure.configure(Key, | k | {
    k.set(Boxed::Yes);
    k.set(Position::Inside(Vertical::Top, Horizontal::Left))
  });

  figure.plot(Lines { x: times.clone(), y: angles.clone() }, | lp | {
    lp.set(Color::Cyan);
    lp.set(Label("angle"));
    lp.set(LineType::Dash)
  });

  figure.plot(Lines { x: times.clone(), y: speeds.clone() }, | lp | {
    lp.set(Color::Gray);
    lp.set(Label("speed"));
    lp.set(LineType::Dash)
  });

  figure.draw().expect("Failed to plot info");
}

fn pendulum_real_simulation() {
  eprint!("angle, speed, length, gravity, air constant: ");
  let angle: f64 = scan!();
  let speed: f64 = scan!();
  let length: f64 = scan!();
  let gravity: f64 = scan!();
  let air_const: f64 = scan!();

  eprint!("steps, delta: ");
  let steps: usize = scan!();
  let delta: f64 = scan!();

  eprint!("angle and speed output names: ");
  let angle_file: String = scan!();
  let speed_file: String = scan!();

  let path = Path::new(angle_file.as_str());
  let mut angle_file = File::create(path).unwrap();

  let path = Path::new(speed_file.as_str());
  let mut speed_file = File::create(path).unwrap();

  let data = harmonic_movement::real::pendulum(speed, angle, length, gravity, air_const, steps, delta);

  let mut angles = Vec::new();
  let mut speeds = Vec::new();
  let mut times  = Vec::new();

  for d in &data {
      angles.push(d.angle);
      speeds.push(d.speed);
      times.push(d.time);
      angle_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.angle)).unwrap();
      speed_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.speed)).unwrap();
  }

  let mut figure = Figure::new();
  figure.configure(Key, | k | {
    k.set(Boxed::Yes);
    k.set(Position::Inside(Vertical::Top, Horizontal::Left))
  });

  figure.plot(Lines { x: times.clone(), y: angles.clone() }, | lp | {
    lp.set(Color::Cyan);
    lp.set(Label("angle"));
    lp.set(LineType::Dash)
  });

  figure.plot(Lines { x: times.clone(), y: speeds.clone() }, | lp | {
    lp.set(Color::Gray);
    lp.set(Label("speed"));
    lp.set(LineType::Dash)
  });

  figure.draw().expect("Failed to plot info");
}

fn spring_ideal_simulation() {
  eprint!("mass, k, position, speed: ");
  let mass: f64 = scan!();
  let spring_k: f64 = scan!();
  let position: f64 = scan!();
  let speed: f64 = scan!();

  eprint!("steps, delta: ");
  let steps: usize = scan!();
  let delta: f64 = scan!();

  eprint!("position and speed output names: ");
  let position_file: String = scan!();
  let speed_file: String = scan!();

  let path = Path::new(position_file.as_str());
  let mut position_file = File::create(path).unwrap();

  let path = Path::new(speed_file.as_str());
  let mut speed_file = File::create(path).unwrap();

  let data = harmonic_movement::ideal::spring(mass, spring_k, position, speed, steps, delta);

  let mut positions = Vec::new();
  let mut speeds    = Vec::new();
  let mut times     = Vec::new();

  for d in &data {
    positions.push(d.position);
    speeds.push(d.speed);
    times.push(d.time);
    position_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.position)).unwrap();
    speed_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.speed)).unwrap();
  }

  let mut figure = Figure::new();
  figure.configure(Key, | k | {
    k.set(Boxed::Yes);
    k.set(Position::Inside(Vertical::Top, Horizontal::Left))
  });

  figure.plot(Lines { x: times.clone(), y: positions.clone() }, | lp | {
    lp.set(Color::Cyan);
    lp.set(Label("position"));
    lp.set(LineType::Dash)
  });

  figure.plot(Lines { x: times.clone(), y: speeds.clone() }, | lp | {
    lp.set(Color::Gray);
    lp.set(Label("speed"));
    lp.set(LineType::Dash)
  });

  figure.draw().expect("Failed to plot info");
}

fn spring_real_simulation() {
  eprint!("mass, k, position, speed, b: ");
  let mass: f64 = scan!();
  let spring_k: f64 = scan!();
  let position: f64 = scan!();
  let speed: f64 = scan!();
  let b_const: f64 = scan!();

  eprint!("steps, delta: ");
  let steps: usize = scan!();
  let delta: f64 = scan!();

  eprint!("position and speed output names: ");
  let position_file: String = scan!();
  let speed_file: String = scan!();

  let path = Path::new(position_file.as_str());
  let mut position_file = File::create(path).unwrap();

  let path = Path::new(speed_file.as_str());
  let mut speed_file = File::create(path).unwrap();

  let data = harmonic_movement::real::spring(mass, spring_k, position, speed, b_const, steps, delta).unwrap();

  let mut positions = Vec::new();
  let mut speeds    = Vec::new();
  let mut times     = Vec::new();

  for d in &data {
    positions.push(d.position);
    speeds.push(d.speed);
    times.push(d.time);
    position_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.position)).unwrap();
    speed_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.speed)).unwrap();
  }

  let mut figure = Figure::new();
  figure.configure(Key, | k | {
    k.set(Boxed::Yes);
    k.set(Position::Inside(Vertical::Top, Horizontal::Left))
  });

  figure.plot(Lines { x: times.clone(), y: positions.clone() }, | lp | {
    lp.set(Color::Cyan);
    lp.set(Label("position"));
    lp.set(LineType::Dash)
  });

  figure.plot(Lines { x: times.clone(), y: speeds.clone() }, | lp | {
    lp.set(Color::Gray);
    lp.set(Label("speed"));
    lp.set(LineType::Dash)
  });

  figure.draw().expect("Failed to plot info");
}

fn lotka_volterra_euler_simulation() {
  eprint!("initial poblation A, initial poblation B: ");
  let pa: f64 = scan!();
  let pb: f64 = scan!();

  eprint!("a, b, c, d: ");
  let a: f64 = scan!();
  let b: f64 = scan!();
  let c: f64 = scan!();
  let d: f64 = scan!();

  eprint!("initial time, final time, number of interactions: ");
  let t0: f64 = scan!();
  let tf: f64 = scan!();
  let n: usize = scan!();

  eprint!("poblation a file, poblation b file: ");
  let poba_file: String = scan!();
  let pobb_file: String = scan!();

  let path = Path::new(&poba_file);
  let mut poba_file = File::create(path).unwrap();

  let path = Path::new(&pobb_file);
  let mut pobb_file = File::create(path).unwrap();

  let data = lotka_volterra::dam_predator_with_euler(pa, pb, a, b, c, d, t0, tf, n);

  let mut time = Vec::new();
  let mut poba = Vec::new();
  let mut pobb = Vec::new();

  for d in &data {
    time.push(d.time);
    poba.push(d.pa);
    pobb.push(d.pb);
    poba_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.pa)).unwrap();
    pobb_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.pb)).unwrap();
  }

  let mut figure = Figure::new();
  figure.configure(Key, | k | {
    k.set(Boxed::Yes);
    k.set(Position::Inside(Vertical::Top, Horizontal::Left))
  });

  figure.plot(Lines { x: time.clone(), y: poba.clone() }, | lp | {
    lp.set(Color::Cyan);
    lp.set(Label("poblation a"));
    lp.set(LineType::Dash)
  });

  figure.plot(Lines { x: time.clone(), y: pobb.clone() }, | lp | {
    lp.set(Color::Gray);
    lp.set(Label("poblation b"));
    lp.set(LineType::Dash)
  });

  figure.draw().expect("Failed to plot info");
}

fn lotka_volterra_rkutta_simulation() {
  eprint!("initial poblation A, initial poblation B: ");
  let pa: f64 = scan!();
  let pb: f64 = scan!();

  eprint!("a, b, c, d: ");
  let a: f64 = scan!();
  let b: f64 = scan!();
  let c: f64 = scan!();
  let d: f64 = scan!();

  eprint!("initial time, final time, number of interactions: ");
  let t0: f64 = scan!();
  let tf: f64 = scan!();
  let n: usize = scan!();

  eprint!("poblation a file, poblation b file: ");
  let poba_file: String = scan!();
  let pobb_file: String = scan!();

  let path = Path::new(&poba_file);
  let mut poba_file = File::create(path).unwrap();

  let path = Path::new(&pobb_file);
  let mut pobb_file = File::create(path).unwrap();

  let data = lotka_volterra::dam_predator_with_rkutta(pa, pb, a, b, c, d, t0, tf, n);

  let mut time = Vec::new();
  let mut poba = Vec::new();
  let mut pobb = Vec::new();

  for d in &data {
    time.push(d.time);
    poba.push(d.pa);
    pobb.push(d.pb);
    poba_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.pa)).unwrap();
    pobb_file.write_fmt(format_args!("{:.6} {:.6}\n", d.time, d.pb)).unwrap();
  }

  let mut figure = Figure::new();
  figure.configure(Key, | k | {
    k.set(Boxed::Yes);
    k.set(Position::Inside(Vertical::Top, Horizontal::Left))
  });

  figure.plot(Lines { x: time.clone(), y: poba.clone() }, | lp | {
    lp.set(Color::Cyan);
    lp.set(Label("poblation a"));
    lp.set(LineType::Dash)
  });

  figure.plot(Lines { x: time.clone(), y: pobb.clone() }, | lp | {
    lp.set(Color::Gray);
    lp.set(Label("poblation b"));
    lp.set(LineType::Dash)
  });

  figure.draw().expect("Failed to plot info");
}

fn main() {
    println!("Choose method to simulate: ");
    println!("1) Pendulum without air resistance");
    println!("2) Pendulum with air resistance");
    println!("3) Spring without cushion");
    println!("4) Spring with cushion");
    println!("5) Lotka-Volterra predator dam (euler method)");
    println!("6) Lotka-Volterra predator dam (runge-kutta 4th order method)");
    print!("Option: ");
    let opt: i32 = scan!();
    match opt {
        1 => pendulum_ideal_simulation(),
        2 => pendulum_real_simulation(),
        3 => spring_ideal_simulation(),
        4 => spring_real_simulation(),
        5 => lotka_volterra_euler_simulation(),
        6 => lotka_volterra_rkutta_simulation(),
        _ => { panic!("Unknown option!!") }
    }
}

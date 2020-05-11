extern crate amb_rs as lib;

use lib::Particle;

const GRAVITY: f32 = 9.8; // [m/sec^2]

pub fn main() {
    let gravity = -GRAVITY; // [m/sec^2]
    let velocity = 0.; // [m/s]
    let position = 1.; // [m]
    let step = 1E-3; // [sec]

    let mut p = Particle::new(step, gravity, velocity, position);

    println!(
        "t: {}, a: {}, v: {}, p: {}\n",
        "[sec]", "[m/s^2]", "[m/s]", "[m]"
    );
    let mut t = 0.;
    loop {
        print!("{}, {}, {}, {} \n", t, p.accel, p.velocity, p.position);
        if p.position <= 0. {
            break;
        }
        p.time_evolution(t);
        t += step;
    }
}

#[allow(dead_code)]
fn without_rk4th(gravity: f32, velocity: f32, position: f32, step: f32) {
    println!("without rk4th");
    print!("t: {}, v: {} \n", "[sec]", "[m/s]");
    let mut t = 0.;
    loop {
        let accelration = gravity;
        let velocity1 = velocity + accelration * t;
        let position1 = position + accelration * t * t / 2.;
        print!("t: {}, v: {}, d: {} \n", t, velocity1, position1);
        if position1 <= 0. {
            break;
        }
        t += step;
    }
}

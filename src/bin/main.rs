extern crate amb_rs as lib;

use lib::RungeKutta4th;

const GRAVITY: f32 = 9.8; // [m/sec^2]

pub fn main() {
    let gravity = -GRAVITY; // [m/sec^2]
    let velocity = 0.; // [m/s]
    let position = 1.; // [m]
    let step = 1E-3; // [sec]

    with_rk4th(gravity, velocity, position, step);
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

fn with_rk4th(accel: f32, velocity: f32, position: f32, step: f32) {
    println!("with rk4th");
    let mut p = Particle::new(step, accel, velocity, position);

    print!("t: {}, v: {} \n", "[sec]", "[m/s]");
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

struct Particle {
    dt: f32,
    accel: f32,
    velocity: f32,
    position: f32,
}

impl Particle {
    pub fn new(dt: f32, accel: f32, velocity: f32, position: f32) -> Self {
        Self {
            dt,
            accel,
            velocity,
            position,
        }
    }

    fn time_evolution(&mut self, t: f32) {
        let (dp, dv) =
            self.time_evolution_core(self.dt, t, self.velocity, self.position);
        self.position += dp;
        self.velocity += dv;
    }
}

impl RungeKutta4th for Particle {
    fn to_velocity(&self, _: f32, _: f32, velocity: f32) -> f32 {
        return velocity;
    }

    fn to_accel(&self, _: f32, _: f32, _: f32) -> f32 {
        return self.accel;
    }
}

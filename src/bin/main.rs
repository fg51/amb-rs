extern crate landing as lib;

pub fn main() {
    lib::learn();
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

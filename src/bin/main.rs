pub fn main() {
    let gravity = -9.8; // [m/sec^2]
    let velocity = 0.; // [m/s]
    let position = 1.; // [m]
    let step = 1E-3; // [sec]

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

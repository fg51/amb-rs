use crate::rk4th::RungeKutta4th;

pub struct Particle {
    dt: f32,
    pub accel: f32,
    pub velocity: f32,
    pub position: f32,
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

    pub fn time_evolution(&mut self, t: f32) {
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

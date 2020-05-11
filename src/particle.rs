use crate::rk4th::RungeKutta4th;

type Vector3 = nalgebra::Vector3<f32>;

pub struct Particle {
    dt: f32,
    pub gravity: Vector3,
    pub velocity: Vector3,
    pub position: Vector3,
    retrofire: Vector3,
    is_retrofire: bool,
}

impl Particle {
    pub fn new(
        dt: f32,
        gravity: Vector3,
        velocity: Vector3,
        position: Vector3,
        retrofire: Vector3,
    ) -> Self {
        Self {
            dt,
            gravity,
            velocity,
            position,
            retrofire,
            is_retrofire: false,
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
    fn to_velocity(&self, _: f32, _: &Vector3, velocity: &Vector3) -> Vector3 {
        return velocity.clone();
    }

    fn to_accel(&self, _: f32, _: &Vector3, _: &Vector3) -> Vector3 {
        if self.is_retrofire {
            self.gravity + self.retrofire
        } else {
            self.gravity
        }
    }
}

pub trait RungeKutta4th {
    // fn time_evolution(&mut self, t: f32);
    fn to_velocity(&self, t: f32, position: f32, velocity: f32) -> f32;
    fn to_accel(&self, t: f32, position: f32, velocity: f32) -> f32;

    fn time_evolution_core(&mut self, dt: f32, t: f32, v0: f32, p0: f32) -> (f32, f32) {
        // NOTE: 1st
        let a1 = self.to_accel(t, p0, v0);
        let v1 = self.to_velocity(t, p0, v0);
        let a1t = Self::step1st(dt, v0, a1);
        let v1t = Self::step1st(dt, p0, v1);

        // NOTE: 2nd
        let a2 = self.to_accel(t + dt / 2.0, v1t, a1t);
        let v2 = self.to_velocity(t + dt / 2.0, v1t, a1t);
        let a2t = Self::step2nd(dt, v0, a2);
        let v2t = Self::step2nd(dt, p0, v2);

        // NOTE: 3rd
        let v3 = self.to_velocity(t + dt / 2.0, v2t, a2t);
        let a3 = self.to_accel(t + dt / 2.0, v2t, a2t);
        let v3t = Self::step3rd(dt, p0, v3);
        let a3t = Self::step3rd(dt, v0, a3);

        // NOTE: 4th
        let v4 = self.to_velocity(t + dt, v3t, a3t);
        let a4 = self.to_accel(t + dt, v3t, a3t);
        let delta_p = Self::step4th(dt, v1, v2, v3, v4);
        let delta_v = Self::step4th(dt, a1, a2, a3, a4);

        (delta_p, delta_v)
    }

    fn step1st(dt: f32, x: f32, dx: f32) -> f32 {
        x + dx * dt / 2.0
    }

    fn step2nd(dt: f32, x: f32, dx: f32) -> f32 {
        x + dx * dt / 2.0
    }

    fn step3rd(dt: f32, x: f32, dx: f32) -> f32 {
        x + dx * dt
    }

    fn step4th(dt: f32, x1: f32, x2: f32, x3: f32, x4: f32) -> f32 {
        dt / 6.0 * (x1 + 2.0 * x2 + 2.0 * x3 + x4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Mock;

    impl RungeKutta4th for Mock {
        fn to_velocity(&self, _t: f32, _: f32, _velocity: f32) -> f32 {
            unimplemented!();
        }

        fn to_accel(&self, _t: f32, _: f32, _: f32) -> f32 {
            unimplemented!();
        }
    }

    #[test]
    fn rungekutta4th_step1st() {
        let v = Mock::step1st(4., 1., 10.);
        assert_eq!(v, 21.);
    }

    #[test]
    fn rungekutta4th_step2nd() {
        let v = Mock::step2nd(4., 1., 10.);
        assert_eq!(v, 21.);
    }

    #[test]
    fn rungekutta4th_step3rd() {
        let v = Mock::step3rd(2., 1., 10.);
        assert_eq!(v, 21.);
    }

    #[test]
    fn rungekutta4th_step4th() {
        let v = Mock::step4th(6., 1., 10., 100., 1000.);
        assert_eq!(v, 1221.);
    }
}

use crate::agent::Agent;
use crate::particle::Particle;
use crate::types::Vector3;

pub struct Environment;

const GRAVITY: f32 = 9.8; // [m/sec^2]
const MAX_TIME: f32 = 1000.; // [sec]
const END_POINT: f32 = 0.; // [0]

impl Environment {
    pub fn learn(&self, agent: &mut Agent) {
        for _ in 0..1 {
            self.learn_one_term(agent);
            agent.update();
        }
    }

    pub fn learn_one_term(&self, agent: &mut Agent) {
        let accel = Vector3::new(0., -GRAVITY, 0.); // [m/sec^2]
        let velocity = Vector3::zeros(); // [m/s]
        let position = Vector3::new(0., 1., 0.); // [m]
        let retrofire = Vector3::new(0., 20., 0.); // [m/sec^2]
        let step = 1E-3; // [sec]

        let mut p = Particle::new(step, accel, velocity, position, retrofire);

        // println!("t: {}, v: {}, p: {}\n", "[sec]", "[m/s]", "[m]");
        let mut t = 0.;
        loop {
            print!("{}, {}, {}\n", t, p.velocity.y, p.position.y);
            if t >= MAX_TIME {
                break;
            }
            if p.position.y <= END_POINT {
                break;
            }
            p.set_action(agent.action(t));
            p.time_evolution(t);
            t += step;
        }
        agent.set_reward(self.calculate_reward(&p.velocity));
    }

    pub fn calculate_reward(&self, v: &Vector3) -> f32 {
        let distance = 1.;
        let t = (2. * distance / GRAVITY).sqrt();
        let v_max = -GRAVITY * t;
        return v.y - v_max;
    }
}

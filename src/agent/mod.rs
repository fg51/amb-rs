pub struct Agent {
    q_values: Vec<Qvalue>,
    triger: f32,
    action: bool,
    eta: f32,   // NOTE: learning ratio
    gamma: f32, // NOTE: discount ratio
}

impl Agent {
    pub fn new() -> Self {
        Self {
            q_values: vec![],
            triger: 100.,
            action: false,
            eta: 0.1,
            gamma: 1.0,
        }
    }

    pub fn action(&mut self, t: f32) -> bool {
        if t > self.triger {
            self.action = true;
        }
        self.action
    }

    pub fn set_reward(&mut self, term: usize, reward: f32) {
        // for i in (0..term + 1).rev() {
        // let discount = self.gamma.powi(term - i);
        //     let q = self.eta * reward * discount;
        //     self.q_values.push(Qvalue::new(self.triger, reward));
        // }
        let q = {
            // let discount = self.gamma.powi(term as i32);
            let discount = 1.0;
            self.eta * reward * discount
        };
        self.q_values.push(Qvalue::new(self.triger, q));
        // for i in 0..term + 1 {
        //     let discount = self.gamma.powi((term - i) as i32);
        //     self.q_values[i].q_value += self.eta * reward * discount;
        // }
    }

    pub fn update(&mut self) {
        self.triger = 100.;
    }
}

#[derive(Clone)]
struct Qvalue {
    t: f32,
    q_value: f32,
}

impl Qvalue {
    pub fn new(t: f32, q_value: f32) -> Self {
        Self { t, q_value }
    }
}

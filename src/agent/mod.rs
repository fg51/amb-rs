pub struct Agent {
    q_values: Vec<Qvalue>,
    triger: f32,
    action: bool,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            q_values: vec![],
            triger: 100.,
            action: false,
        }
    }

    pub fn action(&mut self, t: f32) -> bool {
        if t > self.triger {
            self.action = true;
        }
        self.action
    }

    pub fn set_reward(&mut self, reward: f32) {
        self.q_values.push(Qvalue::new(self.triger, reward));
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

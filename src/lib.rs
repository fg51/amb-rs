mod agent;
use agent::Agent;

mod environment;
use environment::Environment;

mod particle;

mod rk4th;
mod types;

pub fn learn() {
    let env = Environment;
    let mut agent = Agent::new();
    env.learn(&mut agent);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

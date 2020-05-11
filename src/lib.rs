mod rk4th;

mod particle;
pub use particle::Particle;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod rk4th;
pub use rk4th::RungeKutta4th;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

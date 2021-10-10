use crate::random::default::DefaultRandomGenerator;

mod default;

pub trait PRNG {
    // TODO Give these better names once I've figured out what they are.
    fn p_random(&mut self) -> i32;
    fn m_random(&mut self) -> i32;
    fn reset(&mut self);
}

pub fn create_random_generator() -> Box<dyn PRNG> {
    Box::new(DefaultRandomGenerator::new())
}
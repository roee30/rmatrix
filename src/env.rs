use rand::{rngs::ThreadRng, Rng};
pub struct Size {
    pub rows: u16,
    pub cols: u16,
}
pub struct Env {
    rng: ThreadRng,
    pub size: Size,
    pub max_flakes: u16,
    pub max_length: u16,
    pub delay_base: f64,
    pub speed: u16,
}
impl Env {
    pub fn make(size: Size) -> Self {
        let max_flakes = size.cols * 2;
        Self {
            size: size,
            max_flakes: max_flakes,
            rng: rand::thread_rng(),
            max_length: 10,
            speed: 10,
            delay_base: 0.05,
        }
    }

    pub fn new_flake_column(&mut self) -> u16 {
        self.rng.gen_range(0..self.size.cols)
    }

    pub fn random_char(&mut self) -> char {
        std::char::from_u32(self.rng.gen_range(0xff65..=0xff9d)).unwrap()
    }
}

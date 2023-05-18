use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct Generator {
    pub seed: u64,
    rng: SmallRng,
}
impl Generator {
    pub fn from_seed(seed: u64) -> Self {
        let rng = SmallRng::seed_from_u64(seed);

        Self { seed, rng }
    }
}
impl Generator {
    pub fn image_url(&mut self, dimension: (usize, usize), keywords: &[String]) -> String {
        let cache_buster = self.rng.gen::<u16>();
        let (width, height) = dimension;
        format!(
            "https://source.unsplash.com/random/{}x{}?{}&sig={}",
            width,
            height,
            keywords.join(","),
            cache_buster
        )
    }
}

pub trait Generated: Sized {
    fn generate(gen: &mut Generator) -> Self;
    fn generate_from_seed(seed: u64) -> Self {
        Self::generate(&mut Generator::from_seed(seed))
    }
}

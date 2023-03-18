#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod models;
// pub mod diffusion;

pub mod wiener_process {
    // implementation of the Wiener process

    use rand::distributions::Standard;
    use rand::Rng;
    use rand_distr::Distribution;

    pub fn wiener_fn<T: Rng>(rng: &mut T, n: usize, dt: f64) -> Vec<f64> {
        let mut w = vec![0.0; n];
        for i in 1..n {
            let rand_value: f64 = Standard.sample(rng);
            w[i] = rand_value * dt.sqrt();
            // w[i] = w[i - 1] + rand_value * dt.sqrt();
        }
        w
    }
}

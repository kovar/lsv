#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod diffusion;
pub use app::TemplateApp;

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

pub mod lorenz_system {
    // implementation of the Lorenz system

    pub fn lorenz_fn(x: f64, y: f64, z: f64, sigma: f64, rho: f64, beta: f64) -> (f64, f64, f64) {
        let dx = sigma * (y - x);
        let dy = x * (rho - z) - y;
        let dz = x * y - beta * z;
        (dx, dy, dz)
    }
}

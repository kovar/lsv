#[derive(Debug, Clone, Copy)]
pub struct Lorenz {
    x: f64,
    y: f64,
    z: f64,
    sigma: f64,
    rho: f64,
    beta: f64,
}

impl Lorenz {
    pub fn new(x: f64, y: f64, z: f64, sigma: f64, rho: f64, beta: f64) -> Self {
        Self {
            x,
            y,
            z,
            sigma,
            rho,
            beta,
        }
    }
    pub fn step(&mut self, dt: f64) {
        let x = self.x + dt * self.sigma * (self.y - self.x);
        let y = self.y + dt * (self.x * (self.rho - self.z) - self.y);
        let z = self.z + dt * (self.x * self.y - self.beta * self.z);
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

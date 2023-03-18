#[derive(Debug, Clone)]
pub struct Lorenz {
    x: Vec<f64>,
    y: Vec<f64>,
    z: Vec<f64>,
    sigma: f64,
    rho: f64,
    beta: f64,
}

impl Lorenz {
    pub fn new(x: Vec<f64>, y: Vec<f64>, z: Vec<f64>, sigma: f64, rho: f64, beta: f64) -> Self {
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
        let x = self.x.last().unwrap();
        let y = self.y.last().unwrap();
        let z = self.z.last().unwrap();
        let dx = self.sigma * (y - x);
        let dy = x * (self.rho - z) - y;
        let dz = x * y - self.beta * z;
        self.x.push(x + dx * dt);
        self.y.push(y + dy * dt);
        self.z.push(z + dz * dt);
    }
}

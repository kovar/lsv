#[derive(Debug, Clone)]
pub struct Lorenz {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub t: f64,
    pub dt: f64,
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64,
}

impl Default for Lorenz {
    fn default() -> Self {
        Self {
            x: vec![1.0],
            y: vec![1.0],
            z: vec![1.0],
            t: 10.0,
            dt: 0.01,
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }
}
    
impl Lorenz {
    pub fn new(
        x: Vec<f64>,
        y: Vec<f64>,
        z: Vec<f64>,
        t: f64,
        dt: f64,
        sigma: f64,
        rho: f64,
        beta: f64,
    ) -> Self {
        Self {
            x,
            y,
            z,
            t,
            dt,
            sigma,
            rho,
            beta,
        }
    }

    pub fn step(&mut self) {
        let x = self.x.last().unwrap();
        let y = self.y.last().unwrap();
        let z = self.z.last().unwrap();
        let dx = self.sigma * (y - x);
        let dy = x * (self.rho - z) - y;
        let dz = x * y - self.beta * z;
        self.x.push(x + dx * self.dt);
        self.y.push(y + dy * self.dt);
        self.z.push(z + dz * self.dt);
    }

    pub fn solve(&mut self) {
        while self.t > 0.0 {
            self.step();
            self.t -= self.dt;
        }
    }
}

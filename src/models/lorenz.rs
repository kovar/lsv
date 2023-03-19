#[derive(Debug, Clone)]
pub struct Lorenz {
    x: Vec<f64>,
    y: Vec<f64>,
    z: Vec<f64>,
    t: f64,
    dt: f64,
    sigma: f64,
    rho: f64,
    beta: f64,
}

impl Default for Lorenz {
    fn default() -> Self {
        Self {
            x: vec![0.0],
            y: vec![0.0],
            z: vec![0.0],
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

    pub fn get_x(&self) -> &Vec<f64> {
        &self.x
    }

    pub fn get_y(&self) -> &Vec<f64> {
        &self.y
    }

    pub fn get_z(&self) -> &Vec<f64> {
        &self.z
    }

    pub fn get_t(&self) -> &f64 {
        &self.t
    }

    pub fn get_dt(&self) -> &f64 {
        &self.dt
    }

    pub fn get_sigma(&self) -> &f64 {
        &self.sigma
    }

    pub fn get_rho(&self) -> &f64 {
        &self.rho
    }

    pub fn get_beta(&self) -> &f64 {
        &self.beta
    }
}

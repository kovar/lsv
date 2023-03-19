#[derive(Debug, Clone)]
pub struct Lorenz {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub x_0: f64,
    pub y_0: f64,
    pub z_0: f64,
    pub t_max: f64, // assuming t_0 = 0
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
            x_0: 1.0,
            y_0: 1.0,
            z_0: 1.0,
            t_max: 10.0,
            dt: 0.01,
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }
}

impl Lorenz {
    pub fn new(
        x_0: f64,
        y_0: f64,
        z_0: f64,
        t_max: f64,
        dt: f64,
        sigma: f64,
        rho: f64,
        beta: f64,
    ) -> Self {
        let x = vec![x_0];
        let y = vec![y_0];
        let z = vec![z_0];

        Self {
            x,
            y,
            z,
            x_0,
            y_0,
            z_0,
            t_max,
            dt,
            sigma,
            rho,
            beta,
        }
    }

    pub fn step(&mut self) {
        let x_new = self.x.last().unwrap()
            + self.dt * self.sigma * (self.y.last().unwrap() - self.x.last().unwrap());
        let y_new = self.y.last().unwrap()
            + self.dt
                * (self.x.last().unwrap() * (self.rho - self.z.last().unwrap())
                    - self.y.last().unwrap());
        let z_new = self.z.last().unwrap()
            + self.dt
                * (self.x.last().unwrap() * self.y.last().unwrap()
                    - self.beta * self.z.last().unwrap());

        self.x.push(x_new);
        self.y.push(y_new);
        self.z.push(z_new);
    }

    pub fn solve(&mut self) {
        let n_steps = (self.t_max / self.dt) as usize;
        for _ in 0..n_steps {
            self.step();
        }
    }
}

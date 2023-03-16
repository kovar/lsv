use rand::distributions::Standard;
use rand::prelude::*;
use rand_distr::StandardNormal;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::mem;

const MU: f64 = 0.5; // Growth Rate
const SIGMA: f64 = 0.25; // Magnitude of Diffusion/Brownian Motion
const LAMBDA: f64 = 0.5; // Intensity of jump process, jumps become more frequent as this gets bigger
const BETA: f64 = 0.5; // Magnitude of the Jump Term

const END_TIME: f64 = 10.0; // Starts at t = 0.0, ends here

const N_SAMPLES: usize = 1; // number of samples to generate
const STEPSIZE: f64 = 0.02; // stepsize

// number of steps the solver will take
const N_ITERS: usize = (END_TIME / STEPSIZE) as usize;

// estimate the size of the buffer to store soltion
const BUF_CAPACITY: usize = N_ITERS * N_SAMPLES * mem::size_of::<f64>();

// output file
const OUTPUT_FILE: &str = "output.txt";


// equal chance of returning -1 or 1
pub fn plus_or_minus() -> f64 {
    let is_positive: bool = StdRng::from_entropy().sample(Standard);

    if is_positive {
        1.0
    } else {
        -1.0
    }
}

// simulates whether a poisson process will jump within STEPSIZE
fn will_jump() -> bool {
    let uniform_variate: f64 = StdRng::from_entropy().sample(Standard);
    uniform_variate <= LAMBDA * STEPSIZE
}

fn main() {
    let f = File::create(OUTPUT_FILE).unwrap();

    let mut buffer = BufWriter::with_capacity(BUF_CAPACITY, f);

    for _j in 0..N_SAMPLES {
        let mut x: f64 = 0.1;
        for i in 0..N_ITERS {
            write!(&mut buffer, "{} ", x).unwrap();
            let _t: f64 = STEPSIZE * (i as f64);

            // Deterministic term
            let dx = STEPSIZE * MU * x;

            // Diffusion/Brownian Motion term
            let z: f64 = StdRng::from_entropy().sample(StandardNormal);
            let d_w = STEPSIZE.sqrt() * x * SIGMA * z;

            // Jump Process term
            let d_j = if will_jump() {
                BETA * plus_or_minus() * x
            } else {
                0.0
            };

            // update the value of X(t)
            x += dx + d_w + d_j;
        }
        writeln!(&mut buffer, "{} ", x).unwrap();
    }
}

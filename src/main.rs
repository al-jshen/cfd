use ndarray::prelude::*;

fn main() {
    let (nx, dx, nt, dt, c) = initialize_vars();
    let mut ics = initialize_ics(nx, dx);
    println!("{:?}", &ics);
    ics = forward_integrate(nt, ics, c, dt, dx);
    println!("{:?}", &ics);
}

/// Returns a tuple of five numbers:
///     nx: number of grid points within the spatial domain
///     dx: distance between any two adjacent grid points
///     nt: number of timesteps to integrate over
///     dt: amount of time each timestep covers
///     c: wavespeed
fn initialize_vars() -> (usize, f64, usize, f64, f64) {
    let nx = 41;
    let spatial_domain = (0., 2.);
    let dx = (spatial_domain.1 - spatial_domain.0) / (nx - 1) as f64;
    let nt = 25;
    let dt = 0.025;
    let c = 1.;
    (nx, dx, nt, dt, c)
}

fn initialize_ics(nx: usize, dx: f64) -> Array1<f64> {
    let mut u = Array1::<f64>::ones(nx);
    let (lower, upper) = ((0.5 / dx) as isize, (1. / dx) as isize);
    u.slice_mut(s![lower..=upper]).fill(2.);
    u
}

fn forward_integrate(steps: usize, mut ics: Array1<f64>, c: f64, dt: f64, dx: f64) -> Array1<f64> {
    for n in 0..steps {
        let un = ics.clone();
        for i in 1..ics.len() {
            ics[i] = un[i] - c * dt / dx * (un[i] - un[i - 1]);
        }
    }
    ics
}

use bigbang::{CalculateCollisions, GravTree};
mod common;
use common::*;

const NUM_BODIES: usize = 100;
const NUM_STEPS: usize = 10;

fn main() {
    let mf: StandardMass = StandardMass::new(1.5e8, 1.41, 2e30);
    let mut particle_buf = vec![];
    particle_buf.push(Entity {
        x: 0.,
        y: 0.,
        z: 0.,
        vx: 0.,
        vy: 0.,
        vz: 0.,
        // For Dr. Lewis to check: It looks like you're assuming radius == mass in your code. Is
        // this correct? If so, I will just make them equivalent here.
        mass: 1.0 / 2150.032,
        radius: 1.0 / 2150.032,
    });
    let mut m = mf.operator(particle_buf[0].radius);
    let annulus_width = 5. / NUM_BODIES as f64;
    let inner_radius = 0.1;

    for i in 0..NUM_BODIES {
        let d = inner_radius + (i as f64) * annulus_width;
        let theta = fastrand::f64() * 6.28;
        let v = f64::sqrt(m / d);
        particle_buf.push(Entity {
            x: d * f64::cos(theta),
            y: d * f64::sin(theta),
            z: 0.,
            vx: -v * f64::sin(theta),
            vy: v * f64::cos(theta),
            vz: 0.,
            mass: 1e-6,
            radius: 1e-6,
        });
        m += mf.operator(particle_buf[i + 1].radius);
    }

    println!("Starting simulation with {} particles.", particle_buf.len());

    let mut tree = GravTree::new(
        &mut particle_buf,
        /* time_step */
        0.2,
        /* max_entities (per leaf) */
        3,
        /* theta */
        0.2,
        CalculateCollisions::No,
    );

    for i in 0..NUM_STEPS {
        println!("Step {}", i);
        tree = tree.time_step();
    }
}

struct StandardMass {
    rho_fact: f64,
}

impl StandardMass {
    fn new(r0: f64, density_pcm_3: f64, central_mass: f64) -> Self {
        let r0 = r0 * 1000.; // km to m conversion
        Self {
            rho_fact: 1.33333 * 3.14159 * r0 * r0 * r0 * 1e3 * density_pcm_3 / central_mass,
        }
    }

    fn operator(&self, r: f64) -> f64 {
        r * r * r * self.rho_fact
    }
}

use bigbang::{CalculateCollisions, GravTree};
mod common;
use common::*;

const NUM_BODIES: usize = 100;
const NUM_STEPS: usize = 10;

fn main() {
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
        mass: 1.0 / 215.032,
        radius: 1.0 / 215.032,
    });

    for i in 0..NUM_BODIES {
        let d = 0.1 + ((i * 5 / NUM_BODIES) as f64);
        let v = f64::sqrt(1.0 / d);
        particle_buf.push(Entity {
            x: d,
            y: 0.,
            z: 0.,
            vx: v,
            vy: 0.,
            vz: 0.,
            mass: 1e-14,
            radius: 1e-14,
        });
        particle_buf.push(Entity {
            x: -d,
            y: 0.,
            z: 0.,
            vx: -v,
            vy: 0.,
            vz: 0.,
            mass: 1e-14,
            radius: 1e-14,
        });
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

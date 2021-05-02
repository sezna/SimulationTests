//! This module contains code common to both simulations.

use bigbang::{AsEntity, Responsive, SimulationResult};
#[derive(Clone, PartialEq)]
pub struct Entity {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub radius: f64,
    pub mass: f64,
}

impl AsEntity for Entity {
    fn as_entity(&self) -> bigbang::Entity {
        bigbang::Entity {
            x: self.x,
            y: self.y,
            z: self.z,
            vx: self.vx,
            vy: self.vy,
            vz: self.vz,
            radius: self.radius,
            mass: self.mass,
        }
    }
}

impl Responsive for Entity {
    fn respond(&self, simulation_result: SimulationResult<Entity>, time_step: f64) -> Self {
        let (ax, ay, az) = simulation_result.gravitational_acceleration;
        let (x, y, z) = (self.x, self.y, self.z);
        let (mut vx, mut vy, mut vz) = (self.vx, self.vy, self.vz);
        /* Uncomment below to add collision detection */
        /*
        // calculate the collisions
        for other in simulation_result.collisions.clone() {
            let other_mass = if other.radius < 1. { 0.5 } else { 105. };
            let mass_coefficient_v1 = (self_mass - other_mass) / (self_mass + other_mass);
            let mass_coefficient_v2 = (2f64 * other_mass) / (self_mass + other_mass);
            vx = (mass_coefficient_v1 * vx) + (mass_coefficient_v2 * other.vx);
            vy = (mass_coefficient_v1 * vy) + (mass_coefficient_v2 * other.vy);
            vz = (mass_coefficient_v1 * vz) + (mass_coefficient_v2 * other.vz);
        }
        */
        vx += ax * time_step;
        vy += ay * time_step;
        vz += az * time_step;
        Entity {
            vx,
            vy,
            vz,
            x: x + (vx * time_step),
            y: y + (vy * time_step),
            z: z + (vz * time_step),
            radius: self.radius,
            mass: self.mass,
        }
    }
}

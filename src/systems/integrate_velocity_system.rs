use crate::integrate_velocity::*;

impl IntegrateVelocitySystem {
    pub fn update(&mut self, time: hale::Time, e: MainFamily) {
        e.velocity.target_position = e.position.position + e.velocity.velocity * time;
    }
}

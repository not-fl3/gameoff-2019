use crate::movement::*;

impl MovementSystem {
    pub fn update(&mut self, _: hale::Time, e: MainFamily) {
        e.position.position = e.velocity.target_position;
    }
}

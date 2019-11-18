use crate::{smoke::*, *};

impl SmokeSystem {
    pub fn update(&mut self, t: hale::Time, e: MainFamily) {
        e.position.position += e.smoke.speed * t;
    }
}

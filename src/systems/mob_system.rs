use crate::mob::*;

impl MobSystem {
    pub fn update(&mut self, _: hale::Time, entity: MainFamily) {
        let max_speed = entity.mob.max_speed;
        let acceleration = entity.mob.accel;
        let mut in_vec = entity.mob.move_dir;
        if in_vec.squared_length() > 1. {
            in_vec = in_vec.unit();
        }
        let vel = &mut entity.velocity.velocity;

        let brake = vel.unit() * acceleration * 0.5;
        *vel += in_vec * acceleration;
        if brake.squared_length() > vel.squared_length() {
            *vel = hale::Vector2::new(0., 0.);
        } else {
            *vel -= brake;
        }
        *vel *= if in_vec.length() > 0.1 { 0.99 } else { 0.80 };
        let len2 = vel.squared_length();
        if len2 < 10.0 {
            *vel = hale::Vector2::new(0., 0.);
        } else if len2 > max_speed * max_speed {
            *vel = *vel * (max_speed / len2.sqrt());
        }
    }
}

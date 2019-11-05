use crate::{flashing::*, Damage};

impl FlashingSystem {
    pub fn update(&mut self, time: hale::Time, e: MainFamily) {
        if e.flashing.active {
            if e.flashing.cur_time > e.flashing.total_time {
                e.flashing.active = false;
            }
            let t = 1.0 - (e.flashing.cur_time / e.flashing.total_time);
            e.flashing.cur_time += time;
            e.sprite
                .sprite
                .set_color(hale::Color::from_rgba(1., 1., 1., 1.).lerp(e.flashing.colour, t));
        } else {
            e.sprite
                .sprite
                .set_color(hale::Color::from_rgba(1., 1., 1., 1.));
        }
    }

    pub fn on_message_received(&mut self, entity: MainFamily, _: &Damage) {
        entity.flashing.colour = hale::Color::from_rgba(1.0, 0.0, 0.0, 1.0);
        entity.flashing.cur_time = 0.0;
        entity.flashing.total_time = 0.1;
        entity.flashing.active = true;
    }
}

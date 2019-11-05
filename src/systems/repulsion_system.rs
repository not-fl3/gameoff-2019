use crate::{repulsion::*, Collision, *};

impl RepulsionSystem {
    pub fn update(&mut self, _: hale::Time, _: MainFamily) {}

    pub fn on_message_received(&mut self, entity: MainFamily, msg: &Collision) {
        // Only bounce against other things with repulsion
        if self
            .get_world()
            .get_entity(msg.other)
            .has_component::<RepulseField>()
            == false
        {
            return;
        }

        let my_rect = entity.collider.rect + entity.position.position;
        let their_rect = msg.other_rect;

        // Find the bounce-out direction
        let c0 = my_rect.get_center();
        let c1 = their_rect.get_center();
        let mut dir = (c0 - c1).unit();

        // If they're too close, it might fail. Disambiguate
        if dir.squared_length() < 0.1 {
            dir.x = if entity.entity_id < msg.other {
                -1.0
            } else {
                1.0
            };
            dir.y = dir.x;
            dir.normalize();
        }

        // Get the overlap amount
        let mut overlap = hale::Vector2::new(0., 0.);
        overlap.x = my_rect
            .get_horizontal()
            .get_overlap(their_rect.get_horizontal())
            .get_length();
        overlap.y = my_rect
            .get_vertical()
            .get_overlap(their_rect.get_vertical())
            .get_length();
        let axes = hale::Vector2::new(dir.x.abs(), dir.y.abs());

        // // Compute speed and bounce
        let overlap_dist = axes.dot(overlap) + 2.;
        let speed = overlap_dist * entity.repulse_field.multiplier;
        entity.velocity.velocity += dir * speed;
    }
}

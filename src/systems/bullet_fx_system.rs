use crate::{bullet_fx::*, *};

impl BulletFxSystem {
    pub fn update(&mut self, _: hale::Time, _: MainFamily) {}

    pub fn on_message_received(&mut self, entity: MainFamily, msg: &Collision) {
        if msg.other_layer == 1 {
            let my_rect = entity.collider.rect + entity.position.position;

            let mut other = self
                .get_world()
                .get_entity(msg.other);
            let other_position = other.get_component::<Position>().position;
            let other_collider_rect = other
                .get_component::<Collider>()
                .rect;
            let other_rect = other_collider_rect + other_position;

            let x = my_rect.get_range(0).get_overlap(other_rect.get_range(0));

            let x = x.start;
            let y = my_rect.get_range(1).get_overlap(other_rect.get_range(1));
            let y = y.start;

            create_fire(self.get_world(), self.get_api(), hale::Point2::new(x, y));
        }
    }
}


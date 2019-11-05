use crate::{damage::*, Damage, Death};
use hale::MessageSender;

impl DamageSystem {
    pub fn update(&mut self, _: hale::Time, _: MainFamily) {}

    pub fn on_message_received(&mut self, entity: MainFamily, msg: &Damage) {
        let health = &mut entity.health.current;

        *health -= msg.amount;

        if *health <= 0 {
            self.send_message(entity.entity_id, Death {});
            *health = 0;
            self.get_world().destroy_entity(entity.entity_id);
        }
    }
}

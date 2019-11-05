use crate::{bullet::*, Collision, Damage};
use hale::MessageSender;

impl BulletSystem {
    pub fn update(&mut self, _: hale::Time, _: MainFamily) {}

    pub fn on_message_received(&mut self, entity: MainFamily, msg: &Collision) {
        if msg.other_layer == 1 {
            self.get_world().destroy_entity(entity.entity_id);
        }

        // Hit enemy
        if msg.other_layer == 1 {
            self.send_message(
                msg.other,
                Damage {
                    amount: entity.bullet.damage,
                },
            );
            //auto clip = getAPI().getResource<AudioClip>("sound/hit.ogg");
            //getAPI().audio->play(clip, AudioSourcePosition::makePositional(entity.position.position));
        }
    }
}

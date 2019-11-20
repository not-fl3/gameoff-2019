use crate::{enemy_explosion::*, *};
use hale::{FamilyContainer, MessageSender};
use std::cmp::Ordering;

impl EnemyExplosionSystem {
    pub fn update(&mut self, _: hale::Time, e: MainFamily, bases: &FamilyContainer<BasesFamily>) {
        let closest_base = bases.iter().min_by(|a, b| {
            let a = (a.position.position - e.position.position).squared_length();
            let b = (b.position.position - e.position.position).squared_length();
            if a > b { Ordering::Greater } else { Ordering::Less }
        });

        if let Some(closest_base) = closest_base {
            if closest_base.position.position.distance(e.position.position) < 30.0 {
                let mut entity = self.get_world().get_entity(e.entity_id);

                entity.remove_component::<Enemy>();
                entity.remove_component::<Mob>();
                entity.remove_component::<Velocity>();
                entity.remove_component::<Collider>();

                entity.add_component(TTL {
                    time_left: 0.5
                });

                entity.get_component::<SpriteAnimation>().player.set_sequence("explode");

                self.send_message(
                    closest_base.entity_id,
                    Damage {
                        amount: 1,
                    },
                );
            }
        }
    }
}

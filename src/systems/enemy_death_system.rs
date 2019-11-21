use crate::{enemy_death::*, Death, *};

impl EnemyDeathSystem {
    pub fn update(&mut self, _: hale::Time, _: MainFamily) {}

    pub fn on_message_received(&mut self, entity: MainFamily, _msg: &Death) {
        create_coin_spawner(self.get_world(), entity.position.position);
        //String clip = Random::getGlobal().getInt(0, 1) == 0 ? "sound/kill1.ogg" : "sound/kill2.ogg";
        //getAPI().audio->play(getAPI().getResource<AudioClip>(clip), AudioSourcePosition::makePositional(entity.position.position));
    }
}

fn create_coin_spawner(world: &mut World, position: hale::Point2) {
    world
        .create_entity()
        .add_component(CoinSpawner {
            coins: vec![]
        })
        .add_component(Position {
            position
        })
        .add_component(TTL {
            time_left: 0.5
        });
}

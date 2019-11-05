use crate::{enemy_death::*, Death};

impl EnemyDeathSystem {
    pub fn update(&mut self, _: hale::Time, _: MainFamily) {}

    pub fn on_message_received(&mut self, _entity: MainFamily, _msg: &Death) {
        //String clip = Random::getGlobal().getInt(0, 1) == 0 ? "sound/kill1.ogg" : "sound/kill2.ogg";
        //getAPI().audio->play(getAPI().getResource<AudioClip>(clip), AudioSourcePosition::makePositional(entity.position.position));
    }
}

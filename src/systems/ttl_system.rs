use crate::ttl::*;

impl TTLSystem {
    pub fn update(&mut self, delta: hale::Time, entity: MainFamily) {
        entity.ttl.time_left -= delta;
        if entity.ttl.time_left <= 0.0 {
            self.get_world().destroy_entity(entity.entity_id);
        }
    }
}

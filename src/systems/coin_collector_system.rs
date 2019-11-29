use crate::{coin_collector::*, *};
use hale::FamilyContainer;

impl CoinCollectorSystem {
    pub fn update(&mut self, _: hale::Time, e: MainFamily, player: &FamilyContainer<PlayerFamily>) {
        let player = player.iter().next().unwrap();

        if player.position.position.distance(e.position.position) < 15. {
            let world = self.get_world();

            let mut coin = world.get_entity(e.entity_id);
            coin.remove_component::<Coin>();
            coin.add_component(FlyAway {target: hale::Point2::new(hale::rand::gen_range(-300.0, 300.0), -350.0)});
            coin.add_component(TTL {time_left: 10.0});
        }
    }
}

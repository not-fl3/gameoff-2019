use crate::{coin_spawner::*, *};
use hale::FamilyContainer;

impl CoinSpawnerSystem {
    pub fn update(&mut self, delta: hale::Time, e: MainFamily, coins_family: &FamilyContainer<CoinsFamily>) {
        for (ref mut coin, ref mut dir) in e.coin_spawner.coins.iter_mut() {
            if let Some(coin) = coins_family.try_get_by_id(*coin) {
                coin.position.position += *dir * delta;

                *dir += hale::Vector2::new(0.0, 15.);
            }
        }
    }

    pub fn on_entity_add(&mut self, entity: &mut MainFamily, _: bool) {
        for _ in 1 .. hale::rand::gen_range(2, 5) {
            let coin = create_coin(self.get_world(), self.get_api(), entity.position.position);
            let dir = hale::Vector2::new(hale::rand::gen_range(-40., 40.), hale::rand::gen_range(-200., -300.));
            entity.coin_spawner.coins.push((coin, dir));
        }
    }
}

fn create_coin(world: &mut World, api: &mut api::Api, position: hale::Point2) -> hale::EntityId {
    world
        .create_entity()
        .add_component(Sprite {
            sprite: hale::api::Sprite::new().with_scale(hale::Vector2::new(0.8, 0.8)),            
            layer: -2
        })
        .add_component(SpriteAnimation {
            player: hale::AnimationPlayer::new(
                api.get_resource::<hale::api::Animation>("Coin"),
                "default",
                "default",
            ),
            random_first_frame: true
        })
        .add_component(Coin {
        })
        .add_component(Position {
            position
        })
        .uid()
}

use crate::sprite_animation::*;

impl SpriteAnimationSystem {
    pub fn update(&mut self, delta: hale::Time, e: MainFamily) {
        let player = &mut e.sprite_animation.player;

        player.update(delta);
        player.update_sprite(&mut e.sprite.sprite);
    }

    pub fn on_entity_add(&mut self, e: &mut MainFamily, _: bool) {
        if e.sprite_animation.random_first_frame {
            let player = &mut e.sprite_animation.player;

            player.update(hale::rand::gen_range(0., 1.));
            player.update_sprite(&mut e.sprite.sprite);
        }
    }
}

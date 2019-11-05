use crate::sprite_animation::*;

impl SpriteAnimationSystem {
    pub fn update(&mut self, delta: hale::Time, e: MainFamily) {
        let player = &mut e.sprite_animation.player;

        player.update(delta);
        player.update_sprite(&mut e.sprite.sprite);
    }
}

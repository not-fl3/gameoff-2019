use crate::enemy_animation::*;

impl EnemyAnimationSystem {
    pub fn update(&mut self, _: hale::Time, e: MainFamily) {
        let vel = e.mob.face_dir;
        #[rustfmt::skip]
        let dir = if vel.y.abs() > vel.x.abs() { if vel.y < 0.0 { 0 } else { 2 } } else { if vel.x < 0.0 { 3 } else { 1 }};

        let player = &mut e.sprite_animation.player;
        player.set_direction(dir);
    }
}

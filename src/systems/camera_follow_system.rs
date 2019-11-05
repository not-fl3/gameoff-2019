use hale::FamilyContainer;

use crate::camera_follow::*;

impl CameraFollowSystem {
    pub fn update(&mut self, _: hale::Time, e: MainFamily, player: &FamilyContainer<PlayerFamily>) {
        assert!(player.len() == 1);

        let player = player.iter().next().unwrap();

        e.camera.camera.target = player.position.position;
    }
}

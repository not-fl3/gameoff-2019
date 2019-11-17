use hale::FamilyContainer;

use crate::camera_follow::*;

impl CameraFollowSystem {
    pub fn update(&mut self, _: hale::Time, e: MainFamily, player: &FamilyContainer<PlayerFamily>, cursor: &FamilyContainer<CursorFamily>) {
        assert!(player.len() == 1);
        assert!(cursor.len() == 1);

        let player = player.iter().next().unwrap();
        let cursor = cursor.iter().next().unwrap();

        let delta = cursor.position.position - player.position.position;
        let len = delta.length();

        e.camera.camera.target = player.position.position + e.camera.offset;
        e.camera.offset = (delta.unit() * len.min(10.0)).to_point().lerp(e.camera.offset, 0.8);
    }
}

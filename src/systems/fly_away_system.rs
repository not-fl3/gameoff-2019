use crate::fly_away::*;
use hale::FamilyContainer;

impl FlyAwaySystem {
    pub fn update(&mut self, _: hale::Time, e: MainFamily, camera: &FamilyContainer<CameraFamily>) {
        let camera = camera.iter().next().unwrap();

        e.position.position = e.position.position.lerp(camera.camera.camera.target + e.fly_away.target, 0.01);
    }
}

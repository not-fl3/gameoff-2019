use crate::sprite_render::*;
use hale::FamilyContainer;

impl SpriteRenderSystem {
    pub fn update(&mut self, _: hale::Time, main_family: &FamilyContainer<MainFamily>, camera: &FamilyContainer<CameraFamily>) {
        assert!(camera.len() == 1);
        let camera = camera.iter().next().unwrap().camera;
        
        let api = self.get_api();
        let mut painter = api.get_painter().with_camera(&mut camera.camera);

        let mut sprites = main_family.iter().collect::<Vec<_>>();
        sprites.sort_by_key(|s| s.sprite.layer);

        for e in sprites {
            let sprite = &mut e.sprite.sprite;

            sprite.set_position(e.position.position);
            painter.draw_sprite(sprite);
        }
    }
}

use hale::FamilyContainer;
use crate::player_input::*;

impl PlayerInputSystem {
    pub fn update(&mut self, 
        _: hale::Time, 
        entity: MainFamily, 
        camera: &FamilyContainer<CameraFamily>, 
        cursor: &FamilyContainer<CursorFamily>) 
    {
        let input = &entity.player_input.input;
        let move_dir: hale::Vector2 = hale::Vector2::new(input.get_axis(0), input.get_axis(1));
        
        let face_dir = move_dir;

        entity.mob.move_dir = move_dir;

        if face_dir.squared_length() > 0.1 {
            entity.mob.face_dir = face_dir.unit();
        }

        entity.shooter.shooting = input.get_axis(5) == 1.0;

        let camera = &camera.iter().next().unwrap().camera.camera;
        let cursor = cursor.iter().next().unwrap();

        cursor.position.position = camera.screen_to_world(input.get_mouse_position());

        entity.shooter.shoot_dir = (cursor.position.position - entity.position.position).unit();

    }
}

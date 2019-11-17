use crate::health_bars_render::*;
use hale::{FamilyContainer, Rect, Color};

impl HealthBarsRenderSystem {
    pub fn update(&mut self, _: hale::Time, main_family: &FamilyContainer<MainFamily>, camera: &FamilyContainer<CameraFamily>) {
        assert!(camera.len() == 1);
        let camera = camera.iter().next().unwrap().camera;
        
        let api = self.get_api();
        let mut painter = api.get_painter().with_camera(&mut camera.camera);

        for e in main_family {
            let health = e.health.current as f32 / e.health.max as f32;
            let bar_width = 20.0;
            let black_bar_rect = Rect::new(e.position.position.x, e.position.position.y - 12.0, bar_width, 3.0);
            let red_bar_rect = Rect::new(
                e.position.position.x - (bar_width - bar_width * health) * 0.5, 
                e.position.position.y - 12.0, 
                bar_width * health, 
                3.0);

            painter.draw_quad(black_bar_rect, Color::from_rgba(0.0, 0.0, 0.0, 1.0));
            painter.draw_quad(red_bar_rect, Color::from_rgba(1.0, 0.0, 0.0, 1.0));
        }
    }
}

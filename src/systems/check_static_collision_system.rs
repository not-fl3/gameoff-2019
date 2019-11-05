use hale::{EntityId, FamilyContainer, MessageSender, Rect, Vector2};

use crate::{check_static_collision::*, Collision};

impl CheckStaticCollisionSystem {
    pub fn update(
        &mut self,
        t: hale::Time,
        e: MainFamily,
        targets: &FamilyContainer<ObstaclesFamily>,
    ) {
        let myself = e.entity_id;
        let start_pos = e.position.position;
        let end_pos = &mut e.velocity.target_position;
        let mut rect = e.position.position + e.collider.rect;

        // This is how much this entity wants to move
        let desired_delta = *end_pos - start_pos;
        let mut delta = Vector2::new(0., 0.);

        // Try moving it horizontally first...
        delta.x = self.move_horizontal(targets, myself, desired_delta.x, rect);

        // Adjust the entity's collider based on where it'd end up, and try moving vertically from there
        rect += Vector2::new(delta.x, 0.);
        delta.y = self.move_vertical(targets, myself, desired_delta.y, rect);

        // This is where we end up
        *end_pos = start_pos + delta;

        // If we hit an obstacle, update our velocity to reflect that
        if t > 0.0001 {
            if delta.x != desired_delta.x {
                e.velocity.velocity.x = delta.x / t;
            }
            if delta.y != desired_delta.y {
                e.velocity.velocity.y = delta.y / t;
            }
        }
    }

    fn move_horizontal(
        &mut self,
        obstacles_family: &FamilyContainer<ObstaclesFamily>,
        myself: EntityId,
        delta: f32,
        rect: Rect,
    ) -> f32 {
        if delta == 0. {
            0.
        } else {
            self.r#move(obstacles_family, myself, delta, 0, rect)
        }
    }

    fn move_vertical(
        &mut self,
        obstacles_family: &FamilyContainer<ObstaclesFamily>,
        myself: EntityId,
        delta: f32,
        rect: Rect,
    ) -> f32 {
        if delta == 0. {
            0.
        } else {
            self.r#move(obstacles_family, myself, delta, 1, rect)
        }
    }

    fn r#move(
        &mut self,
        obstacles_family: &FamilyContainer<ObstaclesFamily>,
        myself: EntityId,
        delta: f32,
        coord: i32,
        rect: Rect,
    ) -> f32 {
        let my_orthogonal = rect.get_range(1 - coord);
        let my_normal = rect.get_range(coord);

        let my_pos = if delta < 0. {
            rect.get_top_left()[coord as usize]
        } else {
            rect.get_bottom_right()[coord as usize]
        };
        let mut closest = if delta < 0. {
            std::f32::MIN
        } else {
            std::f32::MAX
        };

        let mut best_obstacle = None;

        // TODO: only check surrounding area, instead of all obstacles
        for obstacle in obstacles_family {
            if obstacle.collider.is_static {
                let col_rect = obstacle.collider.rect + obstacle.position.position;

                // First check if there's an orthogonal overlap. If not, no matter how much I move along this axis, I can never collide with this obstacle.
                let obs_orthogonal = col_rect.get_range(1 - coord);
                if my_orthogonal.overlaps(obs_orthogonal) {
                    // We'll also check for a normal overlap. If that's ALSO an overlap, then the two rectangles are overlapping (bad).
                    let obs_normal = col_rect.get_range(coord);
                    if my_normal.overlaps(obs_normal) {
                        // We're inside the obstacle, oh shit.
                        // TODO: handle this better
                        self.on_collided_with(myself, obstacle);
                        return 0.;
                    }

                    let obs_pos = (if delta > 0. {
                        col_rect.get_top_left()
                    } else {
                        col_rect.get_bottom_right()
                    })[coord as usize];
                    if (delta < 0. && obs_pos <= my_pos && obs_pos > closest)
                        || (delta >= 0. && obs_pos >= my_pos && obs_pos < closest)
                    {
                        // Hit this obstacle, and this is closer to any previous found ones
                        closest = obs_pos;
                        best_obstacle = Some(obstacle);
                    }
                }
            }
        }
        let delta_to_impact = closest - my_pos;

        if delta.abs() < delta_to_impact.abs() {
            // No obstacles hit
            return delta;
        } else {
            // Hit something
            if let Some(best_obstacle) = best_obstacle {
                self.on_collided_with(myself, best_obstacle);
            }
            return delta_to_impact;
        }
    }

    fn on_collided_with(&mut self, dynamic: EntityId, obstacle: ObstaclesFamily) {
        self.send_message(
            dynamic,
            Collision {
                other: obstacle.entity_id,
                other_layer: obstacle.collider.layer,
                other_rect: obstacle.collider.rect + obstacle.position.position,
            },
        );
    }
}

use crate::target_seeking::*;
use hale::FamilyContainer;
use std::cmp::Ordering;

fn closest_to_point(iter: impl Iterator<Item = hale::Point2>, point: hale::Point2) -> Option<hale::Point2> {
    iter.min_by(|a, b| {
        let a = (*a - point).squared_length();
        let b = (*b - point).squared_length();
        if a > b { Ordering::Greater } else { Ordering::Less }
    })
    
}
impl TargetSeekingSystem {
    pub fn update(
        &mut self,
        _: hale::Time,
        entity: MainFamily,
        players: &FamilyContainer<PlayersFamily>,
        bases: &FamilyContainer<BasesFamily>,
    ) {
        let my_pos = entity.position.position;
        let closest_player = closest_to_point(
            players.iter().map(|e| e.position.position), 
            my_pos);
        let closest_base = closest_to_point(
            bases.iter().map(|e| e.position.position), 
            my_pos);

        let best_target = match (closest_player, closest_base) {
            // if any players in range
            (Some(closest_player), _) if closest_player.distance(my_pos) < 60. => {
                Some(closest_player)
            },
            // if any bases still exists
            (_, Some(closest_base)) => Some(closest_base),
            // no players in range and no bases - maybe any players not in range?
            (Some(closest_player), _) => Some(closest_player),
            // no targets at all :(
            _ => None
        };

        if let Some(best_target) = best_target {
            let target = best_target - my_pos;
            entity.mob.move_dir = target.unit();
            entity.mob.face_dir = target.unit();
        } else {
            // No players to seek
            entity.mob.move_dir = hale::Vector2::new(0., 0.);
        }
    }
}

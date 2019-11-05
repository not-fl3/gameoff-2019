use hale::{FamilyContainer, MessageSender};

use crate::{check_dynamic_collision::*, Collision};

impl CheckDynamicCollisionSystem {
    pub fn update(
        &mut self,
        _: hale::Time,
        e: MainFamily,
        targets: &FamilyContainer<TargetsFamily>,
    ) {
        // Skip static objects
        if e.collider.is_static {
            return;
        }

        let my_rect = e.position.position + e.collider.rect;
        for o in targets {
            // Don't check against static objects, or against myself
            if !o.collider.is_static && o.entity_id != e.entity_id {
                let rect = o.position.position + o.collider.rect;
                if my_rect.overlaps(rect) {
                    self.send_message(
                        e.entity_id,
                        Collision {
                            other: o.entity_id,
                            other_layer: o.collider.layer,
                            other_rect: rect,
                        },
                    );
                }
            }
        }
    }
}

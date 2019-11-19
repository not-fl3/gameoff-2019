use hale::{api, World};

gen::gen!("src/hale-sample.yaml");

mod systems;

struct GameStage;

impl api::Stage for GameStage {}

pub fn create_smoke(world: &mut World, api: &mut api::Api, position: hale::Point2) {
    world
        .create_entity()
        .add_component(Sprite {
            sprite: hale::api::Sprite::new(),
            layer: 10000
        })
        .add_component(SpriteAnimation {
            player: hale::AnimationPlayer::new(
                api.get_resource::<hale::api::Animation>("Smoke"),
                "smoke",
                "default",
            ),
        })
        .add_component(Smoke {
            speed: hale::Vector2::new(hale::rand::gen_range(-20f32, 20f32), -50.)
        })
        .add_component(Position {
            position
        })
        .add_component(TTL { time_left: 0.5 });
}

pub fn create_fire(world: &mut World, api: &mut api::Api, position: hale::Point2) {
    world
        .create_entity()
        .add_component(Sprite {
            sprite: hale::api::Sprite::new(),
            layer: 10000
        })
        .add_component(SpriteAnimation {
            player: hale::AnimationPlayer::new(
                api.get_resource::<hale::api::Animation>("Smoke"),
                "fire",
                "default",
            ),
        })
        .add_component(Smoke {
            speed: hale::Vector2::new(hale::rand::gen_range(-40f32, 40f32), -50.)
        })
        .add_component(Position {
            position
        })
        .add_component(TTL { time_left: 0.25 }); 
}


fn create_player(world: &mut World, api: &mut api::Api, position: hale::Point2) {
    let weapon = world
        .create_entity()
        .add_component(Position { position })
        .add_component(Sprite {
            sprite: hale::api::Sprite::new(),
            layer: 0,
        })
        .add_component(SpriteAnimation {
            player: hale::AnimationPlayer::new(
                api.get_resource::<hale::api::Animation>("Flamethrower"),
                "default",
                "default",
            ),
        })
        .add_component(Gun {
            cooldown: 0.01,
            kind: "flamethrower".to_string(),
            muzzle: hale::Point2::new(0., 0.)
        })        
        .uid();

    world
        .create_entity()
        .add_component(Position { position })
        .add_component(Velocity {
            velocity: hale::Vector2::new(0.0, 0.0),
            target_position: hale::Point2::new(0.0, 0.0),
        })
        .add_component(SpriteAnimation {
            player: hale::AnimationPlayer::new(
                api.get_resource::<hale::api::Animation>("Player"),
                "idle",
                "default",
            ),
        })
        .add_component(Sprite {
            sprite: hale::api::Sprite::new(),
            layer: 0,
        })
        .add_component(Mob {
            move_dir: hale::Vector2::new(0.0, 0.0),
            face_dir: hale::Vector2::new(0.0, 0.0),
            accel: 50.,
            max_speed: 150.,
        })
        .add_component(PlayerInput {
            input: api.get_virtual_input(),
        })
        .add_component(Player {})
        .add_component(Shooter {
            shooting: false,
            shoot_dir: hale::Vector2::new(0., 0.),
            cooldown: 0.0,
            smokes: 0,
            weapon
        })
        .add_component(Collider {
            rect: hale::Rect::new(-13.0, -13.0, 26.0, 26.0),
            layer: 0,
            trigger: false,
            is_static: false,
        })
        .add_component(RepulseField { multiplier: 10.0 });
}

fn create_cursor(world: &mut World, api: &mut api::Api) {
    world
        .create_entity()
        .add_component(Cursor {})
        .add_component(Position { position: hale::Point2::new(0., 0.) })
        .add_component(Sprite {
            sprite: hale::api::Sprite::new()
                .with_spritesheet(api.resources(), 
                    "spritesheet.json", 
                    "7crosshair.png")
                .with_pivot(hale::Vector2::new(0.5, 0.5)),
            layer: 0,
        });
}

fn create_camera(world: &mut World, target: hale::Point2) {
    world
        .create_entity()
        .add_component(Camera {
            camera: hale::Camera::new(target, hale::Vector2::new(0.3, 0.3)),
            offset: hale::Point2::new(0., 0.)
        });
}

fn create_room(world: &mut World, api: &mut api::Api, pos: hale::Point2, id: hale::EntityId) {
    world
        .create_entity()
        .add_component(Position {
            position: pos + hale::Vector2::new(350.0, 350.0),
        })
        .add_component(Sprite {
            sprite: hale::api::Sprite::new()
                .with_spritesheet(
                    api.resources(),
                    "spritesheet.json",
                    "BG_0.png",
                )
                .with_pivot(hale::Vector2::new(0.5, 0.5)),
            layer: -20,
        });

    let x0 = 125.0;
    let x1 = 565.0;
    let y0 = 220.0;
    let y1 = 605.0;
    let spawners = vec![
        hale::Vector2::new(x1, y1),
        hale::Vector2::new(x0, y1),
        hale::Vector2::new(x1, y0),
        hale::Vector2::new(x0, y0),
    ];

    for spawner_pos in spawners {
        world
            .create_entity()
            .add_component(Position {
                position: pos + spawner_pos,
            })
            .add_component(EnemySpawner {
                cooldown: hale::rand::gen_range(2.0, 4.0),
                room_id: id as i32,
            });
    }
}

fn main() {
    api::start(GameStage, |world, api| {
        register_systems(world, api);

        create_player(world, api, hale::Point2::new(400.0, 350.0));
        create_cursor(world, api);
        create_camera(world, hale::Point2::new(400.0, 350.0));
        create_room(world, api, hale::Point2::new(0.0, 0.0), 0);
    });
}

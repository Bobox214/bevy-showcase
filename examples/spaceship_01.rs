use bevy::{
    prelude::*,
    render::{camera::OrthographicProjection, pass::ClearColor},
};
use bevy_rapier2d::{
    na::Vector2,
    physics::{Gravity, RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
    render::RapierRenderPlugin,
};

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 800;
const CAMERA_SCALE: f32 = 0.1;
const ARENA_WIDTH: f32 = WINDOW_WIDTH as f32 * CAMERA_SCALE;
const ARENA_HEIGHT: f32 = WINDOW_HEIGHT as f32 * CAMERA_SCALE;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Spaceship 01".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.02, 0.02, 0.04)))
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_default_plugins()
        .add_resource(Gravity(Vector2::zeros()))
        .add_startup_system(setup.system())
        .add_system(position_system.system())
        .add_system(user_input_system.system())
        .add_system(player_dampening_system.system())
        .run();
}

struct Player(Entity);

struct Ship {
    /// Ship rotation speed in rad/s
    rotation_speed: f32,
    /// Ship thrust N
    thrust: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dComponents {
        orthographic_projection: OrthographicProjection {
            far: 1000.0 / CAMERA_SCALE,
            ..Default::default()
        },
        transform: Transform::from_scale(CAMERA_SCALE),
        ..Default::default()
    });
    let texture_handle = asset_server.load("assets/spaceship.png").unwrap();
    let body = RigidBodyBuilder::new_dynamic();
    let collider = ColliderBuilder::ball(1.0);
    commands
        .spawn(SpriteComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
                .with_scale(1.0 / 150.0),
            material: materials.add(texture_handle.into()),
            ..Default::default()
        })
        .with(Ship {
            rotation_speed: 10.0,
            thrust: 30.0,
        })
        .with(body)
        .with(collider);
    let player_entity = commands.current_entity().unwrap();
    commands.insert_resource(Player(player_entity));

    //let texture_handle = asset_server
    //    .load("assets/sprite_sphere_256x256.png")
    //    .unwrap();
    //let body = RigidBodyBuilder::new_static().translation(200.0, 200.0);
    //let collider = ColliderBuilder::ball(256.0);
    //commands
    //    .spawn(SpriteComponents {
    //        //translation: Translation::new(200.0, 200.0, 0.0),
    //        material: materials.add(texture_handle.into()),
    //        scale: Scale(1.0),
    //        ..Default::default()
    //    })
    //    .with(body)
    //    .with(collider);
}

fn position_system(mut bodies: ResMut<RigidBodySet>, mut query: Query<&RigidBodyHandleComponent>) {
    for body_handle in &mut query.iter() {
        let mut body = bodies.get_mut(body_handle.handle()).unwrap();
        let mut x = body.position.translation.vector.x;
        let mut y = body.position.translation.vector.y;
        let mut updated = false;
        // Wrap around screen edges
        let half_width = ARENA_WIDTH / 2.0;
        let half_height = ARENA_HEIGHT / 2.0;
        if x < -half_width && body.linvel.x < 0.0 {
            x = half_width;
            updated = true;
        } else if x > half_width && body.linvel.x > 0.0 {
            x = -half_width;
            updated = true;
        }
        if y < -half_height && body.linvel.y < 0.0 {
            y = half_height;
            updated = true;
        } else if y > half_height && body.linvel.y > 0.0 {
            y = -half_height;
            updated = true;
        }
        if updated {
            let mut new_position = body.position.clone();
            new_position.translation.vector.x = x;
            new_position.translation.vector.y = y;
            body.set_position(new_position);
        }
    }
}
fn player_dampening_system(
    time: Res<Time>,
    player: Res<Player>,
    mut bodies: ResMut<RigidBodySet>,
    query: Query<&RigidBodyHandleComponent>,
) {
    let elapsed = time.delta_seconds;
    let body_handle = query.get::<RigidBodyHandleComponent>(player.0).unwrap();
    let mut body = bodies.get_mut(body_handle.handle()).unwrap();
    body.angvel = body.angvel * 0.1f32.powf(elapsed);
    body.linvel = body.linvel * 0.8f32.powf(elapsed);
}

fn user_input_system(
    input: Res<Input<KeyCode>>,
    player: Res<Player>,
    mut bodies: ResMut<RigidBodySet>,
    query: Query<(&RigidBodyHandleComponent, &Ship)>,
) {
    let mut rotation = 0;
    let mut thrust = 0;
    if input.pressed(KeyCode::W) {
        thrust += 1
    }
    if input.pressed(KeyCode::S) {
        thrust -= 1
    }
    if input.pressed(KeyCode::A) {
        rotation += 1
    }
    if input.pressed(KeyCode::D) {
        rotation -= 1
    }
    if rotation != 0 || thrust != 0 {
        let body_handle = query.get::<RigidBodyHandleComponent>(player.0).unwrap();
        let mut body = bodies.get_mut(body_handle.handle()).unwrap();
        let ship = query.get::<Ship>(player.0).unwrap();
        //println!(
        //    "Body world_inv_inertia_sqrt {:?}",
        //    body.world_inv_inertia_sqrt
        //);
        //println!(
        //    "Body mass_properties.inv_mass {:?}",
        //    body.mass_properties.inv_mass
        //);
        if rotation != 0 {
            let rotation = rotation as f32 * ship.rotation_speed;
            body.wake_up();
            body.apply_torque(rotation);
        }
        if thrust != 0 {
            let force = body.position.rotation.transform_vector(&Vector2::y())
                * thrust as f32
                * ship.thrust;
            body.wake_up();
            body.apply_force(force);
        }
    }
}

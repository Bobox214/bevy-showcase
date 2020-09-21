use bevy::{
    prelude::*,
    render::{
        camera::{OrthographicProjection, WindowOrigin},
        pass::ClearColor,
    },
};
use bevy_rapier2d::{
    na::Vector2,
    physics::{EventQueue, Gravity, RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};
use rand::prelude::*;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 800;

fn main() {
    App::build()
        .init_resource::<MousePosition>()
        .add_resource(Msaa { samples: 2 })
        .add_resource(WindowDescriptor {
            title: "Rapier2D Bevy showcase".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.01, 0.01, 0.03)))
        .add_plugin(RapierPhysicsPlugin)
        .add_default_plugins()
        .add_resource(Gravity(Vector2::zeros()))
        .add_startup_system(setup.system())
        .add_system(mouse_position_system.system())
        .add_system(spawn_sphere_system.system())
        .add_system(position_system.system())
        .add_system_to_stage(stage::POST_UPDATE, collision_system.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents {
        orthographic_projection: OrthographicProjection {
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn position_system(mut bodies: ResMut<RigidBodySet>, mut query: Query<&RigidBodyHandleComponent>) {
    for body_handle in &mut query.iter() {
        let mut body = bodies.get_mut(body_handle.handle()).unwrap();
        let mut x = body.position.translation.vector.x;
        let mut y = body.position.translation.vector.y;
        let mut updated = false;
        // Wrap around screen edges
        if x < 0.0 && body.linvel.x < 0.0 {
            x = WINDOW_WIDTH as f32;
            updated = true;
        } else if x > WINDOW_WIDTH as f32 && body.linvel.x > 0.0 {
            x = 0.0;
            updated = true;
        }
        if y < 0.0 && body.linvel.y < 0.0 {
            y = WINDOW_HEIGHT as f32;
            updated = true;
        } else if y > WINDOW_HEIGHT as f32 && body.linvel.y > 0.0 {
            y = 0.0;
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

fn collision_system(events: Res<EventQueue>) {
    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Contact event {:?}", contact_event);
    }
    while let Ok(proximity_event) = events.proximity_events.pop() {
        println!("Received proximity event: {:?}", proximity_event);
    }
}

fn spawn_sphere_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut rng = thread_rng();
        let x = mouse_position.0.x();
        let y = mouse_position.0.y();
        let z = rng.gen_range(0.0, 1.0);
        let vx = rng.gen_range(-(WINDOW_WIDTH as f32) / 4.0, (WINDOW_WIDTH as f32) / 4.0);
        let vy = rng.gen_range(-(WINDOW_HEIGHT as f32) / 4.0, (WINDOW_HEIGHT as f32) / 4.0);
        let texture_handle = asset_server
            .load("assets/sprite_sphere_256x256.png")
            .unwrap();
        let body = RigidBodyBuilder::new_dynamic()
            .translation(x, y)
            .linvel(vx, vy);
        // Negative friction to kind of simulate no loss of energy
        let collider = ColliderBuilder::ball(128.0 * 0.2).friction(-0.5);
        commands
            .spawn(SpriteComponents {
                transform: Transform::from_translation(Vec3::new(x, y, z)).with_scale(0.2),
                material: materials.add(texture_handle.into()),
                ..Default::default()
            })
            .with(body)
            .with(collider);
    }
}

#[derive(Default)]
struct MousePosition(Vec2);

#[derive(Default)]
struct LocalStateMousePositionSystem(EventReader<CursorMoved>);

fn mouse_position_system(
    mut state: Local<LocalStateMousePositionSystem>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut mouse_position: ResMut<MousePosition>,
) {
    for event in state.0.iter(&cursor_moved_events) {
        mouse_position.0 = event.position;
    }
}

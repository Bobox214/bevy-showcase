use bevy::{
    prelude::*,
    render::{
        camera::{OrthographicProjection, WindowOrigin},
        pass::ClearColor,
    },
};
use bevy_rapier2d::{
    na::{self, Isometry2, Vector2},
    physics::{EventQueue, RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};
use rand::prelude::*;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 800;

struct Velocity(Vector2<f32>);
fn main() {
    App::build()
        .init_resource::<MousePosition>()
        .add_resource(Msaa { samples: 2 })
        .add_resource(WindowDescriptor {
            title: "NCollide2D Bevy showcase".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.01, 0.01, 0.03)))
        .add_default_plugins()
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(setup.system())
        .add_system(mouse_position_system.system())
        .add_system(spawn_sphere_system.system())
        .add_system(position_system.system())
        .add_system(collision_system.system())
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

fn position_system(
    time: Res<Time>,
    mut bodies: ResMut<RigidBodySet>,
    mut query: Query<(Mut<Translation>, &RigidBodyHandleComponent, &Velocity)>,
) {
    let elapsed = time.delta_seconds;
    for (mut translation, body_handle, velocity) in &mut query.iter() {
        //*translation.x_mut() += velocity.0.x * elapsed;
        //*translation.y_mut() += velocity.0.y * elapsed;
        //// Wrap around screen edges
        //if translation.x() < 0.0 && velocity.0.x < 0.0 {
        //    *translation.x_mut() = WINDOW_WIDTH as f32
        //} else if translation.x() > WINDOW_WIDTH as f32 && velocity.0.x > 0.0 {
        //    *translation.x_mut() = 0.0;
        //}
        //if translation.y() < 0.0 && velocity.0.y < 0.0 {
        //    *translation.y_mut() = WINDOW_HEIGHT as f32
        //} else if translation.y() > WINDOW_HEIGHT as f32 && velocity.0.y > 0.0 {
        //    *translation.y_mut() = 0.0;
        //}

        let mut body = bodies.get_mut(body_handle.handle()).unwrap();
        *translation.x_mut() = body.position.translation.x;
        *translation.y_mut() = body.position.translation.y;
        //body.set_position(Isometry2::new(
        //    Vector2::new(translation.x() as f32, translation.y() as f32),
        //    na::zero(),
        //));
    }
}

fn collision_system(events: Res<EventQueue>) {
    while let Ok(contact_event) = events.proximity_events.pop() {
        println!("Contact event {:?}", contact_event);
    }
    while let Ok(proximity_event) = events.proximity_events.pop() {
        println!("Received proximity event: {:?}", proximity_event);
    }
    //for (h1, h2, _, manifold) in world.contact_pairs(true) {
    //    if let Some(tracked_contact) = manifold.deepest_contact() {
    //        let contact = tracked_contact.contact;
    //        let contact_normal = contact.normal.into_inner();
    //        let entity1 = *world.collision_object(h1).unwrap().data();
    //        let entity2 = *world.collision_object(h2).unwrap().data();
    //        // Reflect velocity vector of the two object around normal
    //        for (entity, mut velocity) in &mut velocities.iter() {
    //            if entity == entity1 || entity == entity2 {
    //                *velocity = Velocity(reflect(velocity.0, contact_normal));
    //            }
    //        }
    //        // Translate the second object of 'minimal translational distance' to 'depenetrate' the two objects
    //        for (entity, mut translation) in &mut translations.iter() {
    //            if entity == entity2 {
    //                *translation.0.x_mut() += contact_normal[0] * contact.depth;
    //                *translation.0.y_mut() += contact_normal[1] * contact.depth;
    //            }
    //        }
    //    }
    //}
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
        let entity = Entity::new();
        let body = RigidBodyBuilder::new_dynamic().translation(x, y);
        let collider = ColliderBuilder::ball(128.0 * 0.2);
        commands
            .spawn_as_entity(
                entity,
                SpriteComponents {
                    translation: Translation::new(x, y, z),
                    material: materials.add(texture_handle.into()),
                    scale: Scale(0.2),
                    ..Default::default()
                },
            )
            .with(Velocity(Vector2::new(vx, vy)))
            .with(body)
            .with(collider);
    }
}

fn reflect(d: Vector2<f32>, n: Vector2<f32>) -> Vector2<f32> {
    d - 2.0 * n * (d.dot(&n))
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

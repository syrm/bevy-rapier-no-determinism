use bevy::prelude::*;
use bevy::render::camera::WindowOrigin;
use bevy::window::WindowMode;
use bevy::window::WindowPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_rapier2d::prelude::*;
use config::*;

pub(crate) mod config;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                resizable: false,
                transparent: true,
                resize_constraints: bevy::window::WindowResizeConstraints {
                    min_width: WINDOW_HEIGHT * RESOLUTION,
                    max_width: WINDOW_HEIGHT * RESOLUTION,
                    min_height: WINDOW_HEIGHT,
                    max_height: WINDOW_HEIGHT,
                },
                mode: WindowMode::Windowed,
                ..Default::default()
            },
            add_primary_window: true,
            exit_on_all_closed: true,
            close_when_requested: true,
        }))
        //.add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(init_board)
        .add_startup_system(setup_physics)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();

}

fn init_board(mut commands: Commands) {
    let camera = Camera2dBundle {
        projection: OrthographicProjection {
            window_origin: WindowOrigin::BottomLeft,
            ..default()
        },
        ..default()
    };
    commands.spawn_bundle(camera);
}

fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>) {
    /* Create the ground. */
   /* commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(200.0, 25.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(600.0, 5.0, 0.0)));
    */

    /* Create walls. */
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(5.0, 544.0, 0.0)))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5.0, 536.0))
        .insert(Restitution::coefficient(1.0));

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(
            1915.0, 544.0, 0.0,
        )))
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5.0, 536.0))
        .insert(Restitution::coefficient(1.0));

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(960.0, 0.0, 0.0)))
        .insert(Collider::cuboid(1920.0, 5.0))
        .insert(Restitution::coefficient(1.0));

    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(960.0, 1075.0, 0.0)))
        .insert(Collider::cuboid(1920.0, 5.0))
        .insert(Restitution::coefficient(1.0));

    let player: Handle<Image> = asset_server.load("player.png");
    /* Create the bouncing ball. */
    //let _rng = rand::thread_rng();
    for _i in 1..2 {
    let mut bowl = commands
        .spawn_bundle(SpriteBundle {
            texture: player.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(38.0, 38.0)),
                ..Default::default()
            },
            ..Default::default()
        });

        bowl
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(20.0))
        .insert(Restitution::coefficient(1.0))
        .insert(GravityScale(0.0))
 //       .insert(Damping { linear_damping: 0.6, angular_damping: 0.5 })
/*
        .insert(ExternalForce {
            force: Vec2::new(rng.gen_range(-15.0..15.0), 4.0),
            torque: 0.0,
        })
*/
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            100.0, 100.0, 0.0,
        )));

        bowl.insert(ExternalImpulse {
            impulse: Vec2::new(85.0, 50.0),
            torque_impulse: 0.0
        });
    }
}

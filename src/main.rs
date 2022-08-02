use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::{
    prelude::{Collider, LockedAxes, NoUserData, RapierPhysicsPlugin, RigidBody},
    // render::RapierDebugRenderPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(camera_controller)
        .add_system(player_controller)
        .add_system(grab_mouse)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera

    // player
    commands
        .spawn_bundle(TransformBundle::from(Transform::from_xyz(5.0, 3.0, 5.0)))
        .insert(
            LockedAxes::ROTATION_LOCKED_X
                | LockedAxes::ROTATION_LOCKED_Y
                | LockedAxes::ROTATION_LOCKED_Z,
        )
        .insert(Collider::capsule_y(0.5, 0.5))
        .insert(PlayerController::default())
        .insert(RigidBody::Dynamic)
        .with_children(|parent| {
            parent
                .spawn_bundle(Camera3dBundle {
                    transform: Transform::from_xyz(0.0, 0.5, 0.0)
                        .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
                    ..default()
                })
                .insert(CameraController::default());
        });

    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 200.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..Default::default()
            }),
            ..default()
        })
        .insert(Collider::cuboid(200.0, 0.0, 200.0));

    // cubes
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..Default::default()
            }),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.25, 1.5, 0.25)))
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(RigidBody::Dynamic);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..Default::default()
            }),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.25, 3.0, -0.25)))
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(RigidBody::Dynamic);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..Default::default()
            }),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -0.25, 4.5, -0.25,
        )))
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(RigidBody::Dynamic);

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.5, 0.5),
                ..Default::default()
            }),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(-0.25, 6.0, 0.25)))
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(RigidBody::Dynamic);

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        point_light: PointLight {
            intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

#[derive(Component)]
struct CameraController {
    pub enabled: bool,
    pub sensitivity: f32,
    pub pitch: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            sensitivity: 0.5,
            pitch: 0.0,
        }
    }
}

fn camera_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    // Handle mouse input
    let mut mouse_delta = Vec2::ZERO;
    for mouse_event in mouse_events.iter() {
        mouse_delta += mouse_event.delta;
    }

    for (mut transform, mut options) in &mut query {
        if !options.enabled {
            continue;
        }

        if mouse_delta != Vec2::ZERO {
            // Apply look update
            let pitch = (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt).clamp(
                -0.99 * std::f32::consts::FRAC_PI_2,
                0.99 * std::f32::consts::FRAC_PI_2,
            );
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, pitch);
            options.pitch = pitch;
        }
    }
}

#[derive(Component)]

struct PlayerController {
    pub enabled: bool,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub velocity: Vec3,
    pub sensitivity: f32,
    pub yaw: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            enabled: true,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::LShift,
            walk_speed: 10.0,
            run_speed: 30.0,
            friction: 0.5,
            velocity: Vec3::ZERO,
            sensitivity: 0.5,
            yaw: 0.0,
        }
    }
}

fn player_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut PlayerController), Without<Camera>>,
) {
    let dt = time.delta_seconds();

    // Handle mouse input
    let mut mouse_delta = Vec2::ZERO;
    for mouse_event in mouse_events.iter() {
        mouse_delta += mouse_event.delta;
    }

    for (mut transform, mut options) in &mut query {
        if !options.enabled {
            continue;
        }

        // Handle key input
        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(options.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(options.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(options.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(options.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.0;
        }

        if mouse_delta != Vec2::ZERO {
            // Apply look update
            let yaw = options.yaw - mouse_delta.x * options.sensitivity * dt;
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, yaw, 0.0);
            options.yaw = yaw;
        }

        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };
            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation += options.velocity.x * dt * right
            + options.velocity.y * dt * Vec3::Y
            + options.velocity.z * dt * forward;
    }
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
fn grab_mouse(
    mut windows: ResMut<Windows>,
    mouse: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();
    if mouse.just_pressed(MouseButton::Left) {
        window.set_cursor_visibility(false);
        window.set_cursor_lock_mode(true);
    }
    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_visibility(true);
        window.set_cursor_lock_mode(false);
    }
}

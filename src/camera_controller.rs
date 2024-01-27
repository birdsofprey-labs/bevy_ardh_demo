use bevy::{input::mouse::MouseMotion, prelude::*};

use std::f32::consts::*;

pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

#[derive(Component)]
pub struct CameraController {
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_controller);
    }
}

fn camera_controller(
    time: Res<Time>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut transform, mut state)) = query.get_single_mut() {
        // Handle key input
        let mut axis_input = Vec3::ZERO;
        let mut rot = 0.0f32;
        if key_input.pressed(KeyCode::W) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(KeyCode::S) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(KeyCode::D) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(KeyCode::A) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(KeyCode::E) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(KeyCode::Q) {
            axis_input.y -= 1.0;
        }

        if key_input.pressed(KeyCode::Z) {
            rot += 1.0;
        }
        if key_input.pressed(KeyCode::X) {
            rot -= 1.0;
        }


        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(KeyCode::ShiftLeft) {
                1500.0
            } else if key_input.pressed(KeyCode::ControlLeft) {
                50.0
            } else {
                500.0
            };
            state.velocity = axis_input.normalize() * max_speed;
        } else {
            state.velocity *= 0.5; // friction
            if state.velocity.length_squared() < 1e-6 {
                state.velocity = Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation += state.velocity.x * dt * right
            + state.velocity.y * dt * Vec3::Y
            + state.velocity.z * dt * forward;

        // Handle mouse input
        let mut mouse_delta = Vec2::ZERO;
        if mouse_button_input.pressed(MouseButton::Left) {
            for mouse_event in mouse_events.iter() {
                mouse_delta += mouse_event.delta;
            }
        }
        if mouse_delta != Vec2::ZERO {
            // Apply look update
            state.pitch =
                (state.pitch - mouse_delta.y * RADIANS_PER_DOT).clamp(-PI / 2., PI / 2.);
            state.yaw -= mouse_delta.x * RADIANS_PER_DOT;
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, state.yaw, state.pitch);
        }

        transform.rotation = transform.rotation.mul_quat( Quat::from_euler(EulerRot::ZYX, rot*0.01, 0.0, 0.0) );
    }
}
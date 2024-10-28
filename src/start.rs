use bevy::prelude::*;
use bevy::{
    app::{App, Startup},
    asset::Assets,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::system::{Commands, ResMut},
    prelude::default,
    render::mesh::Mesh,
    DefaultPlugins,
};
use wasm_bindgen::prelude::*;

use crate::ball::Ball;
use crate::common::*;

#[wasm_bindgen]
pub fn start(canvas_id: &str) {
    let id = format!("#{}", canvas_id);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some(id.into()),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_ball, //
                update_transforms,
            ),
        )
        .run();
}

fn setup(
    mut windows: Query<&mut Window>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut window = windows.single_mut();
    let canvas_id = window.canvas.as_ref().unwrap().trim_start_matches("#");
    let (width, height) = {
        use wasm_bindgen::JsCast;
        use web_sys::window;

        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();

        let el = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        // Get the "width" and "height" attributes of the canvas
        (el.width() as f32, el.height() as f32)
    };

    window.resolution.set(width, height);
    window.resizable = false;

    commands.spawn(Camera2dBundle::default());

    for _ in 0..1000 {
        let px = (rand::random::<f32>() - 0.5) * window.width();
        let py = (rand::random::<f32>() - 0.5) * window.height();
        Ball::spawn(px, py, &mut commands, &mut meshes, &mut materials);
    }
}

fn move_ball(mut ent: Query<(&mut Position, &mut Velocity, &Ball)>, windows: Query<&Window>) {
    let window = windows.single();
    let window_size = Vec2::new(window.width(), window.height());
    let half_width = window_size.x / 2.;
    let half_height = window_size.y / 2.;

    for (mut position, mut velocity, ball) in &mut ent {
        let v = &mut velocity.value;
        let p = &mut position.value;
        let q = *p + *v;
        if q.x < -half_width + ball.radius {
            p.x = -half_width + ball.radius;
            v.x = v.x.abs();
        }
        if q.x > half_width - ball.radius {
            p.x = half_width - ball.radius;
            v.x = -v.x.abs();
        }
        if q.y < -half_height + ball.radius {
            p.y = -half_height + ball.radius;
            v.y = v.y.abs();
        }
        if q.y > half_height - ball.radius {
            p.y = half_height - ball.radius;
            v.y = -v.y.abs();
        }
        *p = q;
    }
}

fn update_transforms(mut ent: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut ent {
        transform.translation = Vec3::new(position.value.x, position.value.y, 0.);
    }
}

use crate::common::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    velocity: Velocity,
    position: Position,
}

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
}

fn rand_sign() -> f32 {
    if rand::random::<f32>() > 0.5 {
        1.0
    } else {
        -1.0
    }
}

impl Ball {
    pub fn spawn(
        px: f32,
        py: f32,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let ball = Ball {
            radius: rand::random::<f32>() * 20.0 + 1.0,
        };

        let mesh = Mesh::from(Circle::new(ball.radius));
        let color = ColorMaterial::from(Color::rgb(
            rand::random::<f32>() * 0.2,
            rand::random::<f32>(),
            rand::random::<f32>() * 0.2 + 0.6,
        ));

        let mesh_handle = meshes.add(mesh);
        let material_handle = materials.add(color);

        let vx = rand_sign() * ((rand::random::<f32>() * 2.) + 0.5);
        let vy = rand_sign() * ((rand::random::<f32>() * 2.) + 0.5);
        let s = 0.025;
        let (vx, vy) = (vx * s, vy * s);

        commands.spawn((
            BallBundle {
                ball,
                position: Position {
                    value: Vec2::new(px, py),
                },
                velocity: Velocity {
                    value: Vec2::new(vx, vy),
                },
            },
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                material: material_handle,
                ..default()
            },
        ));
    }
}

//! This example shows various ways to configure texture materials in 3D.

mod cameras;

use std::f32::consts::PI;
use std::time::Duration;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
    // window::*,
    winit::*,
};

// static mut scaleFactor: f32 = 0. ;

struct OverlayColor;

impl OverlayColor {
    const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // Tells Wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // provide the ID selector string here
                    canvas: Some("#draupnir".into()),
                    // ... any other window properties ...
                    ..default()
                }),
                ..default()
            }),
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        // Here we define size of our overlay
                        font_size: 35.0,
                        // If we want, we can use a custom font
                        font: default(),
                        // We could also disable font smoothing,
                        font_smoothing: FontSmoothing::default(),
                    },
                    // We can also change color of the overlay
                    text_color: OverlayColor::WHITE,
                    enabled: true,
                },
            },
        ))
        .add_systems(Startup, (setup, init_refresh_rate, cameras::setup_camera))
        .add_systems(Update, (rotate_over_time, cameras::fit_canvas))
        // .add_systems(Update, update_resolution)
        .run();
}

fn init_refresh_rate(mut winit: ResMut<WinitSettings>) {
    winit.focused_mode = UpdateMode::reactive(Duration::from_secs_f32(1.0 / 60.0));
}

#[derive(Component)]
struct RotatingEntity;

fn rotate_over_time(time: Res<Time>, mut query: Query<&mut Transform, With<RotatingEntity>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_z(0.6 * time.delta_secs());
        // transform.rotate_x(0.8 * time.delta_secs());
    }
}

// sets up a scene with textured entities
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // load a texture and retrieve its aspect ratio
    let texture_handle = asset_server.load("branding/bevy_logo_dark_big.png");
    let aspect = 0.25;

    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 8.0;
    let quad_handle = meshes.add(Rectangle::new(quad_width, quad_width * aspect));

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // this material modulates the texture to make it red (and slightly transparent)
    let red_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.0, 0.0, 0.5),
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // and lets make this one blue! (and also slightly transparent)
    let blue_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.0, 1.0, 0.5),
        base_color_texture: Some(texture_handle),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    // textured quad - normal
    commands.spawn((
            Mesh3d(quad_handle.clone()),
            MeshMaterial3d(material_handle),
            Transform::from_xyz(0.0, 0.0, 1.5).with_rotation(Quat::from_rotation_x(-PI / 5.0)),
            cameras::PIXEL_PERFECT_LAYERS,
        ))
        .insert(RotatingEntity);
    // textured quad - modulated
    commands.spawn((
            Mesh3d(quad_handle.clone()),
            MeshMaterial3d(red_material_handle),
            Transform::from_rotation(Quat::from_rotation_x(-PI / 5.0)),
            cameras::PIXEL_PERFECT_LAYERS,
        ))
        .insert(RotatingEntity);
    // textured quad - modulated
    commands.spawn((
            Mesh3d(quad_handle),
            MeshMaterial3d(blue_material_handle),
            Transform::from_xyz(0.0, 0.0, -1.5).with_rotation(Quat::from_rotation_x(-PI / 5.0)),
            cameras::PIXEL_PERFECT_LAYERS,
        ))
        .insert(RotatingEntity);
    // camera
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(3.0, 5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ));
}



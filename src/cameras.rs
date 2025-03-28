use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowResized,
};

/// Low-resolution texture that contains the pixel-perfect world.
/// Canvas itself is rendered to the high-resolution world.
#[derive(Component)]
struct Canvas;

/// Camera that renders the pixel-perfect world to the [`Canvas`].
#[derive(Component)]
struct InGameCamera;

/// Camera that renders the [`Canvas`] (and other graphics on [`HIGH_RES_LAYERS`]) to the screen.
#[derive(Component)]
pub struct OuterCamera;

/// In-game resolution width.
// const RES_WIDTH: u32 = 300;
const RES_WIDTH: u32 = 854;
// const RES_WIDTH: u32 = 1920;

/// In-game resolution height.
// const RES_HEIGHT: u32 = 168;
const RES_HEIGHT: u32 = 480;
// const RES_HEIGHT: u32 = 1080;

/// Default render layers for pixel-perfect rendering.
/// You can skip adding this component, as this is the default.
pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

/// Render layers for high-resolution rendering.
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

pub fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    // this Image serves as a canvas representing the low-resolution game screen
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    // this camera renders whatever is on `PIXEL_PERFECT_LAYERS` to the canvas
    // commands.spawn((
    //     Camera3d,
    //     Camera {
    //         // render before the "main pass" camera
    //         order: -1,
    //         target: RenderTarget::Image(image_handle.clone()),
    //         ..default()
    //     },
    //     Msaa::Off,
    //     InGameCamera,
    //     PIXEL_PERFECT_LAYERS,
    //     Transform::from_xyz(3.0, 5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    // ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: -1,
            target: RenderTarget::Image(image_handle.clone()),
            ..default()
        },
        Transform::from_xyz(3.0, 5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        Msaa::Off,
        InGameCamera,
        PIXEL_PERFECT_LAYERS,
    ));


    // spawn the canvas
    commands.spawn((Sprite::from_image(image_handle), Canvas, HIGH_RES_LAYERS));

    // the "outer" camera renders whatever is on `HIGH_RES_LAYERS` to the screen.
    // here, the canvas and one of the sample sprites will be rendered by this camera
    commands.spawn((Camera2d, Msaa::Off, OuterCamera, HIGH_RES_LAYERS));
}

/// Scales camera projection to fit the window (integer multiples only).
pub fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projection: Single<&mut OrthographicProjection, With<OuterCamera>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / RES_WIDTH as f32;
        let v_scale = event.height / RES_HEIGHT as f32;
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
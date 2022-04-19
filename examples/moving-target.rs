use bevy::{
    pbr::wireframe::WireframePlugin,
    prelude::*,
    render::{options::WgpuOptions, render_resource::WgpuFeatures},
};

use space::{
    camera::tag::*,
    fps::FpsPlugin,
    lock_on::LockOnPlugin,
    origin::{OriginRebasingPlugin, SimulationBundle},
    projectile::{ProjectilePlugin, Target},
    raycast::{RayCastMesh, RayCastSource, RaycastPlugin},
    tag::{MyRaycastSet, PlayerModelTag, PlayerTag},
    util::setup_crosshair,
};
use space::{
    camera::*,
    controller::{tag::ControllerPlayerTag, ControllerPlugin},
};
use space::{scale::*, util::setup_cursor};

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            vsync: false,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ControllerPlugin)
        .add_plugin(OriginRebasingPlugin)
        .add_plugin(RaycastPlugin::<MyRaycastSet>::default())
        .add_plugin(FpsPlugin)
        .add_plugin(LockOnPlugin)
        .add_plugin(ProjectilePlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_cursor)
        .add_startup_system(spawn_target)
        .add_startup_system(spawn_light)
        .add_startup_system(setup_crosshair)
        .add_system(move_target)
        // .add_system(handle_lock_on)
        // .add_system(highlight_marker)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spaceship_handle = asset_server.load("models/spaceship.gltf#Mesh0/Primitive0");

    let dimensions = Vec3::new(0.5, 0.5, 1.0) * M_TO_UNIT_SCALE;

    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        reflectance: 1.0,
        ..Default::default()
    });

    let body = commands
        .spawn_bundle((GlobalTransform::identity(), Transform::identity()))
        .insert(ControllerPlayerTag)
        .insert(PlayerTag)
        .insert_bundle(SimulationBundle::default())
        .id();

    let player = commands
        .spawn_bundle(PbrBundle {
            mesh: spaceship_handle,
            material: cube_material_handle,
            transform: Transform::from_scale(dimensions),
            ..Default::default()
        })
        .insert(PlayerModelTag)
        // .insert(Wireframe)
        .id();

    let camera = commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 2.25, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            perspective_projection: PerspectiveProjection {
                far: 1_000.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RayCastSource::<MyRaycastSet>::new_transform_empty())
        .insert_bundle((LookDirection::default(), CameraTag))
        .id();

    commands
        .entity(body)
        .insert(LookEntity(camera))
        .push_children(&[player, camera]);
}

fn spawn_target(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut spawn_cube = |position, color, name| {
        let scale = 50.0;
        let cube_handle = meshes.add(Mesh::from(shape::Cube::default()));
        let cube_material_handle = materials.add(StandardMaterial {
            base_color: color,
            reflectance: 0.02,
            unlit: false,
            ..Default::default()
        });
        commands
            .spawn_bundle(PbrBundle {
                mesh: cube_handle.clone(),
                material: cube_material_handle.clone(),
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::splat(scale)),
                ..Default::default()
            })
            .insert(Target::default())
            .insert(Name::new(name))
            .insert(RayCastMesh::<MyRaycastSet>::default());
    };

    spawn_cube(Vec3::new(-15.0, 0.0, -15.0), Color::RED, "Sara");
}

fn spawn_light(mut commands: Commands) {
    let theta = std::f32::consts::FRAC_PI_4;
    let light_transform = Mat4::from_euler(EulerRot::ZYX, 0.0, std::f32::consts::FRAC_PI_2, -theta);
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 9_999.0,
            shadow_projection: OrthographicProjection {
                left: -0.35,
                right: 500.35,
                bottom: -0.1,
                top: 5.0,
                near: -5.0,
                far: 5.0,
                ..Default::default()
            },
            shadow_depth_bias: 0.0,
            shadow_normal_bias: 0.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_matrix(light_transform),
        ..Default::default()
    });
}

fn move_target(time: Res<Time>, mut query: Query<(&mut Transform, &mut Target)>) {
    for (mut transform, mut target) in query.iter_mut() {
        let size = 30.0;
        let cosine = time.seconds_since_startup().cos() as f32;
        let sine = time.seconds_since_startup().sin() as f32;
        target.velocity = Vec3::new(cosine * size, sine * size, 0.0);
        transform.translation += target.velocity * time.delta_seconds();
    }
}

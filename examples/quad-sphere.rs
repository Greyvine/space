use bevy::{
    input::system::exit_on_esc_system,
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{options::WgpuOptions, render_resource::WgpuFeatures},
};

use space::{
    camera::tag::*,
    material::{convert_skyboxes, CustomMaterial, SkyboxTextureConversion},
    mesh::QuadSphere,
    origin::{OriginRebasingPlugin, SimulationBundle},
    tag::PlayerTag,
};
use space::{
    camera::*,
    controller::{tag::ControllerPlayerTag, ControllerPlugin},
};
use space::{planet::EarthTag, scale::*};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Planet;

fn main() {
    App::new()
        .init_resource::<SkyboxTextureConversion>()
        .insert_resource(WgpuOptions {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ControllerPlugin)
        .add_plugin(OriginRebasingPlugin)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.1,
        })
        // .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .add_startup_system(spawn_marker)
        .add_system(rotator_system)
        .add_system(exit_on_esc_system)
        .add_system(convert_skyboxes)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dimensions = Vec3::new(1.2, 1.0, 5.0) * M_TO_UNIT_SCALE;

    let cube_handle = meshes.add(Mesh::from(shape::Cube::default()));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        reflectance: 0.02,
        unlit: false,
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
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_scale(dimensions),
            ..Default::default()
        })
        .insert(Wireframe)
        .id();

    let camera = commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 2.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            perspective_projection: PerspectiveProjection {
                far: 10.0 * AU_TO_UNIT_SCALE,
                // near: 0.5 * AU_TO_UNIT_SCALE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle((LookDirection::default(), CameraTag))
        .id();

    commands
        .entity(body)
        .insert(LookEntity(camera))
        .push_children(&[player, camera]);
}

fn spawn_marker(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut skybox_conversion: ResMut<SkyboxTextureConversion>,
) {
    let texture_handle = asset_server.load("textures/earth-cube-map.png");
    skybox_conversion.make_array(texture_handle.clone());

    let material = CustomMaterial {
        base_color_texture: Some(texture_handle.clone()),
        color: Color::GREEN,
    };

    let sphere_handle = meshes.add(Mesh::from(QuadSphere {
        radius: 20.0,
        subdivisions: 30,
    }));

    // let sphere_handle = meshes.add(Mesh::from(shape::UVSphere {
    //     radius: 20.0,
    //     sectors: 5,
    //     stacks: 5,
    // }));

    // let sphere_handle = meshes.add(Mesh::from(shape::Icosphere {
    //     radius: 20.0,
    //     subdivisions: 30,
    // }));

    commands
        .spawn_bundle(MaterialMeshBundle {
            mesh: sphere_handle,
            material: materials.add(material),
            // transform: Transform::from_translation(Vec3::Z * -20.0),
            transform: Transform::from_xyz(0.0, 0.0, -30.0),
            // .with_rotation(Quat::from_rotation_y(PI)),
            ..Default::default()
        })
        // .insert(Wireframe)
        .insert(EarthTag);
}

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<EarthTag>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(0.5 * time.delta_seconds());
    }
}

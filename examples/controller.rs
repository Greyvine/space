use bevy::{
    asset::Asset,
    pbr::{
        wireframe::{Wireframe, WireframePlugin},
        SpecializedMaterial,
    },
    prelude::*,
    render::{options::WgpuOptions, render_resource::WgpuFeatures},
};

use space::{
    camera::tag::*,
    fps::FpsPlugin,
    origin::{OriginRebasingPlugin, SimulationBundle},
    raycast::{event::HoverEvent, RayCastMesh, RayCastSource, RaycastPlugin},
    tag::{PlayerModelTag, PlayerTag},
    util::setup_crosshair,
};
use space::{
    camera::*,
    controller::{tag::ControllerPlayerTag, ControllerPlugin},
};
use space::{scale::*, util::setup_cursor};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Planet;

struct MyRaycastSet;

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
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ControllerPlugin)
        .add_plugin(OriginRebasingPlugin)
        .add_plugin(RaycastPlugin::<MyRaycastSet>::default())
        .add_plugin(FpsPlugin)
        .add_startup_system(setup)
        // .add_startup_system(setup_cursor)
        .add_startup_system(spawn_marker)
        .add_startup_system(spawn_light)
        .add_startup_system(setup_crosshair)
        .add_system(handle_lock_on)
        // .add_system(highlight_marker_events)
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
            mesh: cube_handle,
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
                far: 10.0 * AU_TO_UNIT_SCALE,
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

fn spawn_marker(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut spawn_cube = |position, color, name| {
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
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .insert(Name::new(name))
            .insert(RayCastMesh::<MyRaycastSet>::default());
        // .insert(Wireframe);
    };

    spawn_cube(Vec3::Y * 15.0, Color::RED, "Aaron");
    spawn_cube(Vec3::Y * -15.0, Color::ORANGE, "Sara");
    spawn_cube(Vec3::Z * 15.0, Color::RED, "Jeff");
    spawn_cube(Vec3::Z * -15.0, Color::ORANGE, "David");
    spawn_cube(Vec3::X * 15.0, Color::RED, "Per");
    spawn_cube(Vec3::X * -15.0, Color::ORANGE, "Thomas");
}

fn spawn_light(mut commands: Commands) {
    let theta = std::f32::consts::FRAC_PI_4;
    let light_transform = Mat4::from_euler(EulerRot::ZYX, 0.0, std::f32::consts::FRAC_PI_2, -theta);
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100000.0,
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

// // #[allow(clippy::type_complexity)] <T: Asset>
// fn highlight_marker(
//     query: Query<&mut RayCastSource<MyRaycastSet>>,
//     mut raycast_meshes: Query<&mut Visibility, With<RayCastMesh<MyRaycastSet>>>,
// ) {
//     let source = query.single();
//     for (entity, _) in source.intersections.iter() {
//         if let Ok(mut visibility) = raycast_meshes.get_mut(*entity) {
//             // visibility.is_visible = false;
//         }
//     }
// }

// fn highlight_marker_events(
//     mut hover_events: EventReader<HoverEvent>,
//     mut raycast_meshes: Query<&mut Visibility, With<RayCastMesh<MyRaycastSet>>>,
// ) {
//     for event in hover_events.iter().next() {
//         if let HoverEvent::JustEntered(entity) = event {
//             if let Ok(mut visibility) = raycast_meshes.get_mut(*entity) {
//                 // visibility.is_visible = false;
//             }
//         }
//     }
// }

fn handle_lock_on(
    keys: Res<Input<KeyCode>>,
    query: Query<&mut RayCastSource<MyRaycastSet>>,
    mut raycast_meshes: Query<(&Name, &mut Visibility), With<RayCastMesh<MyRaycastSet>>>,
) {
    if keys.pressed(KeyCode::LControl) {
        let source = query.single();
        if let Some((entity, _)) = source.intersections.iter().next() {
            if let Ok((name, _)) = raycast_meshes.get_mut(*entity) {
                println!("Lock-on to {}!", name.as_str());
            }
        }
    }
}

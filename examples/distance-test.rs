use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    render::{camera::PerspectiveProjection, options::WgpuOptions, render_resource::WgpuFeatures},
};

#[derive(Component)]
pub struct FirstPassCube;

const AU_TO_UNIT_SCALE: f32 = 149_597_870_700.0 * M_TO_UNIT_SCALE;
const KM_TO_UNIT_SCALE: f32 = 1_000.0 * M_TO_UNIT_SCALE;
const M_TO_UNIT_SCALE: f32 = 1.0;
const RADIUS: f32 = 695_508.0 * KM_TO_UNIT_SCALE;

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_startup_system(setup.system())
        .add_system(rotator_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 2.0 * RADIUS;
    let distance = -1.0 * AU_TO_UNIT_SCALE;

    let cube_handle = meshes.add(Mesh::from(shape::Cube { size }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::NAVY,
        reflectance: 0.02,
        emissive: Color::NAVY,
        unlit: false,
        ..Default::default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, distance)),
            ..Default::default()
        })
        .insert(Wireframe)
        .insert(FirstPassCube);

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
            .looking_at(Vec3::default(), Vec3::Y),
        perspective_projection: PerspectiveProjection {
            far: 10.0 * AU_TO_UNIT_SCALE,
            // near: 0.5 * AU_TO_UNIT_SCALE,
            ..Default::default()
        },
        ..Default::default()
    });
}

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<FirstPassCube>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(1.5 * time.delta_seconds());
        transform.rotation *= Quat::from_rotation_z(1.3 * time.delta_seconds());
    }
}

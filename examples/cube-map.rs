use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{
        wireframe::{Wireframe, WireframePlugin},
        MaterialPipeline,
    },
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer,
            BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages, Extent3d,
            SamplerBindingType, ShaderStages, Texture, TextureDimension, TextureFormat,
            TextureSampleType, TextureUsages, TextureViewDimension,
        },
        renderer::RenderDevice,
    },
};
use space::{
    material::{convert_skyboxes, CustomMaterial, SkyboxTextureConversion},
    mesh::QuadSphere,
    planet::EarthTag,
};

const SIZE: (u32, u32) = (1280, 720);

fn main() {
    App::new()
        .init_resource::<SkyboxTextureConversion>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .add_system(convert_skyboxes)
        .add_system(rotator_system)
        .run();
}

/// set up a simple 3D scene
fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut skybox_conversion: ResMut<SkyboxTextureConversion>,
) {
    let texture_handle = asset_server.load("textures/labelled-cube-map.png");
    skybox_conversion.make_array(texture_handle.clone());

    let material_handle = CustomMaterial {
        base_color_texture: Some(texture_handle.clone()),
        color: Color::GREEN,
    };

    // cube
    // commands.spawn().insert_bundle(MaterialMeshBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     material: materials.add(material_handle),
    //     ..Default::default()
    // });

    commands
        .spawn()
        .insert_bundle(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(QuadSphere {
                radius: 2.0,
                subdivisions: 20,
            })),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: materials.add(material_handle),
            ..Default::default()
        })
        .insert(EarthTag)
        .insert(Wireframe);

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<EarthTag>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(0.5 * time.delta_seconds());
    }
}

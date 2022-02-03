use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{wireframe::{Wireframe, WireframePlugin}, MaterialPipeline},
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
use space::mesh::QuadSphere;

const SIZE: (u32, u32) = (1280, 720);

fn main() {
    App::new()
        .init_resource::<SkyboxTextureConversion>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .add_system(convert_skyboxes)
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
        .insert(Wireframe);

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

// This is the struct that will be passed to your shader
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct CustomMaterial {
    pub base_color_texture: Option<Handle<Image>>,
    color: Color,
}

#[derive(Clone)]
pub struct GpuCustomMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

// The implementation of [`Material`] needs this impl to work properly.
impl RenderAsset for CustomMaterial {
    type ExtractedAsset = CustomMaterial;
    type PreparedAsset = GpuCustomMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<MaterialPipeline<Self>>,
        SRes<RenderAssets<Image>>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline, gpu_images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let color = Vec4::from_slice(&extracted_asset.color.as_linear_rgba_f32());
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: color.as_std140().as_bytes(),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let (base_color_texture_view, base_color_sampler) = if let Some(result) = material_pipeline
            .mesh_pipeline
            .get_image_texture(gpu_images, &extracted_asset.base_color_texture)
        {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(extracted_asset));
        };
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(base_color_texture_view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(base_color_sampler),
                },
            ],
            label: None,
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuCustomMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl Material for CustomMaterial {
    // When creating a custom material, you need to define either a vertex shader, a fragment shader or both.
    // If you don't define one of them it will use the default mesh shader which can be found at
    // <https://github.com/bevyengine/bevy/blob/latest/crates/bevy_pbr/src/render/mesh.wgsl>

    // For this example we don't need a vertex shader
    // fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
    //     // Use the same path as the fragment shader since wgsl let's you define both shader in the same file
    //     Some(asset_server.load("shaders/custom_material.wgsl"))
    // }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/custom-material.wgsl"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
                    },
                    count: None,
                },
                // Base Color Texture
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2Array,
                    },
                    count: None,
                },
                // Base Color Texture Sampler
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("cube_map_material_layout"),
        })
    }
}

#[derive(Default)]
pub struct SkyboxTextureConversion {
    /// List of texture handles that should be skyboxes.
    handles: Vec<Handle<Image>>,
}

impl SkyboxTextureConversion {
    /// Takes a handle to a texture whose dimensions are `N` wide by `6*N` high, waits for it to load,
    /// and then reinterprets that texture as an array of 6 textures suitable or a skybox. This is
    /// useful if your skybox texture is not in a format that has layers. This should only be done
    /// once per testure, and will panic if the texture has already be reinterpreted.
    pub fn make_array(&mut self, handle: Handle<Image>) {
        self.handles.push(handle);
    }
}

/// System to handle reinterpreting an Nx6N vertical texture stack as an array of textures suitable
/// for a skybox.
fn convert_skyboxes(
    mut conversions: ResMut<SkyboxTextureConversion>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut i = 0;
    loop {
        // Check each texture in the pending queue to see if it is loaded yet.
        let (handle, texture) = match conversions.handles.get(i) {
            Some(handle) => match textures.get_mut(handle) {
                // If it's loaded, take it out of the queue.
                Some(texture) => (conversions.handles.remove(i), texture),
                None => {
                    i += 1;
                    continue;
                }
            },
            None => break,
        };

        println!("ASSSHOLE!!!");
        info!(
            "Reinterpreting as Skybox Texture {:?}: format: {:?}, len: {}, extents: {:?}",
            handle,
            texture.texture_descriptor.format,
            texture.data.len(),
            texture.texture_descriptor.size
        );
        texture.reinterpret_stacked_2d_as_array(6);
    }
}

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
pub fn convert_skyboxes(
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

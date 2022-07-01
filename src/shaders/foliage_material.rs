use bevy::{
    ecs::system::{lifetimeless::*, *},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::*,
        render_resource::{
            std140::{AsStd140, Std140},
            *,
        },
        renderer::*,
    },
    sprite::*,
};
use bevy_rapier2d::prelude::*;

use super::resources::ExtractedVelocity;

#[derive(TypeUuid, Clone)]
#[uuid = "34e5bafb-2685-471e-8104-a6b20814479b"]
pub struct FoliageMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
    pub velocity: Vec2,
}

#[derive(Clone, AsStd140)]
pub struct FoliageMaterialUniformData {
    pub color: Vec4,
    pub velocity: Vec2,
}

pub struct FoliageMaterialGPU {
    bind_group: BindGroup,
    pub uniform_data: FoliageMaterialUniformData,
    pub buffer: Buffer,
}

impl RenderAsset for FoliageMaterial {
    type ExtractedAsset = FoliageMaterial;
    type PreparedAsset = FoliageMaterialGPU;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<FoliageMaterial>>,
        SRes<RenderAssets<Image>>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, pipeline, images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let (view, sampler) = if let Some(result) = pipeline
            .mesh2d_pipeline
            .get_image_texture(images, &Some(extracted_asset.texture.clone()))
        {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(extracted_asset));
        };

        let uniform_data = FoliageMaterialUniformData {
            color: extracted_asset.color.as_linear_rgba_f32().into(),
            velocity: extracted_asset.velocity,
        };

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: uniform_data.as_std140().as_bytes(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(sampler),
                },
            ],
        });
        Ok(FoliageMaterialGPU {
            bind_group,
            uniform_data,
            buffer,
        })
    }
}

impl Material2d for FoliageMaterial {
    fn bind_group(material: &FoliageMaterialGPU) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            FoliageMaterialUniformData::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        })
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/foliage_material.wgsl"))
    }
}

pub fn prepare_foliage_material(
    mut material_assets: ResMut<RenderAssets<FoliageMaterial>>,
    velocity: Res<ExtractedVelocity>,
    render_queue: Res<RenderQueue>,
) {
    for mat in material_assets.values_mut() {
        mat.uniform_data.velocity = velocity.0;
        render_queue.write_buffer(&mat.buffer, 0, mat.uniform_data.as_std140().as_bytes());
    }
}

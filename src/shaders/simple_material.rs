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

#[derive(TypeUuid, Clone)]
#[uuid = "0c0015b2-0d08-4d49-9f64-dcdd13cf774a"]
pub struct SimpleMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

#[derive(Clone, AsStd140)]
struct SimpleMaterialUniformData {
    color: Vec4,
}

pub struct SimpleMaterialGPU {
    bind_group: BindGroup,
}

impl RenderAsset for SimpleMaterial {
    type ExtractedAsset = SimpleMaterial;
    type PreparedAsset = SimpleMaterialGPU;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<SimpleMaterial>>,
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

        let data = SimpleMaterialUniformData {
            color: extracted_asset.color.as_linear_rgba_f32().into(),
        };

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: data.as_std140().as_bytes(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &&pipeline.material2d_layout,
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
        Ok(SimpleMaterialGPU { bind_group })
    }
}

impl Material2d for SimpleMaterial {
    fn bind_group(material: &SimpleMaterialGPU) -> &BindGroup {
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
                            SimpleMaterialUniformData::std140_size_static() as u64,
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
        Some(asset_server.load("shaders/simple_material.wgsl"))
    }
}

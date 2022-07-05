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
#[uuid = "683f2a9e-c026-448d-a7d6-0a80b63d0f6f"]
pub struct DaylightMaterial {
    pub color: Color,
}

#[derive(Clone, AsStd140)]
struct DaylightMaterialUniformData {
    color: Vec4,
}

pub struct DaylightMaterialGPU {
    bind_group: BindGroup,
}

impl RenderAsset for DaylightMaterial {
    type ExtractedAsset = DaylightMaterial;
    type PreparedAsset = DaylightMaterialGPU;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<DaylightMaterial>>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {

        let data = DaylightMaterialUniformData {
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
            ],
        });
        Ok(DaylightMaterialGPU { bind_group })
    }
}

impl Material2d for DaylightMaterial {
    fn bind_group(material: &DaylightMaterialGPU) -> &BindGroup {
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
                            DaylightMaterialUniformData::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
            ],
        })
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/daylight_material.wgsl"))
    }
}

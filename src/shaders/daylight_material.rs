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
use super::super::light::*;

pub const MAX_POINT_LIGHTS: usize = 64;

#[derive(Clone, AsStd140, Copy, Default)]
pub struct PointLightGPU {
    pos: Vec2,
    radius: f32,
    enabled: u32,
}

impl PointLightGPU {
    pub fn new(pos: Vec2, radius: f32) -> Self {
        PointLightGPU {
            pos,
            radius,
            enabled: 1
        }
    }
}

#[derive(TypeUuid, Clone, Default)]
#[uuid = "683f2a9e-c026-448d-a7d6-0a80b63d0f6f"]
pub struct DaylightMaterial {
    pub color: Color,
    pub point_lights: Vec<PointLightGPU>,
}

#[derive(Clone, AsStd140)]
struct DaylightMaterialUniformData {
    color: Vec4,
    point_lights: [PointLightGPU; MAX_POINT_LIGHTS]
}

pub struct DaylightMaterialGPU {
    bind_group: BindGroup,
    uniform_data: DaylightMaterialUniformData,
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

        let mut point_lights = [PointLightGPU::default(); MAX_POINT_LIGHTS];
        for (i, point_light) in extracted_asset.point_lights.iter().enumerate() {
            point_lights[i] = *point_light;
        }

        let data = DaylightMaterialUniformData {
            color: extracted_asset.color.as_linear_rgba_f32().into(),
            point_lights
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
        Ok(DaylightMaterialGPU { bind_group, uniform_data })
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

pub struct ExtractedLights(Vec<Light>);

pub fn extract_lights(mut cmd: Commands, query: Query<&Light>) {
    cmd.insert_resource(ExtractedLights(query.iter().copied().collect()));
}
pub fn prepare_lights(mut material_assets: ResMut<RenderAssets<DaylightMaterial>>, render_queue: Res<RenderQueue>, lights: Res<ExtractedLights>) {
    for light in lights.0.iter() {
        match light {
            Light::PointLight{ radius, intensity } => {
                for mat in material_assets.values_mut() {
                    // mat.uniform_data.point_lights
                }
            },
            _ => {}
        };
    }
}

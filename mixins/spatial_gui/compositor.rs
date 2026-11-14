// Native GUI extraction and world-space presentation. Injected below HudRenderer,
// so the offscreen pass uses Horizon's exact item, text, glint and blend pipelines.
use super::{Blend, HudBatch, HudRenderer};
use crate::gfx::native_render::{HandDrawn, RenderNeeds, WorldDrawn};
use glam::{DVec3, Mat4, Vec2, Vec3};
use std::{
    cell::{Cell, RefCell},
    ops::Range,
};
use wgpu::util::DeviceExt;

pub use super::spatial_math::Surface;

#[derive(Clone, Debug)]
pub struct Presentation {
    pub frame: u64,
    pub start: u32,
    pub end: u32,
    pub surface: Surface,
    pub opacity: f32,
    pub blur: f32,
    pub tint: [f32; 3],
    pub render_scale: f32,
    pub linear: bool,
    pub anisotropic: bool,
    pub shadow_layers: u32,
    pub shadow_alpha: f32,
    pub shadow_scale: f32,
    pub shadow_offset: [f32; 3],
    pub player_front: bool,
    pub over_player_alpha: f32,
    pub player_id: u32,
    pub closing: bool,
}

struct World {
    device: wgpu::Device,
    depth: wgpu::Texture,
    player: wgpu::Texture,
    player_depth: wgpu::Texture,
    color: wgpu::Texture,
    blur: wgpu::Texture,
    size: [u32; 2],
    camera: DVec3,
    view_proj: Mat4,
}
struct Target {
    device: wgpu::Device,
    texture: wgpu::Texture,
    depth: wgpu::Texture,
    size: [u32; 2],
    format: wgpu::TextureFormat,
}
#[derive(Default)]
struct State {
    presentation: Option<Presentation>,
    world: Option<World>,
    target: Option<Target>,
    drawn: u64,
}
thread_local! {
    static STATE:RefCell<State>=RefCell::new(State::default());
    static REENTRANT:Cell<bool>=const{Cell::new(false)};
    static PIPELINES:RefCell<Vec<(String,wgpu::Device,wgpu::BindGroupLayout,wgpu::RenderPipeline)>>=const{RefCell::new(Vec::new())};
    static SAMPLERS:RefCell<Vec<(bool,bool,wgpu::Device,wgpu::Sampler)>>=const{RefCell::new(Vec::new())};
}
pub fn publish(p: Option<Presentation>) {
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        if p.is_none() {
            s.target = None;
            s.world = None;
        }
        s.presentation = p;
    });
}
pub fn view() -> Option<(DVec3, Mat4, [u32; 2])> {
    STATE.with(|s| {
        s.borrow()
            .world
            .as_ref()
            .map(|w| (w.camera, w.view_proj, w.size))
    })
}
pub fn needs(needs: &mut RenderNeeds) {
    if STATE.with(|s| s.borrow().presentation.is_some()) {
        needs.world_depth = true;
        needs.entity_spans = true;
    }
}

fn texture(
    device: &wgpu::Device,
    size: [u32; 2],
    format: wgpu::TextureFormat,
    label: &str,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: size[0].max(1),
            height: size[1].max(1),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}
fn cached_pipeline(
    key: &str,
    device: &wgpu::Device,
    create: impl FnOnce() -> (wgpu::BindGroupLayout, wgpu::RenderPipeline),
) -> (wgpu::BindGroupLayout, wgpu::RenderPipeline) {
    PIPELINES.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some((_, _, layout, pipeline)) = cache
            .iter()
            .find(|(name, owner, _, _)| name == key && owner == device)
        {
            return (layout.clone(), pipeline.clone());
        }
        cache.retain(|(_, owner, _, _)| owner == device);
        let (layout, pipeline) = create();
        cache.push((key.into(), device.clone(), layout.clone(), pipeline.clone()));
        (layout, pipeline)
    })
}
fn sampler(device: &wgpu::Device, linear: bool, anisotropic: bool) -> wgpu::Sampler {
    SAMPLERS.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some((_, _, _, s)) = cache
            .iter()
            .find(|(l, a, owner, _)| *l == linear && *a == anisotropic && owner == device)
        {
            return s.clone();
        }
        cache.retain(|(_, _, owner, _)| owner == device);
        let filter = if linear {
            wgpu::FilterMode::Linear
        } else {
            wgpu::FilterMode::Nearest
        };
        let s = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Spatial GUI screen filtering"),
            mag_filter: filter,
            min_filter: filter,
            mipmap_filter: if linear {
                wgpu::MipmapFilterMode::Linear
            } else {
                wgpu::MipmapFilterMode::Nearest
            },
            anisotropy_clamp: if linear && anisotropic { 16 } else { 1 },
            ..Default::default()
        });
        cache.push((linear, anisotropic, device.clone(), s.clone()));
        s
    })
}

// A private depth snapshot is recorded before the hand and HUD reuse the world's
// depth attachment. It remains valid with dynamic resolution and shader packs.
pub fn world_drawn(frame: &mut WorldDrawn<'_>) {
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        let Some(p) = state.presentation.as_ref() else {
            return;
        };
        let player_id = p.player_id;
        let blur_strength = p.blur;
        let size = frame.size;
        let (scene, player, player_depth) = state
            .world
            .as_ref()
            .filter(|w| w.size == size && w.device == *frame.device)
            .map(|w| (w.depth.clone(), w.player.clone(), w.player_depth.clone()))
            .unwrap_or_else(|| {
                (
                    texture(
                        frame.device,
                        size,
                        wgpu::TextureFormat::R32Float,
                        "Spatial GUI world depth",
                    ),
                    texture(
                        frame.device,
                        size,
                        wgpu::TextureFormat::R32Float,
                        "Spatial GUI player depth",
                    ),
                    texture(
                        frame.device,
                        size,
                        wgpu::TextureFormat::Depth32Float,
                        "Spatial GUI avatar depth attachment",
                    ),
                )
            });
        let (color, blur) = state
            .world
            .as_ref()
            .filter(|w| {
                w.size == size
                    && w.device == *frame.device
                    && w.color.format() == frame.color_format
            })
            .map(|w| (w.color.clone(), w.blur.clone()))
            .unwrap_or_else(|| {
                (
                    texture(
                        frame.device,
                        size,
                        frame.color_format,
                        "Spatial GUI sharp world",
                    ),
                    texture(
                        frame.device,
                        size,
                        frame.color_format,
                        "Spatial GUI world blur",
                    ),
                )
            });
        let (layout, pipeline) = cached_pipeline("depth-copy", frame.device, || {
            let shader = frame
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Spatial GUI depth extraction"),
                    source: wgpu::ShaderSource::Wgsl(DEPTH_SHADER.into()),
                });
            let layout = frame
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Depth,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    }],
                });
            let pipeline_layout =
                frame
                    .device
                    .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: None,
                        bind_group_layouts: &[Some(&layout)],
                        immediate_size: 0,
                    });
            let pipeline = frame
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Spatial GUI depth copy"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("fullscreen"),
                        compilation_options: Default::default(),
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("copy_depth"),
                        compilation_options: Default::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::R32Float,
                            blend: None,
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: Default::default(),
                    depth_stencil: None,
                    multisample: Default::default(),
                    multiview_mask: None,
                    cache: None,
                });
            (layout, pipeline)
        });
        let group = frame.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(frame.depth),
            }],
        });
        let view = scene.create_view(&Default::default());
        {
            let mut pass = frame
                .encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Spatial GUI world depth snapshot"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &group, &[]);
            pass.draw(0..3, 0..1);
        }
        // Isolate the local avatar's actual posed geometry, rather than a screen
        // rectangle, so screen-over-player alpha follows arms, capes and elytra.
        let mut vertices: Vec<[f32; 6]> = Vec::new();
        let mut ranges = Vec::new();
        for (i, span) in frame.spans.iter().enumerate() {
            if span.player && !span.hidden {
                ranges.push(
                    span.start as usize
                        ..frame
                            .spans
                            .get(i + 1)
                            .map_or(frame.entities.len(), |v| v.start as usize),
                );
            }
        }
        if ranges.is_empty() {
            for (i, &(start, id)) in frame.motion.iter().enumerate() {
                if id == (7_u64 << 32) || id == ((1_u64 << 32) | u64::from(player_id)) {
                    ranges.push(
                        start as usize
                            ..frame
                                .motion
                                .get(i + 1)
                                .map_or(frame.entities.len(), |v| v.0 as usize),
                    );
                }
            }
        }
        for range in ranges {
            for quad in frame.entities
                [range.start.min(frame.entities.len())..range.end.min(frame.entities.len())]
                .chunks_exact(4)
            {
                for j in [0, 1, 2, 0, 2, 3] {
                    let q = frame.jittered * Vec3::from_array(quad[j].pos).extend(1.0);
                    vertices.push([q.x, q.y, q.z, q.w, quad[j].uv[0], quad[j].uv[1]]);
                }
            }
        }
        player_mask(
            &mut MaskDraw {
                device: frame.device,
                encoder: frame.encoder,
                atlas: frame.atlas,
            },
            &player,
            &player_depth,
            &vertices,
            false,
        );
        if blur_strength > 0.0 {
            frame.encoder.copy_texture_to_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: frame.color,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyTextureInfo {
                    texture: &color,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width: size[0],
                    height: size[1],
                    depth_or_array_layers: 1,
                },
            );
            blur_world(frame, &color, &blur, &scene, &player, blur_strength);
        }
        state.world = Some(World {
            device: frame.device.clone(),
            depth: scene,
            player,
            player_depth,
            color,
            blur,
            size,
            camera: frame.camera,
            view_proj: frame.view_proj,
        });
    });
}

struct MaskDraw<'a> {
    device: &'a wgpu::Device,
    encoder: &'a mut wgpu::CommandEncoder,
    atlas: Option<&'a wgpu::Texture>,
}

// The hand has its own projection and depth pass. Keep a separate sentinel in
// the same silhouette texture: comparing its depth with world depth is invalid.
pub fn hand_drawn(frame: &mut HandDrawn<'_>) {
    STATE.with(|cell| {
        let state = cell.borrow();
        let Some(world) = state.world.as_ref() else {
            return;
        };
        if frame.hand.is_empty() || world.size != frame.size {
            return;
        }
        let mut vertices = Vec::new();
        for quad in frame.hand.chunks_exact(4) {
            for j in [0, 1, 2, 0, 2, 3] {
                let q = frame.view_proj * Vec3::from_array(quad[j].pos).extend(1.0);
                vertices.push([q.x, q.y, q.z, q.w, quad[j].uv[0], quad[j].uv[1]]);
            }
        }
        player_mask(
            &mut MaskDraw {
                device: frame.device,
                encoder: frame.encoder,
                atlas: frame.atlas,
            },
            &world.player,
            &world.player_depth,
            &vertices,
            true,
        );
    });
}

fn player_mask(
    frame: &mut MaskDraw<'_>,
    player: &wgpu::Texture,
    player_depth: &wgpu::Texture,
    vertices: &[[f32; 6]],
    hand: bool,
) {
    let player_view = player.create_view(&Default::default());
    let (player_layout, player_pipeline) = cached_pipeline(
        if hand { "hand-mask" } else { "avatar-depth" },
        frame.device,
        || {
            let player_shader = frame
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(PLAYER_SHADER.into()),
                });
            let bindings =
                frame
                    .device
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: None,
                        entries: &[
                            wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Texture {
                                    sample_type: wgpu::TextureSampleType::Float {
                                        filterable: true,
                                    },
                                    view_dimension: wgpu::TextureViewDimension::D2,
                                    multisampled: false,
                                },
                                count: None,
                            },
                            wgpu::BindGroupLayoutEntry {
                                binding: 1,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                                count: None,
                            },
                        ],
                    });
            let layout = frame
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[Some(&bindings)],
                    immediate_size: 0,
                });
            let player_pipeline =
                frame
                    .device
                    .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("Spatial GUI avatar mask"),
                        layout: Some(&layout),
                        vertex: wgpu::VertexState {
                            module: &player_shader,
                            entry_point: Some("vertex"),
                            compilation_options: Default::default(),
                            buffers: &[Some(wgpu::VertexBufferLayout {
                                array_stride: 24,
                                step_mode: wgpu::VertexStepMode::Vertex,
                                attributes: &wgpu::vertex_attr_array![0=>Float32x4,1=>Float32x2],
                            })],
                        },
                        fragment: Some(wgpu::FragmentState {
                            module: &player_shader,
                            entry_point: Some(if hand { "hand" } else { "fragment" }),
                            compilation_options: Default::default(),
                            targets: &[Some(wgpu::ColorTargetState {
                                format: wgpu::TextureFormat::R32Float,
                                blend: None,
                                write_mask: wgpu::ColorWrites::ALL,
                            })],
                        }),
                        primitive: Default::default(),
                        depth_stencil: Some(wgpu::DepthStencilState {
                            format: wgpu::TextureFormat::Depth32Float,
                            depth_write_enabled: Some(true),
                            depth_compare: Some(wgpu::CompareFunction::GreaterEqual),
                            stencil: Default::default(),
                            bias: Default::default(),
                        }),
                        multisample: Default::default(),
                        multiview_mask: None,
                        cache: None,
                    });
            (bindings, player_pipeline)
        },
    );
    let atlas_view = frame.atlas.map(|t| t.create_view(&Default::default()));
    let nearest = sampler(frame.device, false, false);
    let player_group = atlas_view.as_ref().map(|view| {
        frame.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &player_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&nearest),
                },
            ],
        })
    });
    let buffer = frame
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(if vertices.is_empty() {
                &[[0.0; 6]][..]
            } else {
                vertices
            }),
            usage: wgpu::BufferUsages::VERTEX,
        });
    let player_depth_view = player_depth.create_view(&Default::default());
    {
        let mut pass = frame
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Spatial GUI avatar depth"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &player_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: if hand {
                            wgpu::LoadOp::Load
                        } else {
                            wgpu::LoadOp::Clear(wgpu::Color::BLACK)
                        },
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &player_depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
        if let Some(group) = &player_group {
            pass.set_pipeline(&player_pipeline);
            pass.set_bind_group(0, group, &[]);
            pass.set_vertex_buffer(0, buffer.slice(..));
            pass.draw(0..vertices.len() as u32, 0..1);
        }
    }
}

/// Returns true when this invocation was replaced by the extracted-screen pass.
pub fn draw(
    renderer: &HudRenderer,
    encoder: &mut wgpu::CommandEncoder,
    target: &wgpu::TextureView,
    coverage: Option<&wgpu::TextureView>,
    depth: &wgpu::TextureView,
    batch: &HudBatch,
    size: [u32; 2],
    range: Range<u32>,
) -> bool {
    if REENTRANT.with(Cell::get) {
        return false;
    }
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        let Some(p) = state.presentation.clone() else {
            return false;
        };
        if state.world.is_none() {
            return false;
        }
        let intersects = range.start < p.end && range.end > p.start;
        if !intersects && !p.closing {
            return false;
        }
        struct Guard;
        impl Drop for Guard {
            fn drop(&mut self) {
                REENTRANT.with(|g| g.set(false));
            }
        }
        REENTRANT.with(|g| g.set(true));
        let _guard = Guard;
        if p.closing {
            renderer.draw_range(
                encoder,
                target,
                coverage,
                depth,
                batch,
                size,
                range.clone(),
                None,
            );
        } else {
            let before = range.start..range.end.min(p.start);
            if before.start < before.end {
                renderer.draw_range(encoder, target, coverage, depth, batch, size, before, None);
            }
        }
        if state.drawn != p.frame {
            if !p.closing {
                let scale = p.render_scale.clamp(0.25, 1.0);
                let target_size = size.map(|v| ((v as f32 * scale).round() as u32).max(1));
                if state.target.as_ref().is_none_or(|t| {
                    t.size != target_size
                        || t.format != renderer.format
                        || t.device != state.world.as_ref().unwrap().device
                }) {
                    let device = &state.world.as_ref().unwrap().device;
                    state.target = Some(Target {
                        device: device.clone(),
                        texture: texture(
                            device,
                            target_size,
                            renderer.format,
                            "Spatial GUI screen texture",
                        ),
                        depth: texture(
                            device,
                            target_size,
                            wgpu::TextureFormat::Depth32Float,
                            "Spatial GUI screen depth",
                        ),
                        size: target_size,
                        format: renderer.format,
                    });
                }
                let texture = state.target.as_ref().unwrap();
                let view = texture.texture.create_view(&Default::default());
                let gui_depth = texture.depth.create_view(&Default::default());
                {
                    let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Spatial GUI transparent screen clear"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            depth_slice: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        ..Default::default()
                    });
                }
                let mut scaled = HudBatch::new(size);
                scaled.vertices = batch.vertices.clone();
                scaled.blends = batch
                    .blends
                    .iter()
                    .map(|(r, b)| {
                        (
                            r.clone(),
                            match b {
                                Blend::Depth(clip) => {
                                    Blend::Depth(clip.map(|v| (v as f32 * scale).round() as u32))
                                }
                                Blend::DepthGlint(clip) => Blend::DepthGlint(
                                    clip.map(|v| (v as f32 * scale).round() as u32),
                                ),
                                _ => *b,
                            },
                        )
                    })
                    .collect();
                scaled.game_time = batch.game_time;
                scaled.end_portal = batch.end_portal;
                scaled.glint_alpha = batch.glint_alpha;
                scaled.glint_colour = batch.glint_colour;
                scaled.pictures = batch.pictures.clone();
                renderer.draw_range(
                    encoder,
                    &view,
                    None,
                    &gui_depth,
                    &scaled,
                    target_size,
                    p.start..p.end,
                    None,
                );
            }
            if let (Some(world), Some(texture)) = (&state.world, &state.target) {
                composite(
                    world,
                    texture,
                    &p,
                    encoder,
                    target,
                    coverage,
                    renderer.format,
                );
            }
            state.drawn = p.frame;
        }
        if !p.closing && range.end > p.end {
            renderer.draw_range(
                encoder,
                target,
                coverage,
                depth,
                batch,
                size,
                range.start.max(p.end)..range.end,
                None,
            );
        }
        true
    })
}

fn composite(
    world: &World,
    screen: &Target,
    p: &Presentation,
    encoder: &mut wgpu::CommandEncoder,
    target: &wgpu::TextureView,
    coverage: Option<&wgpu::TextureView>,
    format: wgpu::TextureFormat,
) {
    let device = &world.device;
    let (layout, pipeline) = cached_pipeline(
        &format!("composite:{format:?}:{}", coverage.is_some()),
        device,
        || {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Spatial GUI world surface"),
                source: wgpu::ShaderSource::Wgsl(SURFACE_SHADER.into()),
            });
            let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 3,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 4,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(&layout)],
                immediate_size: 0,
            });
            let mut targets = vec![Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })];
            if coverage.is_some() {
                let over = wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::OneMinusSrc,
                    operation: wgpu::BlendOperation::Add,
                };
                targets.push(Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::R8Unorm,
                    blend: Some(wgpu::BlendState {
                        color: over,
                        alpha: over,
                    }),
                    write_mask: wgpu::ColorWrites::RED,
                }));
            }
            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Spatial GUI perspective composite"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[Some(wgpu::VertexBufferLayout {
                        array_stride: 24,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0=>Float32x4,1=>Float32x2],
                    })],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some(if coverage.is_some() {
                        "with_coverage"
                    } else {
                        "fragment"
                    }),
                    compilation_options: Default::default(),
                    targets: &targets,
                }),
                primitive: wgpu::PrimitiveState {
                    cull_mode: None,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            });
            (layout, pipeline)
        },
    );
    let sampler = sampler(device, p.linear, p.anisotropic);
    let screen_view = screen.texture.create_view(&Default::default());
    let scene_view = world.depth.create_view(&Default::default());
    let player_view = world.player.create_view(&Default::default());
    for layer in (0..=p.shadow_layers.min(6)).rev() {
        let shadow = layer != 0;
        let mut surface = p.surface;
        if shadow {
            let k = layer as f32;
            surface.center += (surface.rotation
                * Vec3::new(
                    p.shadow_offset[0] * k,
                    p.shadow_offset[1] * k,
                    -p.shadow_offset[2] * k,
                ))
            .as_dvec3();
            surface.width *= p.shadow_scale;
            surface.height *= p.shadow_scale;
        }
        let segments = if surface.arc.abs() > 0.001 { 96 } else { 1 };
        let mut vertices: Vec<[f32; 6]> = Vec::new();
        for i in 0..segments {
            let u0 = i as f32 / segments as f32;
            let u1 = (i + 1) as f32 / segments as f32;
            for uv in [
                Vec2::new(u0, 0.0),
                Vec2::new(u0, 1.0),
                Vec2::new(u1, 1.0),
                Vec2::new(u0, 0.0),
                Vec2::new(u1, 1.0),
                Vec2::new(u1, 0.0),
            ] {
                let clip =
                    world.view_proj * (surface.point(uv) - world.camera).as_vec3().extend(1.0);
                vertices.push([clip.x, clip.y, clip.z, clip.w, uv.x, uv.y]);
            }
        }
        let params = [
            p.tint[0],
            p.tint[1],
            p.tint[2],
            p.opacity,
            world.size[0] as f32,
            world.size[1] as f32,
            p.over_player_alpha,
            if p.player_front { 1.0 } else { 0.0 },
            if shadow { 1.0 } else { 0.0 },
            p.shadow_alpha,
            layer as f32,
            0.0,
        ];
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&params),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&screen_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&scene_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&player_view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: uniform.as_entire_binding(),
                },
            ],
        });
        let mut attachments = vec![Some(wgpu::RenderPassColorAttachment {
            view: target,
            resolve_target: None,
            depth_slice: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
        })];
        if let Some(view) = coverage {
            attachments.push(Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            }));
        }
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Spatial GUI world-space panel"),
            color_attachments: &attachments,
            ..Default::default()
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.set_vertex_buffer(0, buffer.slice(..));
        pass.draw(0..vertices.len() as u32, 0..1);
    }
}

fn blur_world(
    frame: &mut WorldDrawn<'_>,
    original: &wgpu::Texture,
    temp: &wgpu::Texture,
    scene: &wgpu::Texture,
    player: &wgpu::Texture,
    strength: f32,
) {
    let device = frame.device;
    let (layout, pipeline) = cached_pipeline(
        &format!("world-blur:{:?}", frame.color_format),
        device,
        || {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Spatial GUI world-only blur"),
                source: wgpu::ShaderSource::Wgsl(BLUR_SHADER.into()),
            });
            let mut entries = Vec::new();
            for binding in [0, 2] {
                entries.push(wgpu::BindGroupLayoutEntry {
                    binding,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                });
            }
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            });
            for binding in [3, 4] {
                entries.push(wgpu::BindGroupLayoutEntry {
                    binding,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                });
            }
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: 5,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            });
            let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &entries,
            });
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(&layout)],
                immediate_size: 0,
            });
            let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Spatial GUI Gaussian world blur"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fragment"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: frame.color_format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview_mask: None,
                cache: None,
            });
            (layout, pipeline)
        },
    );
    let linear = sampler(device, true, false);
    let original_view = original.create_view(&Default::default());
    let temp_view = temp.create_view(&Default::default());
    let scene_view = scene.create_view(&Default::default());
    let player_view = player.create_view(&Default::default());
    for horizontal in [true, false] {
        let params = [
            frame.size[0] as f32,
            frame.size[1] as f32,
            strength.clamp(0.0, 20.0),
            if horizontal { 1.0 } else { 0.0 },
        ];
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&params),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let source = if horizontal {
            &original_view
        } else {
            &temp_view
        };
        let destination = if horizontal {
            &temp_view
        } else {
            frame.color_view
        };
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(source),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&linear),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&original_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&scene_view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::TextureView(&player_view),
                },
                wgpu::BindGroupEntry {
                    binding: 5,
                    resource: uniform.as_entire_binding(),
                },
            ],
        });
        let mut pass = frame
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Spatial GUI world blur pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: destination,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.draw(0..3, 0..1);
    }
}
const BLUR_SHADER: &str = r#"
@group(0) @binding(0) var image:texture_2d<f32>;
@group(0) @binding(1) var linear:sampler;
@group(0) @binding(2) var sharp:texture_2d<f32>;
@group(0) @binding(3) var scene:texture_2d<f32>;
@group(0) @binding(4) var player:texture_2d<f32>;
@group(0) @binding(5) var<uniform> params:vec4<f32>;
struct Out{@builtin(position) p:vec4<f32>,@location(0) uv:vec2<f32>}
@vertex fn vertex(@builtin(vertex_index) i:u32)->Out{let p=array<vec2<f32>,3>(vec2(-1.,-1.),vec2(3.,-1.),vec2(-1.,3.))[i];var o:Out;o.p=vec4(p,0.,1.);o.uv=vec2(p.x*.5+.5,.5-p.y*.5);return o;}
@fragment fn fragment(o:Out)->@location(0) vec4<f32>{
 let direction=select(vec2(0.,params.z/params.y),vec2(params.z/params.x,0.),params.w>0.);
 var c=textureSampleLevel(image,linear,o.uv,0.)*.227027;
 c+=(textureSampleLevel(image,linear,o.uv+direction*1.384615,0.)+textureSampleLevel(image,linear,o.uv-direction*1.384615,0.))*.316216;
 c+=(textureSampleLevel(image,linear,o.uv+direction*3.230769,0.)+textureSampleLevel(image,linear,o.uv-direction*3.230769,0.))*.070270;
 if params.w<.5{let pixel=clamp(vec2<i32>(o.p.xy),vec2(0),vec2<i32>(textureDimensions(scene))-vec2(1));let d=textureLoad(scene,pixel,0).r;let avatar=textureLoad(player,pixel,0).r;if avatar>0.&&abs(avatar-d)<.0001{return textureSampleLevel(sharp,linear,o.uv,0.);}}
  return vec4(c.rgb,textureSampleLevel(sharp,linear,o.uv,0.).a);
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_opacity_applies_to_avatar_and_first_person_hand() {
        let instance = wgpu::Instance::default();
        let adapter = pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
        let (device, queue) =
            pollster::block_on(adapter.request_device(&Default::default())).unwrap();
        let size = [16, 16];
        let rgba = wgpu::TextureFormat::Rgba8Unorm;
        let image = texture(&device, size, rgba, "overlap test GUI");
        let atlas = texture(&device, [1, 1], rgba, "overlap test atlas");
        let write = |texture: &wgpu::Texture, data: &[u8], bytes_per_row, size: [u32; 2]| {
            queue.write_texture(
                texture.as_image_copy(),
                data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: None,
                },
                wgpu::Extent3d {
                    width: size[0],
                    height: size[1],
                    depth_or_array_layers: 1,
                },
            );
        };
        write(&image, &[255, 0, 0, 255].repeat(256), 64, size);
        write(&atlas, &[255; 4], 4, [1, 1]);
        let world = World {
            device: device.clone(),
            depth: texture(
                &device,
                size,
                wgpu::TextureFormat::R32Float,
                "test scene depth",
            ),
            player: texture(
                &device,
                size,
                wgpu::TextureFormat::R32Float,
                "test avatar mask",
            ),
            player_depth: texture(
                &device,
                size,
                wgpu::TextureFormat::Depth32Float,
                "test mask depth",
            ),
            color: texture(&device, size, rgba, "test sharp"),
            blur: texture(&device, size, rgba, "test blur"),
            size,
            camera: DVec3::ZERO,
            view_proj: Mat4::IDENTITY,
        };
        let screen = Target {
            device: device.clone(),
            texture: image,
            depth: texture(
                &device,
                size,
                wgpu::TextureFormat::Depth32Float,
                "test GUI depth",
            ),
            size,
            format: rgba,
        };
        let mut p = Presentation {
            frame: 1,
            start: 0,
            end: 6,
            surface: Surface {
                center: DVec3::new(0.0, 0.0, 0.5),
                rotation: glam::Quat::IDENTITY,
                width: 2.0,
                height: 2.0,
                arc: 0.0,
                gui: [16.0; 2],
            },
            opacity: 1.0,
            blur: 0.0,
            tint: [1.0; 3],
            render_scale: 1.0,
            linear: false,
            anisotropic: false,
            shadow_layers: 0,
            shadow_alpha: 0.0,
            shadow_scale: 1.0,
            shadow_offset: [0.0; 3],
            player_front: true,
            over_player_alpha: 50.0 / 255.0,
            player_id: 0,
            closing: false,
        };
        let output = texture(&device, size, rgba, "overlap test output");
        let view = output.create_view(&Default::default());
        let coverage = texture(
            &device,
            size,
            wgpu::TextureFormat::R8Unorm,
            "test HDR coverage",
        );
        let coverage_view = coverage.create_view(&Default::default());
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 256 * 32,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        // The hand's depth (.2) deliberately differs from the scene's (.8):
        // it must use its own projection/mask, not a world-depth comparison.
        for hdr in [false, true] {
            for (hand, front, alpha, scene_depth, expected) in [
                (false, true, 0, 0.8, 0),
                (false, true, 50, 0.8, 50),
                (false, true, 255, 0.8, 255),
                (false, false, 0, 0.8, 255),
                (true, true, 0, 0.8, 0),
                (true, true, 50, 0.8, 50),
                (true, true, 255, 0.8, 255),
                (false, true, 255, 0.9, 0),
            ] {
                p.player_front = front;
                p.over_player_alpha = alpha as f32 / 255.0;
                write(
                    &world.depth,
                    bytemuck::cast_slice(&[scene_depth as f32; 256]),
                    64,
                    size,
                );
                let z = if hand { 0.2 } else { 0.8 };
                let vertices = [
                    [-1., -1., z, 1., 0., 0.],
                    [1., -1., z, 1., 1., 0.],
                    [1., 1., z, 1., 1., 1.],
                    [-1., -1., z, 1., 0., 0.],
                    [1., 1., z, 1., 1., 1.],
                    [-1., 1., z, 1., 0., 1.],
                ];
                let mut encoder = device.create_command_encoder(&Default::default());
                // First clear the world mask, then add a hand without erasing it.
                player_mask(
                    &mut MaskDraw {
                        device: &device,
                        encoder: &mut encoder,
                        atlas: Some(&atlas),
                    },
                    &world.player,
                    &world.player_depth,
                    if hand { &[] } else { &vertices },
                    false,
                );
                if hand {
                    player_mask(
                        &mut MaskDraw {
                            device: &device,
                            encoder: &mut encoder,
                            atlas: Some(&atlas),
                        },
                        &world.player,
                        &world.player_depth,
                        &vertices,
                        true,
                    );
                }
                {
                    let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[
                            Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                depth_slice: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                    store: wgpu::StoreOp::Store,
                                },
                            }),
                            Some(wgpu::RenderPassColorAttachment {
                                view: &coverage_view,
                                resolve_target: None,
                                depth_slice: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                                    store: wgpu::StoreOp::Store,
                                },
                            }),
                        ],
                        ..Default::default()
                    });
                }
                composite(
                    &world,
                    &screen,
                    &p,
                    &mut encoder,
                    &view,
                    hdr.then_some(&coverage_view),
                    rgba,
                );
                encoder.copy_texture_to_buffer(
                    output.as_image_copy(),
                    wgpu::TexelCopyBufferInfo {
                        buffer: &readback,
                        layout: wgpu::TexelCopyBufferLayout {
                            offset: 0,
                            bytes_per_row: Some(256),
                            rows_per_image: None,
                        },
                    },
                    wgpu::Extent3d {
                        width: 16,
                        height: 16,
                        depth_or_array_layers: 1,
                    },
                );
                if hdr {
                    encoder.copy_texture_to_buffer(
                        coverage.as_image_copy(),
                        wgpu::TexelCopyBufferInfo {
                            buffer: &readback,
                            layout: wgpu::TexelCopyBufferLayout {
                                offset: 4096,
                                bytes_per_row: Some(256),
                                rows_per_image: None,
                            },
                        },
                        wgpu::Extent3d {
                            width: 16,
                            height: 16,
                            depth_or_array_layers: 1,
                        },
                    );
                }
                queue.submit([encoder.finish()]);
                readback
                    .slice(..)
                    .map_async(wgpu::MapMode::Read, |result| result.unwrap());
                device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
                {
                    let data = readback.slice(..).get_mapped_range().unwrap();
                    let red = data[8 * 256 + 8 * 4];
                    assert!(
                        (i32::from(red) - expected).abs() <= 1,
                        "hand={hand}, player_front={front}, alpha={alpha}: red={red}"
                    );
                    if hdr {
                        let covered = data[4096 + 8 * 256 + 8];
                        assert!(
                            (i32::from(covered) - expected).abs() <= 1,
                            "HDR coverage={covered}, expected={expected}"
                        );
                    }
                }
                readback.unmap();
            }
        }
    }
}
const DEPTH_SHADER: &str = r#"
@group(0) @binding(0) var scene:texture_depth_2d;
struct Out{@builtin(position) p:vec4<f32>,@location(0) uv:vec2<f32>}
@vertex fn fullscreen(@builtin(vertex_index) i:u32)->Out{
 let p=array<vec2<f32>,3>(vec2(-1.,-1.),vec2(3.,-1.),vec2(-1.,3.))[i];var o:Out;o.p=vec4(p,0.,1.);o.uv=vec2(p.x*.5+.5,.5-p.y*.5);return o;
}
@fragment fn copy_depth(o:Out)->@location(0) f32{
 let s=vec2<i32>(textureDimensions(scene));let p=clamp(vec2<i32>(o.uv*vec2<f32>(s)),vec2(0),s-vec2(1));return textureLoad(scene,p,0);
}
"#;
const PLAYER_SHADER: &str = r#"
@group(0) @binding(0) var atlas:texture_2d<f32>;
@group(0) @binding(1) var nearest:sampler;
struct Out{@builtin(position) p:vec4<f32>,@location(0) uv:vec2<f32>}
@vertex fn vertex(@location(0) p:vec4<f32>,@location(1) uv:vec2<f32>)->Out{var o:Out;o.p=p;o.uv=uv;return o;}
@fragment fn fragment(o:Out)->@location(0) f32{if textureSampleLevel(atlas,nearest,o.uv,0.).a<0.1{discard;}return o.p.z;}
@fragment fn hand(o:Out)->@location(0) f32{if textureSampleLevel(atlas,nearest,o.uv,0.).a<0.1{discard;}return 2.;}
"#;
const SURFACE_SHADER: &str = r#"
@group(0) @binding(0) var screen:texture_2d<f32>;
@group(0) @binding(1) var filtered:sampler;
@group(0) @binding(2) var scene:texture_2d<f32>;
@group(0) @binding(3) var player:texture_2d<f32>;
struct Params{tint:vec4<f32>,size:vec4<f32>,shadow:vec4<f32>}
@group(0) @binding(4) var<uniform> params:Params;
struct Out{@builtin(position) p:vec4<f32>,@location(0) uv:vec2<f32>}
@vertex fn vertex(@location(0) p:vec4<f32>,@location(1) uv:vec2<f32>)->Out{var o:Out;o.p=p;o.uv=uv;return o;}
fn shade(o:Out)->vec4<f32>{
 let pixel=clamp(vec2<i32>(o.p.xy),vec2(0),vec2<i32>(textureDimensions(scene))-vec2(1));
 let d=textureLoad(scene,pixel,0).r;let avatar=textureLoad(player,pixel,0).r;
  var occluded=false;var factor=1.;if avatar>1.||(avatar>0.&&abs(avatar-d)<0.0001){if params.size.w>0.{factor=params.size.z;}}else if o.p.z+0.00001<d{occluded=true;}
 var c=textureSample(screen,filtered,o.uv);
 if params.shadow.x>0.{let texel=1./vec2<f32>(textureDimensions(screen));var a=c.a;
  a+=textureSample(screen,filtered,o.uv+vec2(texel.x*params.shadow.z,0.)).a;
  a+=textureSample(screen,filtered,o.uv-vec2(texel.x*params.shadow.z,0.)).a;
  a+=textureSample(screen,filtered,o.uv+vec2(0.,texel.y*params.shadow.z)).a;
  a+=textureSample(screen,filtered,o.uv-vec2(0.,texel.y*params.shadow.z)).a;c=vec4(0.,0.,0.,a*.2*params.shadow.y);
 }else{c=vec4(c.rgb*params.tint.rgb,c.a);}
  c*=params.tint.a*factor;if occluded||c.a<0.001{discard;}return c;
}
@fragment fn fragment(o:Out)->@location(0) vec4<f32>{return shade(o);}
struct Covered{@location(0) color:vec4<f32>,@location(1) coverage:f32}
@fragment fn with_coverage(o:Out)->Covered{let c=shade(o);var result:Covered;result.color=c;result.coverage=c.a;return result;}
"#;

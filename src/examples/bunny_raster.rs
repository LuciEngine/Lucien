use anyhow::Result;
use glam::{vec3, Vec3};
use image::{gif::GifEncoder, Delay, Frame, Rgba};

use crate::application::EngineApp;
use crate::graphics::raster::{Program, Raster};
use crate::graphics::*;

use slog::info;

pub fn render(app: &EngineApp) -> Result<Vec<u8>> {
    let file = app.loader().load_text("uniform.json")?;
    let render_type = RenderType::Png;
    let primitive_type = PrimitiveType::Triangle;
    let mut uniform: UniformAttributes = serde_json::from_str(&file.as_str())?;
    let height: usize = 500;
    let mut frame_buffer = FrameBuffer::new(
        (height as f32 * uniform.camera.aspect_ratio) as usize,
        height,
    );
    uniform.calc_matrices();

    let vertex_shader =
        Box::new(
            |va: VertexAttributes, uniform: UniformAttributes| VertexAttributes {
                position: uniform.projection_matrix
                    * uniform.view_matrix
                    * uniform.model_matrix
                    * va.position,
                normal: (uniform.normal_matrix * va.normal.extend(0.0)).into(),
                frag_pos: (uniform.model_matrix * va.position).into(),
            },
        );
    info!(app.logger, "created vertex shader.");

    let fragment_shader = Box::new(|va: VertexAttributes, uniform: UniformAttributes| {
        let n = va.normal;
        let v = match uniform.camera.is_perspective {
            true => (uniform.camera.position - va.frag_pos).normalize(),
            false => vec3(0.0, 0.0, -1.0),
        };
        let li: Vec3 = (uniform.light.position - va.frag_pos).normalize();
        let diffuse = uniform.material.diffuse_color * li.dot(n).max(0.0);
        let specular = uniform.material.specular_color
            * n.dot((li + v).normalize())
                .max(0.0)
                .powf(uniform.material.shininess);
        let d = uniform.light.position - va.frag_pos;
        let color = (diffuse + specular) * uniform.light.intensity / d.length_squared();
        FragmentAttributes::new(color.extend(1.0), va.position.into(), n)
    });
    info!(app.logger, "created fragment shader.");

    let blending_shader = Box::new(|fa: FragmentAttributes, previous: FrameBufferAttributes| {
        let alpha = fa.color[3];
        let out = fa.color * alpha + previous.get_color() * (1.0 - alpha);
        if fa.position.z < previous.depth {
            FrameBufferAttributes {
                color: Rgba([
                    (out[0] * 255.0) as u8,
                    (out[1] * 255.0) as u8,
                    (out[2] * 255.0) as u8,
                    (out[3] * 255.0) as u8,
                ]),
                depth: fa.position.z,
            }
        } else {
            previous
        }
    });
    info!(app.logger, "created blending shader.");

    let program = Program {
        vertex_shader,
        fragment_shader,
        blending_shader,
    };

    let mesh = app.loader().load_off("bunny.off")?;
    let vertices: Vec<VertexAttributes> = mesh.as_vertex_attrs()?;
    let indices: Vec<usize> = mesh.indices.iter().map(|&i| i as usize).collect();

    match render_type {
        RenderType::Png => {
            info!(app.logger, "render mode: png.");

            program.render(
                &uniform,
                &vertices,
                &indices,
                &mut frame_buffer,
                primitive_type,
            )?;
            let out = app.path(".").unwrap().join("img/bunny.png");
            frame_buffer.render().save(out)?;

            Ok(frame_buffer.as_raw())
        }
        RenderType::Gif => {
            info!(app.logger, "render mode: gif.");

            let out = app.path(".").unwrap().join("img/bunny.gif");
            let gif = std::fs::File::create(out)?;
            let mut encoder = GifEncoder::new(gif);

            let mut frames: Vec<Frame> = vec![];
            let delay = Delay::from_saturating_duration(std::time::Duration::from_millis(75));
            for _ in 0..20 {
                uniform.transform.angle += 0.1;
                uniform.transform.distance -= 0.02;
                uniform.calc_matrices();
                frame_buffer.clear();
                program.render(
                    &uniform,
                    &vertices,
                    &indices,
                    &mut frame_buffer,
                    primitive_type,
                )?;
                let frame = Frame::from_parts(frame_buffer.render(), 0, 0, delay);
                frames.push(frame);
            }
            encoder.encode_frames(frames)?;

            Ok(frame_buffer.as_raw())
        }
    }
}

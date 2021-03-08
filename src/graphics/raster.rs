use crate::graphics::*;

use anyhow::{Context, Result};
use glam::{vec2, vec3, Mat3, Vec3};

pub trait Raster {
    fn render_triangle(
        &self,
        uniform: &UniformAttributes,
        v1: &VertexAttributes,
        v2: &VertexAttributes,
        v3: &VertexAttributes,
        frame_buffer: &mut FrameBuffer,
    ) -> Result<()>;

    fn render_triangles(
        &self,
        uniform: &UniformAttributes,
        vertices: &[VertexAttributes],
        indices: &[usize],
        frame_buffer: &mut FrameBuffer,
    ) -> Result<()>;

    fn render_line(
        &self,
        uniform: &UniformAttributes,
        v1: &VertexAttributes,
        v2: &VertexAttributes,
        line_thickness: f32,
        frame_buffer: &mut FrameBuffer,
    ) -> Result<()>;

    fn render_lines(
        &self,
        uniform: &UniformAttributes,
        vertices: &[VertexAttributes],
        indices: &[usize],
        line_thickness: f32,
        frame_buffer: &mut FrameBuffer,
    ) -> Result<()>;

    fn render(
        &self,
        uniform: &UniformAttributes,
        vertices: &[VertexAttributes],
        indices: &[usize],
        frame_buffer: &mut FrameBuffer,
        primitive_type: PrimitiveType,
    ) -> Result<()>;
}

// Raster program converts vertex -> fragment -> frame buffer
pub struct Program {
    pub vertex_shader: Box<dyn Fn(VertexAttributes, UniformAttributes) -> VertexAttributes>,
    pub fragment_shader: Box<dyn Fn(VertexAttributes, UniformAttributes) -> FragmentAttributes>,
    pub blending_shader:
        Box<dyn Fn(FragmentAttributes, FrameBufferAttributes) -> FrameBufferAttributes>,
}

// They don't actually render, the data are processed into frame buffer
impl Raster for Program {
    fn render_triangle(
        &self,
        uniform: &UniformAttributes,
        v1: &VertexAttributes,
        v2: &VertexAttributes,
        v3: &VertexAttributes,
        frame_buffer: &mut FrameBuffer,
    ) -> Result<()> {
        let mut p = vec![];
        p.push(v1.position / v1.position.w);
        p.push(v2.position / v2.position.w);
        p.push(v3.position / v3.position.w);

        p.iter_mut().for_each(|point| {
            point[0] = ((point[0] + 1.0) / 2.0) * frame_buffer.width() as f32;
            point[1] = ((point[1] + 1.0) / 2.0) * frame_buffer.height() as f32;
        });

        let f32_cmp = |a: &f32, b: &f32| a.partial_cmp(b).unwrap();
        let x: Vec<f32> = p.iter().map(|point| point[0]).collect();
        let y: Vec<f32> = p.iter().map(|point| point[1]).collect();

        let lx = x
            .iter()
            .cloned()
            .min_by(f32_cmp)
            .context("min error")?
            .max(0.0) as u32;
        let ly = y
            .iter()
            .cloned()
            .min_by(f32_cmp)
            .context("min error")?
            .max(0.0) as u32;
        let ux = x
            .into_iter()
            .max_by(f32_cmp)
            .context("max error")?
            .min(frame_buffer.width() as f32 - 1.0) as u32;
        let uy = y
            .into_iter()
            .max_by(f32_cmp)
            .context("max error")?
            .min(frame_buffer.height() as f32 - 1.0) as u32;

        let mut a = Mat3::ZERO;
        a.x_axis = p[0].truncate();
        a.y_axis = p[1].truncate();
        a.z_axis = p[2].truncate();
        a = a.transpose();
        a.z_axis = Vec3::ONE;
        a = a.transpose();

        let ai = a.inverse();

        for i in lx..=ux {
            for j in ly..=uy {
                let pixel = vec3(i as f32 + 0.5, j as f32 + 0.5, 1.0);
                let b = ai * pixel;
                if b.min_element() >= 0.0 {
                    let va = VertexAttributes::interpolate(*v1, *v2, *v3, b.x, b.y, b.z);
                    if va.position[2] >= -1.0 && va.position[2] <= 1.0 {
                        let frag = (self.fragment_shader)(va, *uniform);
                        let h = frame_buffer.height() - 1;
                        frame_buffer.set(
                            i as usize,
                            h - j as usize,
                            (self.blending_shader)(
                                frag,
                                frame_buffer.get(i as usize, h - j as usize),
                            ),
                        );
                    }
                }
            }
        }
        Ok(())
    }

    fn render_triangles(
        &self,
        uniform: &UniformAttributes,
        vertices: &[VertexAttributes],
        indices: &[usize],
        frame_buffer: &mut FrameBuffer,
    ) -> Result<()> {
        for i in 0..indices.len() / 3 {
            self.render_triangle(
                uniform,
                &vertices[indices[i * 3]],
                &vertices[indices[i * 3 + 1]],
                &vertices[indices[i * 3 + 2]],
                frame_buffer,
            )?;
        }
        Ok(())
    }

    fn render_line(
        &self,
        uniform: &UniformAttributes,
        v1: &VertexAttributes,
        v2: &VertexAttributes,
        line_thickness: f32,
        frame_buffer: &mut FrameBuffer,
    ) -> Result<()> {
        let mut p = vec![];
        p.push(v1.position / v1.position.w);
        p.push(v2.position / v2.position.w);

        p.iter_mut().for_each(|point| {
            point[0] = ((point[0] + 1.0) / 2.0) * frame_buffer.width() as f32;
            point[1] = ((point[1] + 1.0) / 2.0) * frame_buffer.height() as f32;
        });

        let f32_cmp = |a: &f32, b: &f32| a.partial_cmp(b).unwrap();
        let x: Vec<f32> = p.iter().map(|point| point[0]).collect();
        let y: Vec<f32> = p.iter().map(|point| point[1]).collect();

        let lx = (x.iter().cloned().min_by(f32_cmp).context("min error")? - line_thickness).max(0.0)
            as u32;
        let ly = (y.iter().cloned().min_by(f32_cmp).context("max error")? - line_thickness).max(0.0)
            as u32;
        let ux = (x.into_iter().max_by(f32_cmp).context("max error")? + line_thickness)
            .min(frame_buffer.width() as f32 - 1.0) as u32;
        let uy = (y.into_iter().max_by(f32_cmp).context("min error")? + line_thickness)
            .min(frame_buffer.height() as f32 - 1.0) as u32;

        let l1 = vec2(p[0][0], p[0][1]);
        let l2 = vec2(p[1][0], p[1][1]);

        let ll = (l1 - l2).length_squared();

        for i in lx..=ux {
            for j in ly..=uy {
                let pixel = vec2(i as f32 + 0.5, j as f32 + 0.5);
                let t = if ll == 0.0 {
                    0.0
                } else {
                    ((pixel - l1).dot(l2 - l1) / ll).min(1.0).max(0.0)
                };

                let pixel_p = l1 + t * (l2 - l1);

                if (pixel - pixel_p).length_squared() < line_thickness * line_thickness {
                    let va = VertexAttributes::interpolate(*v1, *v2, *v1, 1.0 - t, t, 0.0);
                    let frag = (self.fragment_shader)(va, *uniform);
                    let h = frame_buffer.height() - 1;
                    frame_buffer.set(
                        i as usize,
                        h - j as usize,
                        (self.blending_shader)(frag, frame_buffer.get(i as usize, h - j as usize)),
                    );
                }
            }
        }
        Ok(())
    }

    fn render_lines(
        &self,
        uniform: &UniformAttributes,
        vertices: &[VertexAttributes],
        indices: &[usize],
        line_thickness: f32,
        frame_buffer: &mut FrameBuffer,
    ) -> Result<()> {
        for i in 0..indices.len() / 2 {
            self.render_line(
                uniform,
                &vertices[indices[i * 2]],
                &vertices[indices[i * 2 + 1]],
                line_thickness,
                frame_buffer,
            )?;
        }
        Ok(())
    }

    fn render(
        &self,
        uniform: &UniformAttributes,
        vertices: &[VertexAttributes],
        indices: &[usize],
        frame_buffer: &mut FrameBuffer,
        primitive_type: PrimitiveType,
    ) -> Result<()> {
        match primitive_type {
            PrimitiveType::Triangle => {
                self.render_triangles(uniform, vertices, indices, frame_buffer)
            }
            PrimitiveType::Line => self.render_lines(uniform, vertices, indices, 0.5, frame_buffer),
        }
    }
}

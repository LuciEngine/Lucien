use bytemuck::{Pod, Zeroable};
use glam::{vec3, Mat4, Vec3};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub eye: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub view_proj: [[f32; 4]; 4],
}

unsafe impl Pod for Camera {}
unsafe impl Zeroable for Camera {}

impl Camera {
    // By default, camera always look at `look_at`
    pub fn new(eye: Vec3, aspect_ratio: f32) -> Self {
        let look_at = Vec3::ZERO;
        let up = Vec3::Y;
        let fov = 0.7;
        let z_near = 0.1;
        let z_far = 100.0;

        let mut camera = Camera {
            eye,
            look_at,
            up,
            aspect_ratio,
            fov,
            z_near,
            z_far,
            view_proj: Mat4::ZERO.to_cols_array_2d(),
        };
        camera.update_view_matrix();
        camera
    }

    pub fn update_view_matrix(&mut self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.look_at, self.up);
        let proj = Mat4::perspective_rh(self.fov, self.aspect_ratio, self.z_near, self.z_far);
        let view_proj = proj * view;
        self.view_proj = view_proj.to_cols_array_2d();

        view_proj
    }

    pub fn direction(&self) -> Vec3 {
        self.look_at - self.eye
    }
}

impl Default for Camera {
    fn default() -> Self {
        let eye = vec3(0.0, 1.0, 2.0);
        let aspect_ratio = 16.0 / 9.0;
        Camera::new(eye, aspect_ratio)
    }
}

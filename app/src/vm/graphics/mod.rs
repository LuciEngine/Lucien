pub mod math;
pub use math::WrenVec3;

pub mod light;
pub use light::*;

use glam::vec3;
use ruwren::{get_slot_checked, send_foreign, Class, VM};

pub struct Graphics;

// todo properly raise runtime exception in foreign method
impl Class for Graphics {
    fn initialize(_: &VM) -> Self {
        panic!("Graphics is a purely static class");
    }
}

impl Graphics {
    // create a vector3
    // accepts 3 numbers as params
    pub fn new_vec3(vm: &VM) {
        let x = get_slot_checked!(vm => num 1);
        let y = get_slot_checked!(vm => num 2);
        let z = get_slot_checked!(vm => num 3);
        let vec = WrenVec3(vec3(x as f32, y as f32, z as f32));

        send_foreign!(vm, "graphics", "Vec3", vec => 0);
    }

    // create a point light in current scene
    // accepts 2 vec3 as params
    pub fn new_light(vm: &VM) {
        let position = *get_slot_checked!(vm => foreign WrenVec3 => 1);
        let color = *get_slot_checked!(vm => foreign WrenVec3 => 2);

        send_foreign!(vm, "graphics", "Light", Light { position, color } => 0);
    }

    // pub fn new_point_light(_vm: &VM) {
    //     // get wgpu device
    //     let lock = crate::application::GLOB.lock();
    //     let glob = lock.as_ref().unwrap();
    //
    //     println!("{:?}", glob)
    // }
}

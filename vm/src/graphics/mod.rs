// pub mod light;
use ruwren::{get_slot_checked, send_foreign, Class, VM};
use glam::{vec3, Vec3};

#[derive(Debug)]
pub struct Light {
    position: WrenVec3,
    color: WrenVec3,
}
// todo properly raise runtime exception in foreign method
impl Class for Light {
    fn initialize(_: &VM) -> Self {
        panic!("Cannot init from wren");
    }
}
// foreign methods for wren,
// we don't return values to Rust, but push the results
// to wren vm stack.
impl Light {
    pub fn position(&self, vm: &VM) {
        send_foreign!(vm, "graphics", "Vec3", self.position => 0);
    }

    pub fn color(&self, vm: &VM) {
        send_foreign!(vm, "graphics", "Vec3", self.color => 0);
    }

    // need to be manually implemented so that the module macro can work
    pub fn fmt(&self, vm: &VM) {
        vm.set_slot_string(0, format!("{:?}", self));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WrenVec3(Vec3);

impl Class for WrenVec3 {
    fn initialize(_: &VM) -> Self {
        panic!("Cannot init from wren");
    }
}
impl WrenVec3 {
    pub fn fmt(&self, vm: &VM) {
        vm.set_slot_string(0, format!("{:?}", self));
    }
}

pub struct Graphics;

// todo properly raise runtime exception in foreign method
impl Class for Graphics {
    fn initialize(_: &VM) -> Self {
        panic!("Graphics is a purely static class");
    }
}

impl Graphics {
    // create a point light in current scene
    pub fn new_light(vm: &VM) {
        let x = get_slot_checked!(vm => num 1);
        let y = get_slot_checked!(vm => num 2);
        let z = get_slot_checked!(vm => num 3);
        let position = WrenVec3(vec3(x as f32, y as f32, z as f32));
        let color = WrenVec3(vec3(0.3, 0.5, 0.3));
        send_foreign!(vm, "graphics", "Light", Light { position, color } => 0);
    }
}

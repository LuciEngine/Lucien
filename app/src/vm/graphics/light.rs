use crate::vm::graphics::WrenVec3;
use ruwren::{send_foreign, Class, VM};

use lucien_render::PointLight;

#[derive(Debug)]
pub struct Light {
    pub position: WrenVec3,
    pub color: WrenVec3,
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
    // getter that returns light position
    pub fn position(&self, vm: &VM) {
        send_foreign!(vm, "graphics", "Vec3", self.position => 0);
    }

    // getter that returns light color
    pub fn color(&self, vm: &VM) {
        send_foreign!(vm, "graphics", "Vec3", self.color => 0);
    }

    // getter that returns a formatted string
    // need to be manually implemented so that the module macro can work
    pub fn fmt(&self, vm: &VM) {
        vm.set_slot_string(0, format!("{:?}", self));
    }
}

#[derive(Debug)]
pub struct WrenPointLight(pub PointLight);

impl Class for WrenPointLight {
    fn initialize(_: &VM) -> Self {
        panic!("Cannot init from wren");
    }
}

impl WrenPointLight {
    pub fn position(&self, vm: &VM) {
        send_foreign!(vm, "graphics", "Vec3", WrenVec3(self.0.position) => 0);
    }

    pub fn color(&self, vm: &VM) {
        send_foreign!(vm, "graphics", "Vec3", WrenVec3(self.0.color) => 0);
    }
}

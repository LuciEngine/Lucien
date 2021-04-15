use crate::graphics::WrenVec3;
use ruwren::{send_foreign, Class, VM};

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

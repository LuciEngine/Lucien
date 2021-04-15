use glam::Vec3;
use ruwren::{Class, VM};

#[derive(Debug, Clone, Copy)]
pub struct WrenVec3(pub Vec3);

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

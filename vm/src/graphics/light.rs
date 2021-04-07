use ruwren::{create_module, get_slot_checked, send_foreign, Class, ModuleLibrary, VM};
use glam::{vec3, Vec3};

#[derive(Debug, Clone)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f32,
}

// todo properly create runtime exception in foreign method
impl Class for PointLight {
    fn initialize(_: &VM) -> Self {
        panic!("Cannot initialize from Wren code");
    }
}

impl PointLight {
    fn position(&self, vm: &VM) {
        vm.get_slot_foreign(0);
    }

    fn color(&self, vm: &VM) {
        vm.set_slot_double(0, self.color);
    }
}

impl PointLight {
    fn new(vm: &VM) {
        let position = get_slot_checked!(vm => num 1);
        let color = get_slot_checked!(vm => num 2);
        let intensity = get_slot_checked!(vm => num 3);
        send_foreign!(vm, "graphics", "PointLight", PointLight { position, color, intensity } => 0);
    }
}

use wgpu::util::DeviceExt;

// contents must match 4 bytes layout. should do that thru gpu_data.
pub fn uniform_buffer(contents: &[u8], device: &wgpu::Device, label: Option<&str>) -> wgpu::Buffer {
    let desc = &wgpu::util::BufferInitDescriptor {
        label,
        contents,
        usage: wgpu::BufferUsage::UNIFORM
            | wgpu::BufferUsage::COPY_DST
            | wgpu::BufferUsage::COPY_SRC,
    };
    device.create_buffer_init(desc)
}

// this tells wpgu that we want to read this buffer from the cpu,
// this buffer is by default empty.
pub fn render_buffer(rt: &super::RenderTexture, device: &wgpu::Device) -> wgpu::Buffer {
    let u32_size = std::mem::size_of::<u32>() as u32;
    let size = (u32_size * rt.size.width * rt.size.height) as wgpu::BufferAddress;
    let desc = wgpu::BufferDescriptor {
        size,
        usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::MAP_READ,
        label: Some("Render Texture Buffer"),
        mapped_at_creation: false,
    };
    device.create_buffer(&desc)
}

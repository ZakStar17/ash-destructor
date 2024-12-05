use ash::vk;

use crate::{Alloc, DeviceDestroyable};

impl DeviceDestroyable for vk::PrivateDataSlot {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_private_data_slot(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::SamplerYcbcrConversion {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_sampler_ycbcr_conversion(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::DescriptorUpdateTemplate {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_descriptor_update_template(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::Sampler {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_sampler(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::Fence {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_fence(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::Event {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_event(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::Image {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_image(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::CommandPool {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_command_pool(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::ImageView {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_image_view(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::RenderPass {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_render_pass(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::Framebuffer {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_framebuffer(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::PipelineLayout {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_pipeline_layout(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::PipelineCache {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_pipeline_cache(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::Buffer {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_buffer(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::ShaderModule {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_shader_module(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::Pipeline {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_pipeline(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::Semaphore {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_semaphore(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::DescriptorPool {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_descriptor_pool(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::QueryPool {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_query_pool(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::DescriptorSetLayout {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_descriptor_set_layout(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::BufferView {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.destroy_buffer_view(*self, allocation_callbacks);
    }
}

impl DeviceDestroyable for vk::DeviceMemory {
    unsafe fn destroy_self_alloc(&self, device: &ash::Device, allocation_callbacks: Alloc) {
        device.free_memory(*self, allocation_callbacks);
    }
}

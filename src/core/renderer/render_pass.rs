use crate::core::{
    error::{RtError, RtResult},
    renderer,
};
use ash::vk;

pub struct RenderPass {
    render_pass: vk::RenderPass,
    framebuffers: Vec<vk::Framebuffer>,
}

impl RenderPass {
    pub fn new(device: &ash::Device, swapchain: &renderer::Swapchain) -> RtResult<Self> {
        let render_pass = Self::create_render_pass(device, swapchain)?;
        let framebuffers = Self::create_framebuffers(device, swapchain, &render_pass)?;

        Ok(Self {
            render_pass,
            framebuffers,
        })
    }

    #[inline]
    pub fn render_pass(&self) -> &vk::RenderPass {
        &self.render_pass
    }

    #[inline]
    pub fn framebuffers(&self) -> &[vk::Framebuffer] {
        &self.framebuffers
    }

    pub fn resize(
        &mut self,
        device: &ash::Device,
        swapchain: &renderer::Swapchain,
    ) -> RtResult<()> {
        self.framebuffers
            .iter_mut()
            .for_each(|framebuffer| unsafe { device.destroy_framebuffer(*framebuffer, None) });
        self.framebuffers = Self::create_framebuffers(device, swapchain, &self.render_pass)?;

        Ok(())
    }

    #[inline]
    pub unsafe fn destroy(&mut self, device: &ash::Device) {
        self.framebuffers
            .iter_mut()
            .for_each(|framebuffer| unsafe { device.destroy_framebuffer(*framebuffer, None) });
        unsafe {
            device.destroy_render_pass(self.render_pass, None);
        }
    }

    fn create_render_pass(
        device: &ash::Device,
        swapchain: &renderer::Swapchain,
    ) -> RtResult<vk::RenderPass> {
        let color_attachment = vk::AttachmentDescription::default()
            .format(*swapchain.image_format())
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);
        let color_attachment_ref = vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(std::slice::from_ref(&color_attachment_ref));
        let dependency = vk::SubpassDependency::default()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE);
        let create_info = vk::RenderPassCreateInfo::default()
            .attachments(std::slice::from_ref(&color_attachment))
            .subpasses(std::slice::from_ref(&subpass))
            .dependencies(std::slice::from_ref(&dependency));

        match unsafe { device.create_render_pass(&create_info, None) } {
            Ok(render_pass) => Ok(render_pass),
            Err(err) => Err(RtError::CreateRenderPass(err.into())),
        }
    }

    fn create_framebuffers(
        device: &ash::Device,
        swapchain: &renderer::Swapchain,
        render_pass: &vk::RenderPass,
    ) -> RtResult<Vec<vk::Framebuffer>> {
        let mut swapchain_framebuffers: Vec<vk::Framebuffer> =
            Vec::with_capacity(swapchain.image_views().len());
        let create_info = vk::FramebufferCreateInfo::default()
            .render_pass(*render_pass)
            .width(swapchain.extent().width)
            .height(swapchain.extent().height)
            .layers(1);

        for image_view in swapchain.image_views() {
            let create_info = create_info.attachments(std::slice::from_ref(image_view));
            let framebuffer = match unsafe { device.create_framebuffer(&create_info, None) } {
                Ok(framebuffer) => Ok(framebuffer),
                Err(err) => Err(RtError::CreateFramebuffer(err.into())),
            }?;

            swapchain_framebuffers.push(framebuffer);
        }

        Ok(swapchain_framebuffers)
    }
}

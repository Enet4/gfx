use hal::{self, image as i, pass};
use hal::memory::Properties;
use gl;
use Backend;
use std::cell::Cell;


pub type RawBuffer   = gl::types::GLuint;
pub type Shader      = gl::types::GLuint;
pub type Program     = gl::types::GLuint;
pub type FrameBuffer = gl::types::GLuint;
pub type Surface     = gl::types::GLuint;
pub type Texture     = gl::types::GLuint;
pub type Sampler     = gl::types::GLuint;

#[derive(Debug)]
pub struct Buffer {
    pub(crate) raw: RawBuffer,
    pub(crate) target: gl::types::GLenum,
    pub(crate) cpu_can_read: bool,
    pub(crate) cpu_can_write: bool,
}

#[derive(Debug)]
pub struct BufferView;

#[derive(Debug)]
pub struct Fence(pub Cell<gl::types::GLsync>);
unsafe impl Send for Fence {}
unsafe impl Sync for Fence {}

impl Fence {
    pub(crate) fn new(sync: gl::types::GLsync) -> Self {
        Fence(Cell::new(sync))
    }
}

#[derive(Clone, Debug, Copy)]
pub struct GraphicsPipeline {
    pub(crate) program: Program,
}

#[derive(Clone, Debug, Copy)]
pub struct ComputePipeline {
    pub(crate) program: Program,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Image {
    Surface(Surface),
    Texture(Texture),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
/// Additionally storing the `SamplerInfo` for older OpenGL versions, which
/// don't support separate sampler objects.
pub enum FatSampler {
    Sampler(Sampler),
    Info(i::SamplerInfo),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ImageView {
    Surface(Surface),
    Texture(Texture, i::Level),
    TextureLayer(Texture, i::Level, i::Layer),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DescriptorSetLayout;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DescriptorSet;

#[derive(Debug)]
pub struct DescriptorPool {}

impl hal::DescriptorPool<Backend> for DescriptorPool {
    fn allocate_sets(&mut self, layouts: &[&DescriptorSetLayout]) -> Vec<DescriptorSet> {
        layouts.iter().map(|_| DescriptorSet).collect()
    }

    fn reset(&mut self) {
        unimplemented!()
    }
}

#[derive(Clone, Copy, Debug, Hash)]
pub struct ShaderModule {
    pub(crate) raw: Shader,
}

#[derive(Debug)]
pub struct Memory {
    pub(crate) properties: Properties,
}

impl Memory {
    pub fn can_upload(&self) -> bool {
        self.properties.contains(Properties::CPU_VISIBLE)
    }
    pub fn can_download(&self) -> bool {
        self.properties.contains(Properties::CPU_VISIBLE | Properties::CPU_CACHED)
    }
}

#[derive(Debug)]
pub struct RenderPass {
    pub(crate) attachments: Vec<pass::Attachment>,
    pub(crate) subpasses: Vec<SubpassDesc>,
}

#[derive(Debug)]
pub struct SubpassDesc {
    pub(crate) color_attachments: Vec<usize>,
}

#[derive(Debug)]
pub struct PipelineLayout;

#[derive(Debug)]
// No inter-queue synchronization required for GL.
pub struct Semaphore;

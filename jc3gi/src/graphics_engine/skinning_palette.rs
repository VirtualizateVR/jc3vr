#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// The per-instance skinning palette data (`NGraphicsEngine::CSkinningPaletteData`). Only its static
/// deletion queue is mapped.
pub struct SkinningPaletteData {}
impl SkinningPaletteData {
    pub const UpdateDeleteQueue_ADDRESS: usize = 0x1401566D0;
    /// Drains the double-buffered, lock-protected queue of skinning-palette vertex buffers pending
    /// deletion (`CSkinningPaletteData::UpdateDeleteQueue`, static). Called once per dispatch from
    /// [`GraphicsEngine::HandleDrawThreadTask`](crate::graphics_engine::graphics_engine::GraphicsEngine::HandleDrawThreadTask).
    /// IDA mislabels this address as a GPU-profiler `EndScope`.
    pub unsafe fn UpdateDeleteQueue() {
        unsafe {
            let f: unsafe extern "system" fn() = ::std::mem::transmute(
                Self::UpdateDeleteQueue_ADDRESS,
            );
            f()
        }
    }
}
impl std::convert::AsRef<SkinningPaletteData> for SkinningPaletteData {
    fn as_ref(&self) -> &SkinningPaletteData {
        self
    }
}
impl std::convert::AsMut<SkinningPaletteData> for SkinningPaletteData {
    fn as_mut(&mut self) -> &mut SkinningPaletteData {
        self
    }
}

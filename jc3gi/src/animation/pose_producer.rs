#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// A pose producer (`CPoseProducer`): owns an animated model's blenders and the `T0`/`T1` poses
/// the skinning palette is built from.
pub struct PoseProducer {}
impl PoseProducer {
    pub const MakeSkinningPalette_ADDRESS: usize = 0x141E69600;
    /// Queues a skinning-palette job for this producer: advances the time of any time-updatable
    /// blender, then records the skeleton's bone count, `dtf`, and the two poses into the next slot
    /// of the global skinning job table (up to 0x1000 jobs per frame) together with the output
    /// `palette` buffer. The skinning job later blends the `T0` and `T1` poses by `dtf` into the
    /// palette. Reached from
    /// [`AnimationController::UpdateSkinning`](crate::character::character::AnimationController::UpdateSkinning)
    /// in the render phase.
    pub unsafe fn MakeSkinningPalette(
        &mut self,
        dtf: f32,
        palette: *mut ::std::ffi::c_void,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                dtf: f32,
                palette: *mut ::std::ffi::c_void,
            ) = ::std::mem::transmute(Self::MakeSkinningPalette_ADDRESS);
            f(self as *mut Self as _, dtf, palette)
        }
    }
}
impl std::convert::AsRef<PoseProducer> for PoseProducer {
    fn as_ref(&self) -> &PoseProducer {
        self
    }
}
impl std::convert::AsMut<PoseProducer> for PoseProducer {
    fn as_mut(&mut self) -> &mut PoseProducer {
        self
    }
}

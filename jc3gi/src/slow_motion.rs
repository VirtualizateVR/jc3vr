#![cfg_attr(any(), rustfmt::skip)]
/// `CSlowMotionManager::s_NormalGameSpeed`: the game speed the slow-motion manager returns the
/// clock to when no slow-motion effect is active. Initialized to `1.0`.
///
/// `CSlowMotionManager::Update` (called once per frame from the special-effects update) writes
/// [`Clock::m_UpdateSpeed`](crate::clock::Clock::m_UpdateSpeed) every frame as a blend between this value
/// and the active effect's game speed, weighted by the effect's eased status, so this is the
/// authoritative "normal" clock rate rather than the clock field itself: a direct write to the
/// clock's speed is overwritten on the next update, while a write here persists and still lets
/// scripted slow-motion effects (the wingsuit near-miss, the death camera, sequences) blend
/// against it. `CSlowMotionManager::Reset` copies it straight into the clock.
///
/// The game update divides frame deltas by the resulting clock speed where a real-time quantity
/// is needed (health effects, camera collision, the run state's sub-step budget), and every other
/// consumer of the clock's scaled delta simply runs slower.
pub unsafe fn get_SlowMotionManager_NormalGameSpeed() -> &'static mut f32 {
    unsafe { &mut *(0x142D57830 as *mut f32) }
}

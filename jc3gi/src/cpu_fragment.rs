#![cfg_attr(any(), rustfmt::skip)]
pub const CpuFragmentWaitUntilSignalIsNonZero_ADDRESS: usize = 0x141DFA730;
/// Spin-wait (pumping other ready fragments) until `*signal` is non-zero -- one of Avalanche's CPU
/// job-fragment primitives, the engine's drain for an outstanding async fragment. For example the
/// draw-dispatch fragment signals completion at
/// [`m_DrawThreadWorkSignal`](crate::graphics_engine::graphics_engine::GraphicsEngine::m_DrawThreadWorkSignal). It
/// returns immediately when the signal is already non-zero, and otherwise spins until the matching
/// fragment raises it. Below two primary threads ([`CpuPrimaryCount`](crate::cpu_fragment::CpuPrimaryCount)) the draw work runs inline and
/// never raises its signal, so a wait on that signal never returns.
///
/// The wait is work-stealing: while the signal is zero, the underlying `CpuFragmentWait` loop pops
/// queued fragments off the global sync queue and runs them (`CpuFragmentRun`) on the calling
/// thread. A fragment run on a thread the fragment system does not know (neither a registered worker
/// nor the primary thread) faults on the fragment system's per-thread state.
pub unsafe fn CpuFragmentWaitUntilSignalIsNonZero(signal: *const u32) {
    unsafe {
        let f: unsafe extern "system" fn(signal: *const u32) = ::std::mem::transmute(
            CpuFragmentWaitUntilSignalIsNonZero_ADDRESS,
        );
        f(signal)
    }
}
pub const CpuPrimaryCount_ADDRESS: usize = 0x1410CF770;
/// The number of primary job-worker threads. The engine only dispatches the async draw fragment (and
/// only waits on its signal) when this is greater than `1`; at or below it the draw runs inline and the
/// completion signal is never raised.
pub unsafe fn CpuPrimaryCount() -> i32 {
    unsafe {
        let f: unsafe extern "system" fn() -> i32 = ::std::mem::transmute(
            CpuPrimaryCount_ADDRESS,
        );
        f()
    }
}

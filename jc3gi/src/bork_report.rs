#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// Avalanche's crash and telemetry report ("bork report"). It samples frame metrics and writes them
/// as JSON metric entries and a plain-text `total.txt` summary.
pub struct BorkReport {}
impl BorkReport {
    pub const WriteMetrics_ADDRESS: usize = 0x140878330;
    /// Writes `entry`'s metrics as a JSON array: the per-category GPU times (named by
    /// `GetRenderPassCategoryName`), the frame time and its peak, the per-category GPU memory, the
    /// CPU frame and draw times, the per-phase CPU times named through
    /// [`CpuProfiler::GetScopeName`](crate::cpu_profiler::CpuProfiler::GetScopeName), and the CPU memory
    /// usage.
    pub unsafe fn WriteMetrics(&mut self, entry: *mut crate::bork_report::ReportEntry) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                entry: *mut crate::bork_report::ReportEntry,
            ) = ::std::mem::transmute(Self::WriteMetrics_ADDRESS);
            f(self as *mut Self as _, entry)
        }
    }
    pub const WriteTotalEntry_ADDRESS: usize = 0x1408A30F0;
    /// Writes the sampling session's averages to `total.txt` in the scratch path: build and system
    /// information, then the average GPU time, frame time, memory usage, per-category GPU times and
    /// memory, and per-phase CPU times. Called when sampling stops.
    pub unsafe fn WriteTotalEntry(&mut self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) = ::std::mem::transmute(
                Self::WriteTotalEntry_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
}
impl std::convert::AsRef<BorkReport> for BorkReport {
    fn as_ref(&self) -> &BorkReport {
        self
    }
}
impl std::convert::AsMut<BorkReport> for BorkReport {
    fn as_mut(&mut self) -> &mut BorkReport {
        self
    }
}
#[repr(C, align(8))]
/// One sampled report entry (`CBorkReport::ReportEntry`): the accumulated GPU per-category, GPU
/// memory, CPU per-phase, and memory metrics for a sampling window. Layout not yet mapped.
pub struct ReportEntry {}
impl ReportEntry {}
impl std::convert::AsRef<ReportEntry> for ReportEntry {
    fn as_ref(&self) -> &ReportEntry {
        self
    }
}
impl std::convert::AsMut<ReportEntry> for ReportEntry {
    fn as_mut(&mut self) -> &mut ReportEntry {
        self
    }
}

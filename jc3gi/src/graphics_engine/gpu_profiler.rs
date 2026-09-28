#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// A ring of GPU queries buffered over several frames (`NGraphicsEngine::CGPUBufferedQuery`): each
/// query kind has `count * m_NumBuffers` handles, so a caller can issue queries this frame and read
/// them back buffers later without stalling on the GPU. The issue functions return the handle index
/// they used, or [`INVALID_QUERY_INDEX`](crate::graphics_engine::gpu_profiler::GPUBufferedQuery::INVALID_QUERY_INDEX) when the kind's per-buffer
/// capacity is exhausted.
pub struct GPUBufferedQuery {
    pub m_Timestamps: *mut *mut crate::graphics_engine::graphics_engine::HTimeStampQuery_t,
    pub m_Occlusions: *mut *mut crate::graphics_engine::gpu_profiler::HOcclusionQuery_t,
    pub m_Fences: *mut *mut crate::graphics_engine::gpu_profiler::HEventQuery_t,
    pub m_Frequency: *mut *mut crate::graphics_engine::graphics_engine::HTimeStampDisjointQuery_t,
    /// The per-buffer capacity of each kind, and the buffer count.
    pub m_Params: crate::graphics_engine::gpu_profiler::GPUBufferedQueryCreateParams,
    /// This frame's usage of each kind; `m_NumBuffers` holds the current buffer index.
    pub m_Current: crate::graphics_engine::gpu_profiler::GPUBufferedQueryCreateParams,
}
fn _GPUBufferedQuery_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x30], GPUBufferedQuery>([0u8; 0x30]);
    }
    unreachable!()
}
impl GPUBufferedQuery {
    pub const Create_ADDRESS: usize = 0x1400A0740;
    /// Allocates the handle arrays and creates `count * m_NumBuffers` queries of each kind.
    pub unsafe fn Create(
        &mut self,
        device: *mut crate::graphics_engine::graphics_engine::HDevice_t,
        params: crate::graphics_engine::gpu_profiler::GPUBufferedQueryCreateParams,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                device: *mut crate::graphics_engine::graphics_engine::HDevice_t,
                params: crate::graphics_engine::gpu_profiler::GPUBufferedQueryCreateParams,
            ) = ::std::mem::transmute(Self::Create_ADDRESS);
            f(self as *mut Self as _, device, params)
        }
    }
    pub const Destroy_ADDRESS: usize = 0x1400A08C0;
    /// Destroys every query and frees the handle arrays.
    pub unsafe fn Destroy(
        &mut self,
        device: *mut crate::graphics_engine::graphics_engine::HDevice_t,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                device: *mut crate::graphics_engine::graphics_engine::HDevice_t,
            ) = ::std::mem::transmute(Self::Destroy_ADDRESS);
            f(self as *mut Self as _, device)
        }
    }
    pub const SetTimestamp_ADDRESS: usize = 0x1400A04E0;
    /// Issues the next timestamp query of the current buffer; returns its index or
    /// [`INVALID_QUERY_INDEX`](crate::graphics_engine::gpu_profiler::GPUBufferedQuery::INVALID_QUERY_INDEX).
    pub unsafe fn SetTimestamp(
        &mut self,
        ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
    ) -> i32 {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
            ) -> i32 = ::std::mem::transmute(Self::SetTimestamp_ADDRESS);
            f(self as *mut Self as _, ctx)
        }
    }
    pub const SetFence_ADDRESS: usize = 0x1400A0480;
    /// Issues the next event (fence) query of the current buffer; returns its index or
    /// [`INVALID_QUERY_INDEX`](crate::graphics_engine::gpu_profiler::GPUBufferedQuery::INVALID_QUERY_INDEX).
    pub unsafe fn SetFence(
        &mut self,
        ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
    ) -> i32 {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
            ) -> i32 = ::std::mem::transmute(Self::SetFence_ADDRESS);
            f(self as *mut Self as _, ctx)
        }
    }
    pub const SetOcclusionBegin_ADDRESS: usize = 0x1400A0540;
    /// Begins the next occlusion query of the current buffer; returns its index or
    /// [`INVALID_QUERY_INDEX`](crate::graphics_engine::gpu_profiler::GPUBufferedQuery::INVALID_QUERY_INDEX).
    pub unsafe fn SetOcclusionBegin(
        &mut self,
        ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
    ) -> i32 {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
            ) -> i32 = ::std::mem::transmute(Self::SetOcclusionBegin_ADDRESS);
            f(self as *mut Self as _, ctx)
        }
    }
    pub const SetOcclusionEnd_ADDRESS: usize = 0x1400A05A0;
    /// Ends the occlusion query at `index` (ignored when it is [`INVALID_QUERY_INDEX`](crate::graphics_engine::gpu_profiler::GPUBufferedQuery::INVALID_QUERY_INDEX)).
    pub unsafe fn SetOcclusionEnd(
        &mut self,
        ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
        index: i32,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
                index: i32,
            ) = ::std::mem::transmute(Self::SetOcclusionEnd_ADDRESS);
            f(self as *mut Self as _, ctx, index)
        }
    }
    pub const SetFrequencyBegin_ADDRESS: usize = 0x1400A05D0;
    /// Begins the next timestamp-disjoint (frequency) query of the current buffer; returns its index
    /// or [`INVALID_QUERY_INDEX`](crate::graphics_engine::gpu_profiler::GPUBufferedQuery::INVALID_QUERY_INDEX).
    pub unsafe fn SetFrequencyBegin(
        &mut self,
        ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
    ) -> i32 {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
            ) -> i32 = ::std::mem::transmute(Self::SetFrequencyBegin_ADDRESS);
            f(self as *mut Self as _, ctx)
        }
    }
    pub const GetOcclusionResult_ADDRESS: usize = 0x1400A06C0;
    /// Reads the occlusion query at `index` (0 when it is [`INVALID_QUERY_INDEX`](crate::graphics_engine::gpu_profiler::GPUBufferedQuery::INVALID_QUERY_INDEX)).
    pub unsafe fn GetOcclusionResult(
        &mut self,
        ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
        index: i32,
    ) -> u32 {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
                index: i32,
            ) -> u32 = ::std::mem::transmute(Self::GetOcclusionResult_ADDRESS);
            f(self as *mut Self as _, ctx, index)
        }
    }
    pub const GetFrequencyResult_ADDRESS: usize = 0x1400A0700;
    /// Reads the timestamp frequency of the disjoint query at `index` (0 when it is [`INVALID_QUERY_INDEX`](crate::graphics_engine::gpu_profiler::GPUBufferedQuery::INVALID_QUERY_INDEX)).
    pub unsafe fn GetFrequencyResult(
        &mut self,
        ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
        index: i32,
    ) -> u64 {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
                index: i32,
            ) -> u64 = ::std::mem::transmute(Self::GetFrequencyResult_ADDRESS);
            f(self as *mut Self as _, ctx, index)
        }
    }
}
impl GPUBufferedQuery {
    /// The index the issue functions return when the kind's per-buffer capacity is exhausted. The
    /// end and read functions accept it and do nothing (the reads return 0).
    pub const INVALID_QUERY_INDEX: i32 = -1;
}
impl std::convert::AsRef<GPUBufferedQuery> for GPUBufferedQuery {
    fn as_ref(&self) -> &GPUBufferedQuery {
        self
    }
}
impl std::convert::AsMut<GPUBufferedQuery> for GPUBufferedQuery {
    fn as_mut(&mut self) -> &mut GPUBufferedQuery {
        self
    }
}
#[derive(Copy, Clone)]
#[repr(C, align(2))]
/// The per-kind query counts of a [`GPUBufferedQuery`](crate::graphics_engine::gpu_profiler::GPUBufferedQuery) (`CGPUBufferedQuery::SCreateParams`). The
/// same shape holds both the capacities and the per-frame usage counters.
pub struct GPUBufferedQueryCreateParams {
    pub m_NumTimestamps: u16,
    pub m_NumOcclusions: u16,
    pub m_NumFences: u16,
    pub m_NumFrequency: u8,
    pub m_NumBuffers: u8,
}
fn _GPUBufferedQueryCreateParams_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x8], GPUBufferedQueryCreateParams>([0u8; 0x8]);
    }
    unreachable!()
}
impl GPUBufferedQueryCreateParams {}
impl std::convert::AsRef<GPUBufferedQueryCreateParams> for GPUBufferedQueryCreateParams {
    fn as_ref(&self) -> &GPUBufferedQueryCreateParams {
        self
    }
}
impl std::convert::AsMut<GPUBufferedQueryCreateParams> for GPUBufferedQueryCreateParams {
    fn as_mut(&mut self) -> &mut GPUBufferedQueryCreateParams {
        self
    }
}
#[repr(C, align(1))]
/// The engine's developer GPU profiler (`NGraphicsEngine::CGPUProfiler`, `g_GPUProfiler`): per-scope
/// GPU and CPU times, occlusion, and draw-call counts over the render-pass categories. Only the
/// construction and the result-count accessor survive; the scope recording and result
/// read-back are compiled out. Created by `CGraphicsEngine::InitializeSystem`.
pub struct GPUProfiler {
    _field_0: [u8; 2344],
}
fn _GPUProfiler_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x928], GPUProfiler>([0u8; 0x928]);
    }
    unreachable!()
}
impl GPUProfiler {
    pub unsafe fn get() -> Option<&'static mut Self> {
        unsafe {
            let ptr: *mut Self = *(5417317992usize as *mut *mut Self);
            ptr.as_mut()
        }
    }
}
impl GPUProfiler {
    pub const Constructor_ADDRESS: usize = 0x1400F3E60;
    pub unsafe fn Constructor(
        &mut self,
    ) -> *mut crate::graphics_engine::gpu_profiler::GPUProfiler {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
            ) -> *mut crate::graphics_engine::gpu_profiler::GPUProfiler = ::std::mem::transmute(
                Self::Constructor_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
    pub const GetScopeResultCount_ADDRESS: usize = 0x1400B7EF0;
    /// The number of scope results in the last frame's results (80-byte `SScopeData` entries).
    pub unsafe fn GetScopeResultCount(&self) -> i32 {
        unsafe {
            let f: unsafe extern "system" fn(this: *const Self) -> i32 = ::std::mem::transmute(
                Self::GetScopeResultCount_ADDRESS,
            );
            f(self as *const Self as _)
        }
    }
    pub const GetScopeResult_ADDRESS: usize = 0x1400A0F90;
    /// The scope result at `index`. Compiled to return null.
    pub unsafe fn GetScopeResult(&self, index: i32) -> *const ::std::ffi::c_void {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                index: i32,
            ) -> *const ::std::ffi::c_void = ::std::mem::transmute(
                Self::GetScopeResult_ADDRESS,
            );
            f(self as *const Self as _, index)
        }
    }
}
impl std::convert::AsRef<GPUProfiler> for GPUProfiler {
    fn as_ref(&self) -> &GPUProfiler {
        self
    }
}
impl std::convert::AsMut<GPUProfiler> for GPUProfiler {
    fn as_mut(&mut self) -> &mut GPUProfiler {
        self
    }
}
#[repr(C, align(8))]
/// A graphics-backend event (fence) query handle (`Graphics::HEventQuery_t`).
pub struct HEventQuery_t {}
impl HEventQuery_t {}
impl std::convert::AsRef<HEventQuery_t> for HEventQuery_t {
    fn as_ref(&self) -> &HEventQuery_t {
        self
    }
}
impl std::convert::AsMut<HEventQuery_t> for HEventQuery_t {
    fn as_mut(&mut self) -> &mut HEventQuery_t {
        self
    }
}
#[repr(C, align(8))]
/// A graphics-backend occlusion query handle (`Graphics::HOcclusionQuery_t`).
pub struct HOcclusionQuery_t {}
impl HOcclusionQuery_t {}
impl std::convert::AsRef<HOcclusionQuery_t> for HOcclusionQuery_t {
    fn as_ref(&self) -> &HOcclusionQuery_t {
        self
    }
}
impl std::convert::AsMut<HOcclusionQuery_t> for HOcclusionQuery_t {
    fn as_mut(&mut self) -> &mut HOcclusionQuery_t {
        self
    }
}
#[repr(C, align(8))]
/// A GPU timer for one render pass (`NGraphicsEngine::CRenderPassGpuTimingQuery`): a four-slot ring
/// of start/end timestamp and disjoint queries, read back three frames later into a timing
/// history. The render engine owns one instance, timing the particle pass.
pub struct RenderPassGpuTimingQuery {
    /// The four ring slots.
    pub m_TimeStamps: *mut crate::graphics_engine::gpu_profiler::RenderPassGpuTimingQueryTimeStampData,
    pub m_CurrentTimeStampIndex: u32,
    pub m_CurrentHistoryIndex: u32,
    /// The GPU time of recent frames in milliseconds.
    pub m_ParticleGPUTimingHistory: [f32; 10],
}
fn _RenderPassGpuTimingQuery_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x38], RenderPassGpuTimingQuery>([0u8; 0x38]);
    }
    unreachable!()
}
impl RenderPassGpuTimingQuery {
    pub const Create_ADDRESS: usize = 0x1400A0A20;
    /// Allocates the four ring slots and creates their queries.
    pub unsafe fn Create(
        &mut self,
        device: *mut crate::graphics_engine::graphics_engine::HDevice_t,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                device: *mut crate::graphics_engine::graphics_engine::HDevice_t,
            ) = ::std::mem::transmute(Self::Create_ADDRESS);
            f(self as *mut Self as _, device)
        }
    }
    pub const Destroy_ADDRESS: usize = 0x1400A0AC0;
    /// Destroys the ring's queries and frees it.
    pub unsafe fn Destroy(
        &mut self,
        device: *mut crate::graphics_engine::graphics_engine::HDevice_t,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                device: *mut crate::graphics_engine::graphics_engine::HDevice_t,
            ) = ::std::mem::transmute(Self::Destroy_ADDRESS);
            f(self as *mut Self as _, device)
        }
    }
    pub const BeginGPUTimingQuery_ADDRESS: usize = 0x1400A0B40;
    /// Advances the ring, reads the oldest slot's timestamps and frequency into the history, then
    /// records the start timestamp and begins the disjoint query of the new slot.
    pub unsafe fn BeginGPUTimingQuery(
        &mut self,
        ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
            ) = ::std::mem::transmute(Self::BeginGPUTimingQuery_ADDRESS);
            f(self as *mut Self as _, ctx)
        }
    }
    pub const EndGPUTimingQuery_ADDRESS: usize = 0x1400A0C70;
    /// Records the current slot's end timestamp and ends its disjoint query.
    pub unsafe fn EndGPUTimingQuery(
        &mut self,
        ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
            ) = ::std::mem::transmute(Self::EndGPUTimingQuery_ADDRESS);
            f(self as *mut Self as _, ctx)
        }
    }
    pub const GetAverageGpuTiming_ADDRESS: usize = 0x1400A0CC0;
    /// A fixed weighted average of the
    /// [`m_ParticleGPUTimingHistory`](crate::graphics_engine::gpu_profiler::RenderPassGpuTimingQuery::m_ParticleGPUTimingHistory) samples, in milliseconds.
    pub unsafe fn GetAverageGpuTiming(&mut self) -> f32 {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) -> f32 = ::std::mem::transmute(
                Self::GetAverageGpuTiming_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
}
impl std::convert::AsRef<RenderPassGpuTimingQuery> for RenderPassGpuTimingQuery {
    fn as_ref(&self) -> &RenderPassGpuTimingQuery {
        self
    }
}
impl std::convert::AsMut<RenderPassGpuTimingQuery> for RenderPassGpuTimingQuery {
    fn as_mut(&mut self) -> &mut RenderPassGpuTimingQuery {
        self
    }
}
#[repr(C, align(8))]
/// One slot of a [`RenderPassGpuTimingQuery`](crate::graphics_engine::gpu_profiler::RenderPassGpuTimingQuery) ring (`CRenderPassGpuTimingQuery::STimeStampData`).
pub struct RenderPassGpuTimingQueryTimeStampData {
    pub m_TimeStampQueryStart: *mut crate::graphics_engine::graphics_engine::HTimeStampQuery_t,
    pub m_TimeStampQueryEnd: *mut crate::graphics_engine::graphics_engine::HTimeStampQuery_t,
    pub m_GPUFrequencyQuery: *mut crate::graphics_engine::graphics_engine::HTimeStampDisjointQuery_t,
    /// The last read timestamp frequency (initialised to 100 MHz).
    pub m_Frequency: u64,
}
fn _RenderPassGpuTimingQueryTimeStampData_size_check() {
    unsafe {
        ::std::mem::transmute::<
            [u8; 0x20],
            RenderPassGpuTimingQueryTimeStampData,
        >([0u8; 0x20]);
    }
    unreachable!()
}
impl RenderPassGpuTimingQueryTimeStampData {}
impl std::convert::AsRef<RenderPassGpuTimingQueryTimeStampData>
for RenderPassGpuTimingQueryTimeStampData {
    fn as_ref(&self) -> &RenderPassGpuTimingQueryTimeStampData {
        self
    }
}
impl std::convert::AsMut<RenderPassGpuTimingQueryTimeStampData>
for RenderPassGpuTimingQueryTimeStampData {
    fn as_mut(&mut self) -> &mut RenderPassGpuTimingQueryTimeStampData {
        self
    }
}
#[repr(C, align(8))]
/// A scoped graphics debug marker (`Graphics::CScopeMarker`): the constructor opens a named scope on
/// the context with [`BeginScopeMarker`](crate::graphics_engine::graphics_engine::BeginScopeMarker) and the
/// destructor closes it with [`EndScopeMarker`](crate::graphics_engine::graphics_engine::EndScopeMarker).
pub struct ScopeMarker {
    pub m_Context: *mut crate::graphics_engine::graphics_engine::HContext_t,
}
fn _ScopeMarker_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x8], ScopeMarker>([0u8; 0x8]);
    }
    unreachable!()
}
impl ScopeMarker {
    pub const Constructor_ADDRESS: usize = 0x140099990;
    pub unsafe fn Constructor(
        &mut self,
        ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
        name: *const u8,
    ) -> *mut crate::graphics_engine::gpu_profiler::ScopeMarker {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                ctx: *mut crate::graphics_engine::graphics_engine::HContext_t,
                name: *const u8,
            ) -> *mut crate::graphics_engine::gpu_profiler::ScopeMarker = ::std::mem::transmute(
                Self::Constructor_ADDRESS,
            );
            f(self as *mut Self as _, ctx, name)
        }
    }
    pub const Destructor_ADDRESS: usize = 0x1400999C0;
    pub unsafe fn Destructor(&mut self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) = ::std::mem::transmute(
                Self::Destructor_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
}
impl std::convert::AsRef<ScopeMarker> for ScopeMarker {
    fn as_ref(&self) -> &ScopeMarker {
        self
    }
}
impl std::convert::AsMut<ScopeMarker> for ScopeMarker {
    fn as_mut(&mut self) -> &mut ScopeMarker {
        self
    }
}

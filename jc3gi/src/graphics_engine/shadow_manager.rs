#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// One shadow frustum's bookkeeping in [`ShadowManager`](crate::graphics_engine::shadow_manager::ShadowManager) (`SShadowFrustum`): the render passes that
/// draw its atlas slice, followed by the per-parity pass settings and fit state.
pub struct CascadeData {
    /// The pair of render passes that draw this cascade's atlas slice (dynamic and static
    /// geometry). [`ShadowManager::CommitRenderPassSettings`](crate::graphics_engine::shadow_manager::ShadowManager::CommitRenderPassSettings) gates them per dispatch via
    /// [`RenderPassState::m_Enabled`](crate::graphics_engine::render_pass::RenderPassState::m_Enabled).
    pub m_Passes: [*mut crate::graphics_engine::render_pass::RenderPass; 2],
    _field_10: [u8; 40],
    /// The pass settings for each frame parity (`[m_FrameIndex & 1]`), the per-parity slice
    /// indices [`ShadowManager::CommitRenderPassSettings`](crate::graphics_engine::shadow_manager::ShadowManager::CommitRenderPassSettings) reads to re-point the passes' render
    /// setups.
    pub m_RenderPassSettings: [crate::graphics_engine::shadow_manager::ShadowRenderPassSettings; 2],
    _field_60: [u8; 1024],
}
fn _CascadeData_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x460], CascadeData>([0u8; 0x460]);
    }
    unreachable!()
}
impl CascadeData {}
impl std::convert::AsRef<CascadeData> for CascadeData {
    fn as_ref(&self) -> &CascadeData {
        self
    }
}
impl std::convert::AsMut<CascadeData> for CascadeData {
    fn as_mut(&mut self) -> &mut CascadeData {
        self
    }
}
#[repr(C, align(8))]
/// The cascaded sun-shadow system. Each sim frame, [`UpdateRender`](crate::graphics_engine::shadow_manager::ShadowManager::UpdateRender)
/// fits the scheduled cascades to the active camera and writes the parity-buffered fit data; each
/// dispatch, [`CommitRenderPassSettings`](crate::graphics_engine::shadow_manager::ShadowManager::CommitRenderPassSettings) enables the
/// scheduled atlas passes. Cascade re-renders are amortised across frames by an update pattern, so
/// an unscheduled cascade keeps its previous fit and contents.
pub struct ShadowManager {
    _field_0: [u8; 4],
    /// The settings-side enable flag. The per-frame [`UpdateRender`](crate::graphics_engine::shadow_manager::ShadowManager::UpdateRender)
    /// syncs the engine to it via [`SetEnabled`](crate::graphics_engine::shadow_manager::ShadowManager::SetEnabled) whenever it differs
    /// from the live state -- the graphics-menu path for toggling shadows.
    pub m_Enabled: bool,
    _field_5: [u8; 2],
    /// The number of active sun cascades for each frame parity. [`RenderPass::SetRenderContextCamera`](crate::graphics_engine::render_pass::RenderPass::SetRenderContextCamera)
    /// copies the parity's entry into
    /// [`RenderContext::m_ActiveCascadeCount`](crate::graphics_engine::graphics_engine::RenderContext::m_ActiveCascadeCount).
    pub m_ActiveCascades: [u8; 2],
    /// The number of active shadow passes (cascades plus spot shadows) for each frame parity.
    pub m_ActiveShadowPassCount: [u8; 2],
    _field_b: [u8; 85],
    /// The sun-shadow cascade atlas: a single depth `Texture2DArray` whose slices hold every cascade's
    /// (and the spot shadows') fitted shadow map. Created by [`SetEnabled`](crate::graphics_engine::shadow_manager::ShadowManager::SetEnabled)
    /// (`CreateShadowDepthTexture`, one render target per array slice). Which slice each pass draws
    /// into is chosen per frame parity by the frustum's
    /// [`m_RenderPassSettings`](crate::graphics_engine::shadow_manager::CascadeData::m_RenderPassSettings).
    pub m_AtlasTexture: *mut crate::graphics_engine::texture::Texture,
    _field_68: [u8; 96],
    /// One render setup per atlas slice. [`CommitRenderPassSettings`](crate::graphics_engine::shadow_manager::ShadowManager::CommitRenderPassSettings)
    /// points each enabled pass at the setup of the slice its
    /// [`ShadowRenderPassSettings`](crate::graphics_engine::shadow_manager::ShadowRenderPassSettings) names.
    pub m_DepthRenderSetup: [*mut crate::graphics_engine::graphics_engine::HRenderSetup_t; 12],
    _field_128: [u8; 8],
    /// The per-cascade slots (passes plus fit bookkeeping).
    pub m_Cascades: [crate::graphics_engine::shadow_manager::CascadeData; 8],
    _field_2430: [u8; 56],
    /// The per-cascade shadow-map update level, indexed by cascade. A cascade re-fits and re-renders
    /// only every `2^level` frames (level 0 = every frame); between refreshes its fit is copied
    /// forward from the previous update. `CalculateUpdatePattern` assigns the levels (the default
    /// pattern is `{0, 1, 2, 3}` -- the nearest cascade every frame, each further one half as often),
    /// and [`SetActiveShadowPassCount`](crate::graphics_engine::shadow_manager::ShadowManager::SetActiveShadowPassCount) reads them each frame,
    /// gated against a rolling counter, to decide which cascades refresh. This is the mechanism that
    /// amortises cascade re-renders across frames.
    pub m_CascadeUpdateLevels: [i32; 6],
    _field_2480: [u8; 3548],
    /// The global sun-shadow transparency (0 = opaque shadows, 1 = no shadows), registered as the
    /// console variable `"Dev|Graphics|Shadows|Transparency"` and defaulting to 0.
    /// [`GetShadowFade`](crate::graphics_engine::shadow_manager::ShadowManager::GetShadowFade) returns it while the shadow system is enabled.
    pub m_ShadowTransparency: f32,
    /// The forward-material cascade blocks for each frame parity, written by
    /// [`UpdateRender`](crate::graphics_engine::shadow_manager::ShadowManager::UpdateRender). [`RenderPass::SetRenderContextCamera`](crate::graphics_engine::render_pass::RenderPass::SetRenderContextCamera) copies the
    /// parity's block into
    /// [`RenderContext::m_ShadowCascades`](crate::graphics_engine::graphics_engine::RenderContext::m_ShadowCascades).
    pub m_ShadowCascades: [crate::graphics_engine::graphics_engine::ShadowCascades; 2],
    /// The shadow light camera the static-shadow passes draw with; the uniform-light update rebuilds
    /// it from the cascade's matrices each dispatch.
    pub m_ScratchCamera: crate::camera::camera::Camera,
    /// The per-slice projective shadow transforms for each frame parity (`[parity][slice]`),
    /// written by [`UpdateRender`](crate::graphics_engine::shadow_manager::ShadowManager::UpdateRender). [`RenderPass::SetRenderContextCamera`](crate::graphics_engine::render_pass::RenderPass::SetRenderContextCamera) copies
    /// the parity's matrices into
    /// [`RenderContext::m_ShadowMatrices`](crate::graphics_engine::graphics_engine::RenderContext::m_ShadowMatrices).
    pub m_ShadowTextureMatrix: [[crate::types::math::Matrix4; 8]; 2],
    _field_3e50: [u8; 9088],
    /// The terrain horizon maps, `[map][parity]`. [`RenderPass::SetRenderContextCamera`](crate::graphics_engine::render_pass::RenderPass::SetRenderContextCamera) copies the
    /// parity's pair into
    /// [`RenderContext::m_HorizonMap0`](crate::graphics_engine::graphics_engine::RenderContext::m_HorizonMap0)
    /// and [`m_HorizonMap1`](crate::graphics_engine::graphics_engine::RenderContext::m_HorizonMap1).
    pub m_HorizonMaps: [[*mut crate::graphics_engine::texture::Texture; 2]; 2],
    /// The horizon-map blend time for each frame parity, staged by `SetGlobalShaderConstants` into
    /// the vertex GlobalConstants (row 4 `.w`) as `m_HorizonTime[m_RenderFrameNo & 1]`.
    pub m_HorizonTime: [f32; 2],
    /// The horizon-map occlusion scale, staged by `SetGlobalShaderConstants` into the vertex
    /// GlobalConstants (row 5 `.w`) when [`m_EnableHorizonMapping`](crate::graphics_engine::shadow_manager::ShadowManager::m_EnableHorizonMapping) is
    /// set, and 0 otherwise.
    pub m_HorizonScale: f32,
    /// Whether horizon mapping is enabled.
    pub m_EnableHorizonMapping: bool,
    _field_61fd: [u8; 3],
    /// The texture of the manager's black texture holder (`m_BlackTexture`), bound in place of a null
    /// horizon map by
    /// [`SetupLightingTextures`](crate::graphics_engine::render_engine::RenderBlockTypeBase::SetupLightingTextures).
    pub m_HorizonMapFallback: *mut crate::graphics_engine::texture::Texture,
}
fn _ShadowManager_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x6208], ShadowManager>([0u8; 0x6208]);
    }
    unreachable!()
}
impl ShadowManager {
    pub const InitializeSystem_ADDRESS: usize = 0x1401A1E20;
    /// Initializes the shadow system at startup (called by `CGraphicsEngine::InitializeSystem` on
    /// [`GraphicsEngine::m_ShadowManager`](crate::graphics_engine::graphics_engine::GraphicsEngine::m_ShadowManager)):
    /// sets the 1024-texel map size, clears [`shadow_system_enabled`](get_shadow_system_enabled), enables the manager through
    /// [`SetEnabled`](crate::graphics_engine::shadow_manager::ShadowManager::SetEnabled), and starts the horizon-map streams.
    pub unsafe fn InitializeSystem(&mut self, ctx: *const ::std::ffi::c_void) -> bool {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                ctx: *const ::std::ffi::c_void,
            ) -> bool = ::std::mem::transmute(Self::InitializeSystem_ADDRESS);
            f(self as *mut Self as _, ctx)
        }
    }
    pub const GetShadowFade_ADDRESS: usize = 0x140177940;
    /// The global sun-shadow fade factor staged into the shader constants:
    /// [`m_ShadowTransparency`](crate::graphics_engine::shadow_manager::ShadowManager::m_ShadowTransparency) while the shadow system is enabled
    /// ([`shadow_system_enabled`](get_shadow_system_enabled)), otherwise 1.
    pub unsafe fn GetShadowFade(&self) -> f32 {
        unsafe {
            let f: unsafe extern "system" fn(this: *const Self) -> f32 = ::std::mem::transmute(
                Self::GetShadowFade_ADDRESS,
            );
            f(self as *const Self as _)
        }
    }
    pub const CommitRenderPassSettings_ADDRESS: usize = 0x1401779C0;
    /// The per-dispatch pass gate: clears every shadow pass's
    /// [`m_Enabled`](crate::graphics_engine::render_pass::RenderPassState::m_Enabled) flag, then
    /// re-enables the passes the update pattern scheduled this frame and re-points their render
    /// targets by the frame parity.
    pub unsafe fn CommitRenderPassSettings(
        &mut self,
        ctx: *mut crate::graphics_engine::graphics_engine::RenderContext,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                ctx: *mut crate::graphics_engine::graphics_engine::RenderContext,
            ) = ::std::mem::transmute(Self::CommitRenderPassSettings_ADDRESS);
            f(self as *mut Self as _, ctx)
        }
    }
    pub const SetEnabled_ADDRESS: usize = 0x14019EE50;
    /// Creates or destroys the shadow render targets and passes; the settings path behind
    /// [`m_Enabled`](crate::graphics_engine::shadow_manager::ShadowManager::m_Enabled).
    pub unsafe fn SetEnabled(&mut self, enabled: bool) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self, enabled: bool) = ::std::mem::transmute(
                Self::SetEnabled_ADDRESS,
            );
            f(self as *mut Self as _, enabled)
        }
    }
    pub const UpdateRender_ADDRESS: usize = 0x1401C7370;
    /// The sim-side per-frame update: syncs [`m_Enabled`](crate::graphics_engine::shadow_manager::ShadowManager::m_Enabled) via
    /// [`SetEnabled`](crate::graphics_engine::shadow_manager::ShadowManager::SetEnabled), fits the scheduled cascades to
    /// [`CameraManager::m_ActiveCamera`](crate::camera::camera_manager::CameraManager::m_ActiveCamera)
    /// (the fit frustum comes from that camera's `m_ProjectionF`), copies the unscheduled
    /// cascades' previous fits forward, writes the parity-indexed cascade transforms and per-slice
    /// matrices the draw side reads, and regenerates the cull planes.
    pub unsafe fn UpdateRender(&mut self, dt: f32, dtf: f32) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self, dt: f32, dtf: f32) = ::std::mem::transmute(
                Self::UpdateRender_ADDRESS,
            );
            f(self as *mut Self as _, dt, dtf)
        }
    }
    pub const SetActiveShadowPassCount_ADDRESS: usize = 0x14018A7D0;
    /// Sets the number of active shadow passes (sun cascades plus spot shadows) and rebuilds the
    /// per-frame cascade update schedule: it refreshes the amortisation pattern (recomputing
    /// [`m_CascadeUpdateLevels`](crate::graphics_engine::shadow_manager::ShadowManager::m_CascadeUpdateLevels) on a cascade/spot count change),
    /// then marks each cascade as either refreshing this frame -- when
    /// `((1 << m_CascadeUpdateLevels[c]) - 1) & rolling_counter == 0` -- or copying its previous fit
    /// forward. Called from `CGameStateRun::UpdateShadows` before
    /// [`UpdateRender`](crate::graphics_engine::shadow_manager::ShadowManager::UpdateRender).
    pub unsafe fn SetActiveShadowPassCount(&mut self, count: i32) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self, count: i32) = ::std::mem::transmute(
                Self::SetActiveShadowPassCount_ADDRESS,
            );
            f(self as *mut Self as _, count)
        }
    }
}
impl std::convert::AsRef<ShadowManager> for ShadowManager {
    fn as_ref(&self) -> &ShadowManager {
        self
    }
}
impl std::convert::AsMut<ShadowManager> for ShadowManager {
    fn as_mut(&mut self) -> &mut ShadowManager {
        self
    }
}
#[repr(C, align(4))]
/// The per-parity render-pass settings of one shadow frustum (`SShadowRenderPassSettings`): which
/// atlas slices its dynamic and static passes draw into.
pub struct ShadowRenderPassSettings {
    /// The atlas slice the static-geometry pass draws into, or negative when it does not draw.
    /// [`ShadowManager::CommitRenderPassSettings`](crate::graphics_engine::shadow_manager::ShadowManager::CommitRenderPassSettings) points the static pass at
    /// [`m_DepthRenderSetup`](crate::graphics_engine::shadow_manager::ShadowManager::m_DepthRenderSetup) of this slice.
    pub m_SliceIndexStatic: i32,
    /// The atlas slice the dynamic-geometry pass draws into.
    pub m_SliceIndexDynamic: i32,
    /// The static slice the dynamic slice is seeded from.
    pub m_CopyFromStaticSliceIndex: i32,
    pub m_SlotUse: i32,
    pub m_StaticGeneration: i32,
}
fn _ShadowRenderPassSettings_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x14], ShadowRenderPassSettings>([0u8; 0x14]);
    }
    unreachable!()
}
impl ShadowRenderPassSettings {}
impl std::convert::AsRef<ShadowRenderPassSettings> for ShadowRenderPassSettings {
    fn as_ref(&self) -> &ShadowRenderPassSettings {
        self
    }
}
impl std::convert::AsMut<ShadowRenderPassSettings> for ShadowRenderPassSettings {
    fn as_mut(&mut self) -> &mut ShadowRenderPassSettings {
        self
    }
}
/// The shadow system's global enable (`CShadowManager::m_SystemEnabled`). It gates
/// [`ShadowManager::GetShadowFade`](crate::graphics_engine::shadow_manager::ShadowManager::GetShadowFade) and the per-dispatch
/// [`ShadowManager::CommitRenderPassSettings`](crate::graphics_engine::shadow_manager::ShadowManager::CommitRenderPassSettings) call.
pub unsafe fn get_shadow_system_enabled() -> &'static mut bool {
    unsafe { &mut *(0x142ED75BB as *mut bool) }
}

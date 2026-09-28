#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// The landscape manager (`CLandscapeManager`). Opaque.
pub struct LandscapeManager {}
impl LandscapeManager {
    pub const UpdatePatchSystem_ADDRESS: usize = 0x14034B2F0;
    /// Assembles the landscape patch system's cull frusta (see [`PatchFrustumId`](crate::patch_system::PatchFrustumId) and
    /// [`PatchViewDesc::MAX_FRUSTUMS`](crate::patch_system::PatchViewDesc::MAX_FRUSTUMS)) from the occluder manager's main cull camera, the active
    /// shadow passes of the current frame parity, and the two reflective-shadow frusta, then runs
    /// the patch system update with them. Skipped unless the previous patch-system frame is
    /// complete or `force` is set.
    pub unsafe fn UpdatePatchSystem(&mut self, dt: f32, force: bool) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self, dt: f32, force: bool) = ::std::mem::transmute(
                Self::UpdatePatchSystem_ADDRESS,
            );
            f(self as *mut Self as _, dt, force)
        }
    }
}
impl std::convert::AsRef<LandscapeManager> for LandscapeManager {
    fn as_ref(&self) -> &LandscapeManager {
        self
    }
}
impl std::convert::AsMut<LandscapeManager> for LandscapeManager {
    fn as_mut(&mut self) -> &mut LandscapeManager {
        self
    }
}
#[repr(C, align(8))]
/// The argument block passed to a patch map's per-patch CPU update fragment. Partial: only the
/// fields the base-terrain fragment ([`TerrainPatchUpdate`](crate::patch_system::TerrainPatchUpdate)) reads are mapped.
pub struct PatchContext {
    _field_0: [u8; 8],
    /// The view description for this update; see [`PatchViewDesc`](crate::patch_system::PatchViewDesc).
    pub m_ViewDesc: *const crate::patch_system::PatchViewDesc,
    _field_10: [u8; 8],
    /// The patch's header; see [`PatchHeader`](crate::patch_system::PatchHeader).
    pub m_Header: *mut crate::patch_system::PatchHeader,
    _field_20: [u8; 8],
    /// The patch's per-patch constant memory. For the base terrain maps this is the patch's
    /// `CRenderBlockTerrain` instance, constructed in place when the map was created.
    pub m_RenderBlock: *mut crate::graphics_engine::render_block::RenderBlockTerrain,
    /// The patch map's type context. For the terrain maps it is the render-pass pointer table the
    /// fragment submits into.
    pub m_TypeContext: *mut ::std::ffi::c_void,
    _field_38: [u8; 24],
}
fn _PatchContext_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x50], PatchContext>([0u8; 0x50]);
    }
    unreachable!()
}
impl PatchContext {}
impl std::convert::AsRef<PatchContext> for PatchContext {
    fn as_ref(&self) -> &PatchContext {
        self
    }
}
impl std::convert::AsMut<PatchContext> for PatchContext {
    fn as_mut(&mut self) -> &mut PatchContext {
        self
    }
}
#[repr(u32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// The frustum ids `CLandscapeManager::UpdatePatchSystem` assigns to the entries of a landscape
/// patch-system update ([`PatchViewDesc::m_FrustumIDs`](crate::patch_system::PatchViewDesc::m_FrustumIDs)). The per-patch fragments route a patch to
/// render passes by the id of each set [`PatchHeader::m_VisibilityBits`](crate::patch_system::PatchHeader::m_VisibilityBits) bit.
pub enum PatchFrustumId {
    FRUSTUM_ID_MAIN_CAMERA = 0isize as _,
    FRUSTUM_ID_SHADOW_PASS_0 = 1isize as _,
    FRUSTUM_ID_SHADOW_PASS_1 = 2isize as _,
    FRUSTUM_ID_SHADOW_PASS_2 = 3isize as _,
    FRUSTUM_ID_SHADOW_PASS_3 = 4isize as _,
    FRUSTUM_ID_SHADOW_PASS_4 = 5isize as _,
    FRUSTUM_ID_SHADOW_PASS_5 = 6isize as _,
    FRUSTUM_ID_SHADOW_PASS_6 = 7isize as _,
    FRUSTUM_ID_SHADOW_PASS_7 = 8isize as _,
    FRUSTUM_ID_REFLECTIVE_SHADOW_0 = 9isize as _,
    FRUSTUM_ID_REFLECTIVE_SHADOW_1 = 10isize as _,
    FRUSTUM_ID_DEPTH_PREPASS = 12isize as _,
}
fn _PatchFrustumId_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], PatchFrustumId>([0u8; 0x4]);
    }
    unreachable!()
}
#[repr(C, align(8))]
/// A patch's bookkeeping header within a patch map (the Avalanche `patchsystem` dependency
/// library's grid-of-patches streaming, used by the landscape systems: the base VolumetricTerrain
/// patch maps `terrain_patch_lod9..12`, water, and vegetation). Partial: only the fields the
/// terrain patch-update fragment reads are mapped.
pub struct PatchHeader {
    _field_0: [u8; 12],
    /// The patch's dynamic memory size; reset to 0 when the patch finishes destruction.
    pub m_DynamicSize: u32,
    /// The patch's X coordinate in its map grid. The patch's world X derives as
    /// `(m_PatchX << m_Lod) + world_origin_x`, scaled by the world context's inverse XZ scale.
    pub m_PatchX: i16,
    /// The patch's Z coordinate in its map grid; see [`m_PatchX`](crate::patch_system::PatchHeader::m_PatchX).
    pub m_PatchZ: i16,
    /// Patch lifecycle status bits (`EPatchSystemStatusBits`, the `STATUS_*` constants).
    pub m_StatusBits: u16,
    _field_16: [u8; 3],
    /// A per-patch state counter the patch's update fragment advances during multi-step
    /// construction or destruction.
    pub m_UserState: u8,
    /// The patch map's LOD level (9..=12 for the base terrain maps; the patch spans
    /// `1 << m_Lod` world units per side before world scaling).
    pub m_Lod: u8,
    _field_1b: [u8; 1],
    /// Per-frustum visibility bits, written by the patch system's BFBC cull step each update: bit
    /// `i` is set when the patch's AABB passed the cull for frustum entry `i` of the update's view
    /// description ([`PatchViewDesc::m_FrustumIDs`](crate::patch_system::PatchViewDesc::m_FrustumIDs) maps entries to frustum ids). For the landscape
    /// patch system entry 0, and so bit 0, is always the main cull camera
    /// ([`PatchFrustumId::FRUSTUM_ID_MAIN_CAMERA`](crate::patch_system::PatchFrustumId::FRUSTUM_ID_MAIN_CAMERA)); the last populated bit is its second run
    /// ([`PatchFrustumId::FRUSTUM_ID_DEPTH_PREPASS`](crate::patch_system::PatchFrustumId::FRUSTUM_ID_DEPTH_PREPASS)). The patch-update
    /// fragments read these to decide which render passes receive the patch's render block.
    pub m_VisibilityBits: u16,
    _field_1e: [u8; 10],
    /// The patch's BFBC occludee handle, allocated by `BFBCAdd` when the patch map is created; the
    /// patch's world AABB is pushed through `BFBCSetAABB` under this handle during construction.
    pub m_BFBCOcludee: u16,
    _field_2a: [u8; 6],
}
fn _PatchHeader_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x30], PatchHeader>([0u8; 0x30]);
    }
    unreachable!()
}
impl PatchHeader {}
impl PatchHeader {
    /// [`m_StatusBits`](crate::patch_system::PatchHeader::m_StatusBits) bit (`PATCH_SYSTEM_STATUS_BIT_CONSTRUCTED`): the patch
    /// is constructed.
    pub const STATUS_CONSTRUCTED: u16 = 1;
    /// [`m_StatusBits`](crate::patch_system::PatchHeader::m_StatusBits) bit
    /// (`PATCH_SYSTEM_STATUS_BIT_CONSTRUCTION_COMPLETE`): construction has completed.
    pub const STATUS_CONSTRUCTION_COMPLETE: u16 = 16;
    /// [`m_StatusBits`](crate::patch_system::PatchHeader::m_StatusBits) bit
    /// (`PATCH_SYSTEM_STATUS_BIT_CONSTRUCTION_PENDING`): construction is pending.
    pub const STATUS_CONSTRUCTION_PENDING: u16 = 4;
    /// [`m_StatusBits`](crate::patch_system::PatchHeader::m_StatusBits) bit
    /// (`PATCH_SYSTEM_STATUS_BIT_DESTRUCTION_COMPLETE`): destruction has completed.
    pub const STATUS_DESTRUCTION_COMPLETE: u16 = 8;
    /// [`m_StatusBits`](crate::patch_system::PatchHeader::m_StatusBits) bit
    /// (`PATCH_SYSTEM_STATUS_BIT_DESTRUCTION_PENDING`): destruction is pending.
    pub const STATUS_DESTRUCTION_PENDING: u16 = 2;
    /// [`m_StatusBits`](crate::patch_system::PatchHeader::m_StatusBits) bit (`PATCH_SYSTEM_STATUS_BIT_INSIDE_WORLD`): the patch
    /// lies inside the world.
    pub const STATUS_INSIDE_WORLD: u16 = 32;
}
impl std::convert::AsRef<PatchHeader> for PatchHeader {
    fn as_ref(&self) -> &PatchHeader {
        self
    }
}
impl std::convert::AsMut<PatchHeader> for PatchHeader {
    fn as_mut(&mut self) -> &mut PatchHeader {
        self
    }
}
#[repr(C, align(8))]
/// The per-update view description handed to the patch-update fragments: the set of cull frusta
/// the patch system's BFBC step ran this update. For the landscape patch system
/// (`CLandscapeManager::UpdatePatchSystem`) the entries are: entry 0 = the occluder manager's main
/// cull camera ([`FRUSTUM_ID_MAIN_CAMERA`](crate::patch_system::PatchFrustumId::FRUSTUM_ID_MAIN_CAMERA)), entries 1..=N
/// = the active shadow cascades (the `FRUSTUM_ID_SHADOW_PASS_*` ids), then the two
/// reflective-shadow frusta (the `FRUSTUM_ID_REFLECTIVE_SHADOW_*` ids), and a final second run of
/// the same main cull camera
/// ([`FRUSTUM_ID_DEPTH_PREPASS`](crate::patch_system::PatchFrustumId::FRUSTUM_ID_DEPTH_PREPASS)); see
/// [`PatchFrustumId`](crate::patch_system::PatchFrustumId). Partial mapping.
pub struct PatchViewDesc {
    _field_0: [u8; 48],
    /// The frustum id for each entry, indexed by the same entry index as the patch's
    /// [`PatchHeader::m_VisibilityBits`](crate::patch_system::PatchHeader::m_VisibilityBits) bit.
    pub m_FrustumIDs: *const u32,
    /// The number of frustum entries this update (also the number of populated visibility bits).
    pub m_FrustumCount: u32,
    _field_3c: [u8; 4],
}
fn _PatchViewDesc_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x40], PatchViewDesc>([0u8; 0x40]);
    }
    unreachable!()
}
impl PatchViewDesc {}
impl PatchViewDesc {
    /// The capacity of the frustum-entry arrays `CLandscapeManager::UpdatePatchSystem` fills: the
    /// main camera, up to eight shadow passes, two reflective-shadow frusta, and the second main
    /// camera run.
    pub const MAX_FRUSTUMS: u32 = 12;
}
impl std::convert::AsRef<PatchViewDesc> for PatchViewDesc {
    fn as_ref(&self) -> &PatchViewDesc {
        self
    }
}
impl std::convert::AsMut<PatchViewDesc> for PatchViewDesc {
    fn as_mut(&mut self) -> &mut PatchViewDesc {
        self
    }
}
/// The [`UpdateTessellatedTriangles`](crate::patch_system::UpdateTessellatedTriangles) `metric_type` that derives tessellation from the camera's
/// distance beyond the patch bounds. The name is descriptive; the engine uses a bare integer.
pub const TESSELLATION_METRIC_DISTANCE: i32 = 2;
/// The [`UpdateTessellatedTriangles`](crate::patch_system::UpdateTessellatedTriangles) `metric_type` that compares each quad's metric against the
/// squared `manual_metric_value`. The name is descriptive; the engine uses a bare integer.
pub const TESSELLATION_METRIC_MANUAL: i32 = 1;
pub const TerrainPatchUpdate_ADDRESS: usize = 0x1410C98A0;
/// The per-patch CPU fragment for the base VolumetricTerrain patch maps (registered against the
/// [`TerrainPatchUpdate`](crate::patch_system::TerrainPatchUpdate) hash by `OldTerrainPatchSystem::TerrainPatchSystemCreateLayers`). It
/// advances patch construction and destruction (creating or destroying the patch's
/// [`RenderBlockTerrain`](crate::graphics_engine::render_block::RenderBlockTerrain) constant buffers,
/// updating the global terrain mask, and adjusting per-LOD memory statistics), and for constructed
/// patches it updates the block's alpha fade, sort id, and tessellated index count, then submits
/// the block to one render pass per set [`PatchHeader::m_VisibilityBits`](crate::patch_system::PatchHeader::m_VisibilityBits) bit:
/// [`FRUSTUM_ID_MAIN_CAMERA`](crate::patch_system::PatchFrustumId::FRUSTUM_ID_MAIN_CAMERA) routes to the color passes
/// (the near-detail pass for base-LOD tiles within one tile of the camera, the near pass for other
/// tiles at the context's high-detail LOD, and the far pass otherwise), the
/// `FRUSTUM_ID_SHADOW_PASS_*` ids to the static shadow passes (only when the terrain type renders
/// to shadow maps), the `FRUSTUM_ID_REFLECTIVE_SHADOW_*` ids to the reflective shadow passes, and
/// [`FRUSTUM_ID_DEPTH_PREPASS`](crate::patch_system::PatchFrustumId::FRUSTUM_ID_DEPTH_PREPASS) to the depth prepass.
pub unsafe fn TerrainPatchUpdate(
    patch_context: *mut crate::patch_system::PatchContext,
    fragment_context: *mut ::std::ffi::c_void,
) {
    unsafe {
        let f: unsafe extern "system" fn(
            patch_context: *mut crate::patch_system::PatchContext,
            fragment_context: *mut ::std::ffi::c_void,
        ) = ::std::mem::transmute(TerrainPatchUpdate_ADDRESS);
        f(patch_context, fragment_context)
    }
}
pub const TerrainPatchSystemFragment_ADDRESS: usize = 0x1410CAC50;
/// The per-patch CPU fragment for the volumetric-patch terrain maps (the
/// `CRenderBlockTerrainPatch` system — the terrain renderer active in the game world; the
/// [`TerrainPatchUpdate`](crate::patch_system::TerrainPatchUpdate) maps are dormant there). Structurally parallel to [`TerrainPatchUpdate`](crate::patch_system::TerrainPatchUpdate):
/// it advances patch construction and destruction, and for constructed patches updates the block's
/// alpha fade and tessellated index count, then submits the patch's render blocks to one render
/// pass per set [`PatchHeader::m_VisibilityBits`](crate::patch_system::PatchHeader::m_VisibilityBits) bit with the same frustum-id routing
/// ([`FRUSTUM_ID_MAIN_CAMERA`](crate::patch_system::PatchFrustumId::FRUSTUM_ID_MAIN_CAMERA) to the color passes, the
/// `FRUSTUM_ID_SHADOW_PASS_*` ids to the shadow passes, the ids from
/// [`FRUSTUM_ID_REFLECTIVE_SHADOW_0`](crate::patch_system::PatchFrustumId::FRUSTUM_ID_REFLECTIVE_SHADOW_0) up to
/// [`FRUSTUM_ID_DEPTH_PREPASS`](crate::patch_system::PatchFrustumId::FRUSTUM_ID_DEPTH_PREPASS), exclusive, to the
/// reflective shadow passes, and `FRUSTUM_ID_DEPTH_PREPASS` to the depth prepass). Base-LOD (9)
/// patches within the camera's 2x2 quadrant additionally submit per-quadrant sub-blocks to the
/// near/tessellation passes.
pub unsafe fn TerrainPatchSystemFragment(
    patch_context: *mut crate::patch_system::PatchContext,
    fragment_context: *mut ::std::ffi::c_void,
) {
    unsafe {
        let f: unsafe extern "system" fn(
            patch_context: *mut crate::patch_system::PatchContext,
            fragment_context: *mut ::std::ffi::c_void,
        ) = ::std::mem::transmute(TerrainPatchSystemFragment_ADDRESS);
        f(patch_context, fragment_context)
    }
}
pub const TreeDataPatch_ADDRESS: usize = 0x1410C8FA0;
/// A per-patch CPU fragment with no per-patch work (`TreeDataPatch`; identical
/// bodies may share this address): it only completes pending lifecycle transitions, finishing a
/// pending destruction (clearing [`PatchHeader::m_DynamicSize`](crate::patch_system::PatchHeader::m_DynamicSize) and setting
/// [`STATUS_DESTRUCTION_COMPLETE`](crate::patch_system::PatchHeader::STATUS_DESTRUCTION_COMPLETE)) or a pending
/// construction (setting
/// [`STATUS_CONSTRUCTION_COMPLETE`](crate::patch_system::PatchHeader::STATUS_CONSTRUCTION_COMPLETE)).
pub unsafe fn TreeDataPatch(
    patch_context: *mut crate::patch_system::PatchContext,
    fragment_context: *mut ::std::ffi::c_void,
) {
    unsafe {
        let f: unsafe extern "system" fn(
            patch_context: *mut crate::patch_system::PatchContext,
            fragment_context: *mut ::std::ffi::c_void,
        ) = ::std::mem::transmute(TreeDataPatch_ADDRESS);
        f(patch_context, fragment_context)
    }
}
pub const TerrainPatchSystemCreateLayers_ADDRESS: usize = 0x14032FAB0;
/// The base VolumetricTerrain patch system's construction-time setup
/// (`OldTerrainPatchSystem::TerrainPatchSystemCreateLayers`): for each of the four base LODs it
/// allocates the map's `CRenderBlockTerrain` array in constant memory, creates the blocks, and
/// registers the `terrain_patch_lod%d` patch map (fragment [`TerrainPatchUpdate`](crate::patch_system::TerrainPatchUpdate)) with a
/// dependency on the previous layer.
pub unsafe fn TerrainPatchSystemCreateLayers(
    ctx: *const ::std::ffi::c_void,
    system: *mut ::std::ffi::c_void,
) {
    unsafe {
        let f: unsafe extern "system" fn(
            ctx: *const ::std::ffi::c_void,
            system: *mut ::std::ffi::c_void,
        ) = ::std::mem::transmute(TerrainPatchSystemCreateLayers_ADDRESS);
        f(ctx, system)
    }
}
pub const UpdateTessellatedTriangles_ADDRESS: usize = 0x1410C9690;
/// Computes a streamed terrain patch's tessellated triangle count for the camera at `cam_pos`
/// (`patch` is the `STerrainStreamPatch`): with `metric_type`
/// [`TESSELLATION_METRIC_DISTANCE`](crate::patch_system::TESSELLATION_METRIC_DISTANCE) it scales each quad's tessellation by the camera's distance
/// beyond the patch bounds divided by `tess_factor_edge`; with
/// [`TESSELLATION_METRIC_MANUAL`](crate::patch_system::TESSELLATION_METRIC_MANUAL) it compares each quad's metric against `manual_metric_value`. The
/// terrain patch update stores the result on the patch's render block.
pub unsafe fn UpdateTessellatedTriangles(
    patch: *const ::std::ffi::c_void,
    cam_pos: *const crate::types::math::Vector3,
    metric_type: i32,
    manual_metric_value: f32,
    num_indices: u32,
    tess_factor_edge: f32,
) -> u64 {
    unsafe {
        let f: unsafe extern "system" fn(
            patch: *const ::std::ffi::c_void,
            cam_pos: *const crate::types::math::Vector3,
            metric_type: i32,
            manual_metric_value: f32,
            num_indices: u32,
            tess_factor_edge: f32,
        ) -> u64 = ::std::mem::transmute(UpdateTessellatedTriangles_ADDRESS);
        f(
            patch,
            cam_pos,
            metric_type,
            manual_metric_value,
            num_indices,
            tess_factor_edge,
        )
    }
}

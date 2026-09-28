#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// The occluder collection the BFBC occluder-frustum build draws candidates from. Opaque.
pub struct BFBCOccluderCollection {}
impl BFBCOccluderCollection {}
impl std::convert::AsRef<BFBCOccluderCollection> for BFBCOccluderCollection {
    fn as_ref(&self) -> &BFBCOccluderCollection {
        self
    }
}
impl std::convert::AsMut<BFBCOccluderCollection> for BFBCOccluderCollection {
    fn as_mut(&mut self) -> &mut BFBCOccluderCollection {
        self
    }
}
#[repr(C, align(8))]
/// A BFBC (beam-frustum-based-culling) occluder frustum: a set of planes the cull tests boxes
/// against. Opaque.
pub struct BFBCOccluderFrustum {}
impl BFBCOccluderFrustum {}
impl std::convert::AsRef<BFBCOccluderFrustum> for BFBCOccluderFrustum {
    fn as_ref(&self) -> &BFBCOccluderFrustum {
        self
    }
}
impl std::convert::AsMut<BFBCOccluderFrustum> for BFBCOccluderFrustum {
    fn as_mut(&mut self) -> &mut BFBCOccluderFrustum {
        self
    }
}
#[repr(C, align(8))]
/// The settings for the BFBC occluder-frustum build. Opaque.
pub struct BFBCOccluderFrustumSettings {}
impl BFBCOccluderFrustumSettings {}
impl std::convert::AsRef<BFBCOccluderFrustumSettings> for BFBCOccluderFrustumSettings {
    fn as_ref(&self) -> &BFBCOccluderFrustumSettings {
        self
    }
}
impl std::convert::AsMut<BFBCOccluderFrustumSettings> for BFBCOccluderFrustumSettings {
    fn as_mut(&mut self) -> &mut BFBCOccluderFrustumSettings {
        self
    }
}
#[repr(C, align(8))]
/// A BFBC process state. Opaque.
pub struct BFBCProcessState {}
impl BFBCProcessState {}
impl std::convert::AsRef<BFBCProcessState> for BFBCProcessState {
    fn as_ref(&self) -> &BFBCProcessState {
        self
    }
}
impl std::convert::AsMut<BFBCProcessState> for BFBCProcessState {
    fn as_mut(&mut self) -> &mut BFBCProcessState {
        self
    }
}
pub const BFBCCalculateCameraFrustum_ADDRESS: usize = 0x141E3B930;
/// Builds a six-plane camera cull frustum into `out` from a row-major view matrix and projection
/// matrix: it multiplies them and extracts the planes from the product (Gribb–Hartmann), so the
/// result is independent of the projection's handedness and depth convention.
pub unsafe fn BFBCCalculateCameraFrustum(
    view: *const f32,
    projection: *const f32,
    out: *mut crate::graphics_engine::bfbc::BFBCOccluderFrustum,
) {
    unsafe {
        let f: unsafe extern "system" fn(
            view: *const f32,
            projection: *const f32,
            out: *mut crate::graphics_engine::bfbc::BFBCOccluderFrustum,
        ) = ::std::mem::transmute(BFBCCalculateCameraFrustum_ADDRESS);
        f(view, projection, out)
    }
}
pub const BFBCCalculateOccluderFrustums_ADDRESS: usize = 0x141E46CA0;
/// Builds the occluder frustums for a cull: selects occluder candidates from `collection` with
/// `settings`, then builds a frustum per chosen occluder into `out_frustums`. Returns the number of
/// frustums built.
pub unsafe fn BFBCCalculateOccluderFrustums(
    collection: *mut crate::graphics_engine::bfbc::BFBCOccluderCollection,
    settings: *const crate::graphics_engine::bfbc::BFBCOccluderFrustumSettings,
    state: *mut crate::graphics_engine::bfbc::BFBCProcessState,
    out_frustums: *mut crate::graphics_engine::bfbc::BFBCOccluderFrustum,
) -> i32 {
    unsafe {
        let f: unsafe extern "system" fn(
            collection: *mut crate::graphics_engine::bfbc::BFBCOccluderCollection,
            settings: *const crate::graphics_engine::bfbc::BFBCOccluderFrustumSettings,
            state: *mut crate::graphics_engine::bfbc::BFBCProcessState,
            out_frustums: *mut crate::graphics_engine::bfbc::BFBCOccluderFrustum,
        ) -> i32 = ::std::mem::transmute(BFBCCalculateOccluderFrustums_ADDRESS);
        f(collection, settings, state, out_frustums)
    }
}

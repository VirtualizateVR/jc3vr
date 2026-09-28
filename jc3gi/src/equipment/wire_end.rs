#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// A latched grapple target (`CWireEnd::SHookTargetInfo`): the target position, plus the target or
/// game object it is relative to and the position in that object's space. The hook fills it from the aim state when it fires, and the wire end flies to and attaches at it.
pub struct HookTargetInfo {
    /// The target the hook homes on, or null for a target-less (surface) latch.
    pub m_HomingHookTarget: *mut crate::aim::target::Target,
    /// The world-space target position at the time it was latched.
    /// `CGrapplingHook::SetUpTarget` writes the aim slot's position here.
    pub m_TargetPosition: crate::types::math::Vector3,
    _field_14: [u8; 12],
    /// Whether the aim slot was in range when the target was latched.
    pub m_WasInRange: bool,
    _field_21: [u8; 7],
    /// The target game object (a `boost::weak_ptr<CGameObject>`: object pointer and weak control
    /// block), or null.
    pub m_TargetGameObject: [*mut ::std::ffi::c_void; 2],
}
fn _HookTargetInfo_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x38], HookTargetInfo>([0u8; 0x38]);
    }
    unreachable!()
}
impl HookTargetInfo {
    pub const SetupTargetFromTarget_ADDRESS: usize = 0x140926C00;
    /// Latches `target` with `local_position` in the target's game-object space.
    pub unsafe fn SetupTargetFromTarget(
        &mut self,
        target: *mut crate::aim::target::Target,
        local_position: *const crate::types::math::Vector3,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                target: *mut crate::aim::target::Target,
                local_position: *const crate::types::math::Vector3,
            ) = ::std::mem::transmute(Self::SetupTargetFromTarget_ADDRESS);
            f(self as *mut Self as _, target, local_position)
        }
    }
    pub const SetupTargetFromGameObject_ADDRESS: usize = 0x140926E50;
    /// Latches the game object `game_object` (a `CGameObject`) with `local_position` in its space.
    pub unsafe fn SetupTargetFromGameObject(
        &mut self,
        game_object: *mut ::std::ffi::c_void,
        local_position: *const crate::types::math::Vector3,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                game_object: *mut ::std::ffi::c_void,
                local_position: *const crate::types::math::Vector3,
            ) = ::std::mem::transmute(Self::SetupTargetFromGameObject_ADDRESS);
            f(self as *mut Self as _, game_object, local_position)
        }
    }
    pub const GetTargetWorldPosition_ADDRESS: usize = 0x140927040;
    /// Writes the latched target's current world position, following the target object when there
    /// is one.
    pub unsafe fn GetTargetWorldPosition(
        &self,
        result: *mut crate::types::math::Vector3,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                result: *mut crate::types::math::Vector3,
            ) = ::std::mem::transmute(Self::GetTargetWorldPosition_ADDRESS);
            f(self as *const Self as _, result)
        }
    }
}
impl std::convert::AsRef<HookTargetInfo> for HookTargetInfo {
    fn as_ref(&self) -> &HookTargetInfo {
        self
    }
}
impl std::convert::AsMut<HookTargetInfo> for HookTargetInfo {
    fn as_mut(&mut self) -> &mut HookTargetInfo {
        self
    }
}
#[repr(C, align(8))]
/// One end of a grapple wire (`CWireEnd`). A `CGrapplingHookWire` holds two: the near end at the
/// device and the far end at the hook. An end extends toward a latched [`HookTargetInfo`](crate::equipment::wire_end::HookTargetInfo), then
/// attaches to a proxy (`CGrapplePointProxy`), not to a bare coordinate. The layout is unmapped.
pub struct WireEnd {}
impl WireEnd {
    pub const SetExtend_ADDRESS: usize = 0x14093D700;
    /// Puts the end into its extending state: it flies from `start_pos` at `speed` toward
    /// `target_info`, as a ghost wire when `ghost_wire`.
    pub unsafe fn SetExtend(
        &mut self,
        start_pos: *const crate::types::math::Vector3,
        speed: f32,
        target_info: *mut crate::equipment::wire_end::HookTargetInfo,
        ghost_wire: bool,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                start_pos: *const crate::types::math::Vector3,
                speed: f32,
                target_info: *mut crate::equipment::wire_end::HookTargetInfo,
                ghost_wire: bool,
            ) = ::std::mem::transmute(Self::SetExtend_ADDRESS);
            f(self as *mut Self as _, start_pos, speed, target_info, ghost_wire)
        }
    }
    pub const Attach_ADDRESS: usize = 0x14094AFB0;
    /// Attaches the end at `hit_pos`: builds the attachment proxy from the ray hit `cast_info` (a
    /// `CCastRayInfo`) with `SetupProxyForAttachment` and moves the end to its attached state.
    pub unsafe fn Attach(
        &mut self,
        hit_pos: *const crate::types::math::Vector3,
        cast_info: *mut ::std::ffi::c_void,
        hook_owner: *const crate::types::shared_ptr::SharedPtr<
            crate::character::character::Character,
        >,
        reattaching: bool,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                hit_pos: *const crate::types::math::Vector3,
                cast_info: *mut ::std::ffi::c_void,
                hook_owner: *const crate::types::shared_ptr::SharedPtr<
                    crate::character::character::Character,
                >,
                reattaching: bool,
            ) = ::std::mem::transmute(Self::Attach_ADDRESS);
            f(self as *mut Self as _, hit_pos, cast_info, hook_owner, reattaching)
        }
    }
    pub const AttachToProxy_ADDRESS: usize = 0x1408F4850;
    /// Attaches the end to an existing attachment proxy (a `CGrapplePointProxy`).
    pub unsafe fn AttachToProxy(&mut self, proxy: *mut ::std::ffi::c_void) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                proxy: *mut ::std::ffi::c_void,
            ) = ::std::mem::transmute(Self::AttachToProxy_ADDRESS);
            f(self as *mut Self as _, proxy)
        }
    }
    pub const AttachToCharacterBone_ADDRESS: usize = 0x1408F91F0;
    /// Attaches the end to bone `bone` of `character` at `local_offset`, through a new
    /// `CCharacterBoneAttachedPointProxy`.
    pub unsafe fn AttachToCharacterBone(
        &mut self,
        character: *const crate::types::shared_ptr::SharedPtr<
            crate::character::character::Character,
        >,
        bone: i32,
        local_offset: *const crate::types::math::Matrix4,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                character: *const crate::types::shared_ptr::SharedPtr<
                    crate::character::character::Character,
                >,
                bone: i32,
                local_offset: *const crate::types::math::Matrix4,
            ) = ::std::mem::transmute(Self::AttachToCharacterBone_ADDRESS);
            f(self as *mut Self as _, character, bone, local_offset)
        }
    }
}
impl std::convert::AsRef<WireEnd> for WireEnd {
    fn as_ref(&self) -> &WireEnd {
        self
    }
}
impl std::convert::AsMut<WireEnd> for WireEnd {
    fn as_mut(&mut self) -> &mut WireEnd {
        self
    }
}

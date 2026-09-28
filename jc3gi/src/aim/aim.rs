#![cfg_attr(any(), rustfmt::skip)]
#[repr(i32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// The player's aim mode (`CPlayer::EAimModes`): normal or precision aim. `CPlayer::SetAimMode`
/// switches it and passes it on to the wielded weapon's
/// [`SetAimMode`](crate::equipment::weapon::WeaponBase::SetAimMode).
pub enum AimMode {
    AIM_NORMAL = 0isize as _,
    AIM_PRECISION = 1isize as _,
    AIM_MAX = 2isize as _,
}
fn _AimMode_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], AimMode>([0u8; 0x4]);
    }
    unreachable!()
}
#[repr(i32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// How the player's aim is assisted (`CPlayerAimControl::EAimType`), stored in
/// [`PlayerAimControl::m_AimType`](crate::aim::aim::PlayerAimControl::m_AimType). `CPlayer::SetAimMode` selects normal aim in precision aim
/// (unless the weapon keeps aim assist in precision aim) and auto aim otherwise, and forces normal
/// aim when the aim-assist setting is off.
pub enum AimType {
    ENone = 0isize as _,
    ENormal = 1isize as _,
    EAuto = 2isize as _,
}
fn _AimType_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], AimType>([0u8; 0x4]);
    }
    unreachable!()
}
#[repr(C, align(8))]
/// The local player's aim state (`CPlayerAimControl`, an `ITargetSystemListener`), owned by the
/// player. It is a target cache indexed by [`TargetType`](crate::aim::target::TargetType): every per-purpose array holds one slot
/// per target type, filled each tick from one camera-derived raycast
/// ([`UpdateDirectAim`](crate::aim::aim::PlayerAimControl::UpdateDirectAim)) and the target system's fitness scoring
/// ([`UpdateAllTargets`](crate::aim::aim::PlayerAimControl::UpdateAllTargets)). The weapons, grapple, melee, and grenade code
/// read their own slot. In normal third-person play the aim is camera-derived.
pub struct PlayerAimControl {
    pub __vftable: u64,
    /// Per slot, whether the direct-aim ray hit something within the slot's range.
    pub m_DirectHits: [bool; 6],
    _field_e: [u8; 2],
    /// Per slot, the surface normal of the last direct-aim hit.
    pub m_LastDirectAimHitNormals: [crate::types::math::Vector3; 6],
    /// Per slot, the target the direct-aim ray hit, or null.
    pub m_DirectTargets: [*mut crate::aim::target::Target; 6],
    /// Per slot, the selected target: the direct target or the best-scoring candidate from aim
    /// assist, or null.
    pub m_Target: [*mut crate::aim::target::Target; 6],
    _field_b8: [u8; 48],
    /// Per slot, the ray variant that fills the slot.
    pub m_RayTypeSetting: [crate::aim::aim::RayType; 6],
    /// Per slot, the direct-aim hit position, or the point at the slot's range along the ray when
    /// nothing was hit.
    pub m_DirectPositions: [crate::types::math::Vector3; 6],
    /// Whether the direct grapple hit is on a grapplable surface.
    pub m_DirectGrapplePosIsGrapplable: bool,
    _field_149: [u8; 3],
    /// Per slot, the world-space aim position: the direct position, pulled toward the selected
    /// target by aim assist. Slot [`TARGET_WEAPONS`](crate::aim::target::TargetType::TARGET_WEAPONS) is the point every
    /// weapon aims at, and slot [`TARGET_GRAPPLE`](crate::aim::target::TargetType::TARGET_GRAPPLE) is the grapple aim
    /// point.
    pub m_AimPos: [crate::types::math::Vector3; 6],
    /// Per slot, [`m_AimPos`](crate::aim::aim::PlayerAimControl::m_AimPos) as of the previous tick. The grapple reticle
    /// interpolates from it toward the current position.
    pub m_LastAimPos: [crate::types::math::Vector3; 6],
    /// Per slot, whether the aim position is within the slot's range.
    pub m_IsInRange: [bool; 6],
    /// Per slot, whether the direct position is within the slot's range.
    pub m_DirectPosIsInRange: [bool; 6],
    _field_1e8: [u8; 6],
    /// Whether the grapple slot uses the direct position rather than a target; the grapple reticle
    /// is then centred on the screen instead of projected.
    pub m_IsGrappleUsingDirectPos: bool,
    /// Whether the last-valid grapple cache below holds a target.
    pub m_HasLastValidGrapple: bool,
    /// The cached grapple aim position, kept so a momentarily lost grapple target persists.
    pub m_LastValidGrappleAimPos: crate::types::math::Vector3,
    /// The surface normal at [`m_LastValidGrappleAimPos`](crate::aim::aim::PlayerAimControl::m_LastValidGrappleAimPos).
    pub m_LastValidGrappleNormal: crate::types::math::Vector3,
    /// The cached grapple target, or null.
    pub m_LastValidGrappleTarget: *mut crate::aim::target::Target,
    /// The cached grapple game object (a `boost::weak_ptr<CGameObject>`: object pointer and weak
    /// control block).
    pub m_LastHitGrappleTargetGO: [*mut ::std::ffi::c_void; 2],
    _field_220: [u8; 96],
    /// The length of the direct-aim ray: the override distance when positive, otherwise the larger
    /// of the weapon and grapple slots' maximum aim distances.
    pub m_RayCastDistance: f32,
    /// Per slot, the maximum aim distance, recomputed by [`UpdateRange`](crate::aim::aim::PlayerAimControl::UpdateRange): the
    /// weapon slot takes the largest weapon range over the selected, wingsuit, and vehicle weapons
    /// (default 100), the sticky and snap slots copy it, the grapple slot takes the grapple
    /// tuning's range, the grenade slot the grenade weapon's range, and the melee slot is zero.
    pub m_MaxAimDistance: [f32; 6],
    /// A ray length that replaces the computed one when positive.
    pub m_OverrideRayCastDistance: f32,
    _field_2a0: [u8; 4],
    /// Whether and how aim assist applies.
    pub m_AimType: crate::aim::aim::AimType,
    _field_2a8: [u8; 48],
    /// The character this aim control serves.
    pub m_Owner: *mut crate::character::character::Character,
}
fn _PlayerAimControl_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x2E0], PlayerAimControl>([0u8; 0x2E0]);
    }
    unreachable!()
}
impl PlayerAimControl {
    pub const GetAdjustedCameraMatrix_ADDRESS: usize = 0x140C3E510;
    /// Returns the matrix used by the aim raycast: in aim-down-sights it reads the alternate-aim
    /// transform, otherwise the camera transform. `weapon` adds an optional ballistic pitch. The
    /// result is written through `result` and also returned.
    pub unsafe fn GetAdjustedCameraMatrix(
        result: *mut crate::types::math::Matrix4,
        weapon: *mut crate::equipment::weapon::WeaponBase,
    ) -> *mut crate::types::math::Matrix4 {
        unsafe {
            let f: unsafe extern "system" fn(
                result: *mut crate::types::math::Matrix4,
                weapon: *mut crate::equipment::weapon::WeaponBase,
            ) -> *mut crate::types::math::Matrix4 = ::std::mem::transmute(
                Self::GetAdjustedCameraMatrix_ADDRESS,
            );
            f(result, weapon)
        }
    }
    pub const UpdateDirectAim_ADDRESS: usize = 0x140CE5350;
    /// Raycasts from the camera position along camera-forward to find the aim target.
    pub unsafe fn UpdateDirectAim(&mut self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) = ::std::mem::transmute(
                Self::UpdateDirectAim_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
    pub const UpdatePostSim_ADDRESS: usize = 0x140CF5C50;
    /// The per-tick aim update after the camera for `character`: runs
    /// [`UpdateInput`](crate::aim::aim::PlayerAimControl::UpdateInput), [`UpdateRange`](crate::aim::aim::PlayerAimControl::UpdateRange),
    /// [`UpdateDirectAim`](crate::aim::aim::PlayerAimControl::UpdateDirectAim), [`UpdateAllTargets`](crate::aim::aim::PlayerAimControl::UpdateAllTargets),
    /// and `UpdateLastValidGrappleTarget`.
    pub unsafe fn UpdatePostSim(
        &mut self,
        dt: f32,
        character: *mut crate::character::character::Character,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                dt: f32,
                character: *mut crate::character::character::Character,
            ) = ::std::mem::transmute(Self::UpdatePostSim_ADDRESS);
            f(self as *mut Self as _, dt, character)
        }
    }
    pub const UpdatePreSim_ADDRESS: usize = 0x140C65920;
    /// The pre-simulation aim update, which polls the aim control's pending raycast requests for
    /// completion.
    pub unsafe fn UpdatePreSim(&mut self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) = ::std::mem::transmute(
                Self::UpdatePreSim_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
    pub const UpdateInput_ADDRESS: usize = 0x140CE9210;
    /// Folds the aim stick input and applies the yaw and pitch dead zones.
    pub unsafe fn UpdateInput(&mut self, dt: f32) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self, dt: f32) = ::std::mem::transmute(
                Self::UpdateInput_ADDRESS,
            );
            f(self as *mut Self as _, dt)
        }
    }
    pub const UpdateRange_ADDRESS: usize = 0x140CA88C0;
    /// Recomputes [`m_MaxAimDistance`](crate::aim::aim::PlayerAimControl::m_MaxAimDistance) and
    /// [`m_RayCastDistance`](crate::aim::aim::PlayerAimControl::m_RayCastDistance) from the current weapon, wingsuit, vehicle,
    /// grapple, and grenade tuning.
    pub unsafe fn UpdateRange(&mut self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) = ::std::mem::transmute(
                Self::UpdateRange_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
    pub const UpdateAllTargets_ADDRESS: usize = 0x140CE7690;
    /// Target selection and aim assist: scores the target system's candidates per slot with the
    /// per-purpose fitness functions, picks [`m_Target`](crate::aim::aim::PlayerAimControl::m_Target), and pulls
    /// [`m_AimPos`](crate::aim::aim::PlayerAimControl::m_AimPos) toward the selected targets.
    pub unsafe fn UpdateAllTargets(&mut self, dt: f32) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self, dt: f32) = ::std::mem::transmute(
                Self::UpdateAllTargets_ADDRESS,
            );
            f(self as *mut Self as _, dt)
        }
    }
    pub const CheckDirectTargets_ADDRESS: usize = 0x140CE4BB0;
    /// Distributes the direct-aim ray's hit into every slot except melee: fills
    /// [`m_DirectHits`](crate::aim::aim::PlayerAimControl::m_DirectHits), [`m_DirectPositions`](crate::aim::aim::PlayerAimControl::m_DirectPositions),
    /// [`m_AimPos`](crate::aim::aim::PlayerAimControl::m_AimPos), [`m_IsInRange`](crate::aim::aim::PlayerAimControl::m_IsInRange), and
    /// [`m_DirectTargets`](crate::aim::aim::PlayerAimControl::m_DirectTargets) per slot according to its
    /// [`m_RayTypeSetting`](crate::aim::aim::PlayerAimControl::m_RayTypeSetting) and range. `collector` is the ray's
    /// `CPlayerAimRaycastFilterCacheFirstHoleyHitAndMinDistanceHit` filter and `non_holey_info`
    /// its `CCastRayInfo`.
    pub unsafe fn CheckDirectTargets(
        &mut self,
        collector: *const ::std::ffi::c_void,
        non_holey_info: *mut ::std::ffi::c_void,
        ray_start: *const crate::types::math::Vector3,
        ray_dir: *const crate::types::math::Vector3,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                collector: *const ::std::ffi::c_void,
                non_holey_info: *mut ::std::ffi::c_void,
                ray_start: *const crate::types::math::Vector3,
                ray_dir: *const crate::types::math::Vector3,
            ) = ::std::mem::transmute(Self::CheckDirectTargets_ADDRESS);
            f(self as *mut Self as _, collector, non_holey_info, ray_start, ray_dir)
        }
    }
    pub const UpdateCrosshairLook_ADDRESS: usize = 0x140CE8BE0;
    /// Updates the per-slot crosshair look (`CPlayerAimControl::ECrosshairLook`: in range with a
    /// neutral, allied, or hostile target, in range with no target, or out of range).
    pub unsafe fn UpdateCrosshairLook(&mut self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) = ::std::mem::transmute(
                Self::UpdateCrosshairLook_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
    pub const GetCurrentWeapon_ADDRESS: usize = 0x140C65E10;
    /// Returns the weapon that serves `target_type` through `result`, or a null pointer.
    pub unsafe fn GetCurrentWeapon(
        &self,
        result: *mut crate::types::shared_ptr::SharedPtr<
            crate::equipment::weapon::WeaponBase,
        >,
        target_type: crate::aim::target::TargetType,
    ) -> *mut crate::types::shared_ptr::SharedPtr<crate::equipment::weapon::WeaponBase> {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                result: *mut crate::types::shared_ptr::SharedPtr<
                    crate::equipment::weapon::WeaponBase,
                >,
                target_type: crate::aim::target::TargetType,
            ) -> *mut crate::types::shared_ptr::SharedPtr<
                crate::equipment::weapon::WeaponBase,
            > = ::std::mem::transmute(Self::GetCurrentWeapon_ADDRESS);
            f(self as *const Self as _, result, target_type)
        }
    }
    pub const GetRaycastStartPosition_ADDRESS: usize = 0x140C2B610;
    /// Returns the camera position moved 0.1 units along camera-forward, from the unadjusted
    /// camera matrix. The aim-distance measurements use it; the raycast computes its own origin.
    pub unsafe fn GetRaycastStartPosition(
        &self,
        result: *mut crate::types::math::Vector3,
    ) -> *mut crate::types::math::Vector3 {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                result: *mut crate::types::math::Vector3,
            ) -> *mut crate::types::math::Vector3 = ::std::mem::transmute(
                Self::GetRaycastStartPosition_ADDRESS,
            );
            f(self as *const Self as _, result)
        }
    }
    pub const GetRaycastMinDistanceStartPosition_ADDRESS: usize = 0x140C65B10;
    /// Returns a start point blended from the wielded weapon's barrel and the character's head,
    /// against the aim matrix `aim_mat`. The raycast uses its distance along the ray as the
    /// minimum-distance hit fraction.
    pub unsafe fn GetRaycastMinDistanceStartPosition(
        &self,
        result: *mut crate::types::math::Vector3,
        aim_mat: *mut crate::types::math::Matrix4,
    ) -> *mut crate::types::math::Vector3 {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                result: *mut crate::types::math::Vector3,
                aim_mat: *mut crate::types::math::Matrix4,
            ) -> *mut crate::types::math::Vector3 = ::std::mem::transmute(
                Self::GetRaycastMinDistanceStartPosition_ADDRESS,
            );
            f(self as *const Self as _, result, aim_mat)
        }
    }
    pub const GetTargetBodyPosition_ADDRESS: usize = 0x140CDA570;
    /// Returns a target's body aim position through `result`, optionally for a grenade throw,
    /// with an adjusted height, or clamped to the hip.
    pub unsafe fn GetTargetBodyPosition(
        result: *mut crate::types::math::Vector3,
        target: *mut crate::aim::target::Target,
        for_grenade: bool,
        adjust_y_pos: bool,
        clamp_to_hip: bool,
    ) -> *mut crate::types::math::Vector3 {
        unsafe {
            let f: unsafe extern "system" fn(
                result: *mut crate::types::math::Vector3,
                target: *mut crate::aim::target::Target,
                for_grenade: bool,
                adjust_y_pos: bool,
                clamp_to_hip: bool,
            ) -> *mut crate::types::math::Vector3 = ::std::mem::transmute(
                Self::GetTargetBodyPosition_ADDRESS,
            );
            f(result, target, for_grenade, adjust_y_pos, clamp_to_hip)
        }
    }
    pub const GetAverageWeaponTargetPosition_ADDRESS: usize = 0x140C14230;
    /// Copies the weapon slot's aim position (`m_AimPos[TARGET_WEAPONS]`) to `result`. The
    /// body sits beside
    /// [`GetActiveGrappleTargetPosition`](crate::aim::aim::PlayerAimControl::GetActiveGrappleTargetPosition); it has no direct
    /// callers, as callers read the field inline.
    pub unsafe fn GetAverageWeaponTargetPosition(
        &self,
        result: *mut crate::types::math::Vector3,
    ) -> *mut crate::types::math::Vector3 {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                result: *mut crate::types::math::Vector3,
            ) -> *mut crate::types::math::Vector3 = ::std::mem::transmute(
                Self::GetAverageWeaponTargetPosition_ADDRESS,
            );
            f(self as *const Self as _, result)
        }
    }
    pub const GetActiveGrappleTargetPosition_ADDRESS: usize = 0x140C14250;
    /// Returns a pointer to the grapple slot's aim position (`m_AimPos[TARGET_GRAPPLE]`).
    pub unsafe fn GetActiveGrappleTargetPosition(
        &self,
    ) -> *const crate::types::math::Vector3 {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
            ) -> *const crate::types::math::Vector3 = ::std::mem::transmute(
                Self::GetActiveGrappleTargetPosition_ADDRESS,
            );
            f(self as *const Self as _)
        }
    }
    pub const GetCurrentGrappleTargetGO_ADDRESS: usize = 0x140C554B0;
    /// Returns the cached grapple game object, or null unless
    /// [`m_HasLastValidGrapple`](crate::aim::aim::PlayerAimControl::m_HasLastValidGrapple) and the grapple slot's
    /// [`m_IsInRange`](crate::aim::aim::PlayerAimControl::m_IsInRange) are both set.
    pub unsafe fn GetCurrentGrappleTargetGO(&self) -> *mut ::std::ffi::c_void {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
            ) -> *mut ::std::ffi::c_void = ::std::mem::transmute(
                Self::GetCurrentGrappleTargetGO_ADDRESS,
            );
            f(self as *const Self as _)
        }
    }
    pub const GetCurrentGrappleTargetWeakGO_ADDRESS: usize = 0x140C2B5C0;
    /// The weak-pointer form of [`GetCurrentGrappleTargetGO`](crate::aim::aim::PlayerAimControl::GetCurrentGrappleTargetGO):
    /// copies [`m_LastHitGrappleTargetGO`](crate::aim::aim::PlayerAimControl::m_LastHitGrappleTargetGO) into `result` under the
    /// same conditions, and a null weak pointer otherwise.
    pub unsafe fn GetCurrentGrappleTargetWeakGO(
        &self,
        result: *mut [*mut ::std::ffi::c_void; 2],
    ) -> *mut [*mut ::std::ffi::c_void; 2] {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                result: *mut [*mut ::std::ffi::c_void; 2],
            ) -> *mut [*mut ::std::ffi::c_void; 2] = ::std::mem::transmute(
                Self::GetCurrentGrappleTargetWeakGO_ADDRESS,
            );
            f(self as *const Self as _, result)
        }
    }
    pub const IsLastValidGrappleAimPosStillValid_ADDRESS: usize = 0x140C3E5F0;
    /// Re-validates the last-valid grapple cache against the current camera: rejects it when out
    /// of the grapple range from the camera or outside the grapple tuning's view angles, and
    /// otherwise re-queues its visibility raycast.
    pub unsafe fn IsLastValidGrappleAimPosStillValid(&mut self) -> bool {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) -> bool = ::std::mem::transmute(
                Self::IsLastValidGrappleAimPosStillValid_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
}
impl std::convert::AsRef<PlayerAimControl> for PlayerAimControl {
    fn as_ref(&self) -> &PlayerAimControl {
        self
    }
}
impl std::convert::AsMut<PlayerAimControl> for PlayerAimControl {
    fn as_mut(&mut self) -> &mut PlayerAimControl {
        self
    }
}
#[repr(C, align(8))]
/// The animation-graph condition that the evaluating character is the player's current weapon
/// target (`CPlayerIsAutoAimingCondition`). Its layout is unmapped.
pub struct PlayerIsAutoAimingCondition {}
impl PlayerIsAutoAimingCondition {
    pub const Evaluate_ADDRESS: usize = 0x1407E0620;
    /// Evaluates the condition for the character owning `instance` (a
    /// `NAnimationSystem::CStateMachineInstance`): true when the player's
    /// `m_Target[TARGET_WEAPONS]` is that character and is not also the direct target.
    pub unsafe fn Evaluate(&self, instance: *const ::std::ffi::c_void) -> bool {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                instance: *const ::std::ffi::c_void,
            ) -> bool = ::std::mem::transmute(Self::Evaluate_ADDRESS);
            f(self as *const Self as _, instance)
        }
    }
}
impl std::convert::AsRef<PlayerIsAutoAimingCondition> for PlayerIsAutoAimingCondition {
    fn as_ref(&self) -> &PlayerIsAutoAimingCondition {
        self
    }
}
impl std::convert::AsMut<PlayerIsAutoAimingCondition> for PlayerIsAutoAimingCondition {
    fn as_mut(&mut self) -> &mut PlayerIsAutoAimingCondition {
        self
    }
}
#[repr(i32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// The raycast variant used for an aim slot (`CPlayerAimControl::ERayType`), stored per
/// [`TargetType`](crate::aim::target::TargetType) in [`PlayerAimControl::m_RayTypeSetting`](crate::aim::aim::PlayerAimControl::m_RayTypeSetting). It selects which of the direct-aim
/// ray's recorded hits fills the slot.
pub enum RayType {
    HIT_HOLEY = 0isize as _,
    SKIP_HOLEY = 1isize as _,
    HIT_MIN_DISTANCE = 2isize as _,
}
fn _RayType_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], RayType>([0u8; 0x4]);
    }
    unreachable!()
}
pub const NAutoAimToTarget_Update_ADDRESS: usize = 0x140809C60;
/// The character-state task that overrides the aim direction toward a locked target.
unsafe fn NAutoAimToTarget_Update(
    ctx: *mut crate::state::StateContext,
    p: *mut ::std::ffi::c_void,
) {
    unsafe {
        let f: unsafe extern "system" fn(
            ctx: *mut crate::state::StateContext,
            p: *mut ::std::ffi::c_void,
        ) = ::std::mem::transmute(NAutoAimToTarget_Update_ADDRESS);
        f(ctx, p)
    }
}

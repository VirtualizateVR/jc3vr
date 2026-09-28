#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// The grappling-hook equipment item (`CGrapplingHook`, a `CGameObject`): owns the wires, the
/// reel/tether state machine, and the cached fire target. A character's hook is held by its
/// inventory ([`Inventory::m_GrapplingHook`](crate::character::inventory::Inventory::m_GrapplingHook)).
pub struct GrapplingHook {
    _field_0: [u8; 564],
    /// The current state-machine state; see [`GrapplingHookState`](crate::equipment::grappling_hook::GrapplingHookState). The state records the *last*
    /// reel outcome as much as the current one: leaving a `GHS_REELED_*` attachment does not
    /// reliably return it to [`GHS_INACTIVE`](crate::equipment::grappling_hook::GrapplingHookState::GHS_INACTIVE) — readers such as
    /// `IsHookAimingState` and `ShouldShowAttachedUI` therefore combine it with
    /// [`m_ActiveWire`](crate::equipment::grappling_hook::GrapplingHook::m_ActiveWire) rather than trusting it alone.
    pub m_State: crate::equipment::grappling_hook::GrapplingHookState,
    /// The latched fire target, filled by [`SetUpTarget`](crate::equipment::grappling_hook::GrapplingHook::SetUpTarget) when the hook fires.
    /// [`GetLastGrappleTargetPos`](crate::equipment::grappling_hook::GrapplingHook::GetLastGrappleTargetPos) reads its current world position,
    /// so the fire and its arm IK follow the latched target rather than the live aim slot.
    pub m_HookFireInfo: crate::equipment::wire_end::HookTargetInfo,
    /// The countdown (seconds) armed when a hook fire is committed, within which the fire
    /// animation must emit its release track message before the fire is abandoned. Set to `2.0`
    /// alongside [`m_WaitingForGrappleFire`](crate::equipment::grappling_hook::GrapplingHook::m_WaitingForGrappleFire) by the fire
    /// dispatchers (`FireHookTowardsPos`, `TryFireHook`).
    pub m_WaitForFireSignalTimeout: f32,
    /// Whether a grapple-hook fire is committed and awaiting the fire animation's release track
    /// message: set when the fire dispatches (before the fire act plays and the wire spawns),
    /// cleared when the hook releases or the fire times out. The fire act's directional alignment
    /// of the character runs within this window.
    pub m_WaitingForGrappleFire: bool,
    /// The dual-tether counterpart of
    /// [`m_WaitingForGrappleFire`](crate::equipment::grappling_hook::GrapplingHook::m_WaitingForGrappleFire); the two are cleared
    /// together as one word.
    pub m_WaitingForTetherFire: bool,
    _field_276: [u8; 162],
    /// The cached reel-in target position in world space. The reel-in aim IK turns the chest
    /// toward it while a wire is active. The offset is confirmed; the name is
    /// unconfirmed.
    pub m_CachedReelTargetPosition: crate::types::math::Vector3,
    _field_324: [u8; 164],
    /// The wire currently driving the reel/attach flow, or null when none is live. The state
    /// readers null-check this before interpreting [`m_State`](crate::equipment::grappling_hook::GrapplingHook::m_State).
    pub m_ActiveWire: crate::types::shared_ptr::SharedPtr<
        crate::equipment::grappling_hook::GrapplingHookWire,
    >,
}
fn _GrapplingHook_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x3D8], GrapplingHook>([0u8; 0x3D8]);
    }
    unreachable!()
}
impl GrapplingHook {
    pub const SetUpTarget_ADDRESS: usize = 0x140939CF0;
    /// Latches the `player`'s (a `CPlayer`) aim slot `target_type` into `target_info`: reads the
    /// slot's [`m_AimPos`](crate::aim::aim::PlayerAimControl::m_AimPos) and
    /// [`m_Target`](crate::aim::aim::PlayerAimControl::m_Target) from the player's aim control, stores the
    /// position in the target object's space, and falls back to the aim control's current grapple
    /// game object only for [`TARGET_GRAPPLE`](crate::aim::target::TargetType::TARGET_GRAPPLE). Later changes to the
    /// aim slot do not move the latched target.
    pub unsafe fn SetUpTarget(
        player: *mut ::std::ffi::c_void,
        target_info: *mut crate::equipment::wire_end::HookTargetInfo,
        target_type: crate::aim::target::TargetType,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                player: *mut ::std::ffi::c_void,
                target_info: *mut crate::equipment::wire_end::HookTargetInfo,
                target_type: crate::aim::target::TargetType,
            ) = ::std::mem::transmute(Self::SetUpTarget_ADDRESS);
            f(player, target_info, target_type)
        }
    }
    pub const GetLastGrappleTargetPos_ADDRESS: usize = 0x14093A4B0;
    /// Writes the latched fire target's current world position
    /// ([`m_HookFireInfo`](crate::equipment::grappling_hook::GrapplingHook::m_HookFireInfo)) and returns true.
    pub unsafe fn GetLastGrappleTargetPos(
        &self,
        position: *mut crate::types::math::Vector3,
    ) -> bool {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                position: *mut crate::types::math::Vector3,
            ) -> bool = ::std::mem::transmute(Self::GetLastGrappleTargetPos_ADDRESS);
            f(self as *const Self as _, position)
        }
    }
    pub const TryFireHook_ADDRESS: usize = 0x1409432E0;
    /// Computes the fire direction toward the latched target and hands it to
    /// [`DoActFire`](crate::equipment::grappling_hook::GrapplingHook::DoActFire).
    pub unsafe fn TryFireHook(
        &mut self,
        holder: *const crate::types::shared_ptr::SharedPtr<
            crate::character::character::Character,
        >,
        dual_tether_fire: bool,
        grapple_reel_fire: bool,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                holder: *const crate::types::shared_ptr::SharedPtr<
                    crate::character::character::Character,
                >,
                dual_tether_fire: bool,
                grapple_reel_fire: bool,
            ) = ::std::mem::transmute(Self::TryFireHook_ADDRESS);
            f(self as *mut Self as _, holder, dual_tether_fire, grapple_reel_fire)
        }
    }
    pub const DoActFire_ADDRESS: usize = 0x1408FED80;
    /// Queues the directional fire act for `aim_direction` on `holder`: picks the
    /// `ACT_GRAPPLE_FIRE_*` sector (or `ACT_GRAPPLE_TETHER_FIRE_*` for a dual tether) from the
    /// angle between the direction and the character. No wire is created here; the hook spawns on
    /// the fire animation's release track message.
    pub unsafe fn DoActFire(
        aim_direction: *const crate::types::math::Vector3,
        holder: *const crate::types::shared_ptr::SharedPtr<
            crate::character::character::Character,
        >,
        dual_tether_fire: bool,
        grapple_reel_fire: bool,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                aim_direction: *const crate::types::math::Vector3,
                holder: *const crate::types::shared_ptr::SharedPtr<
                    crate::character::character::Character,
                >,
                dual_tether_fire: bool,
                grapple_reel_fire: bool,
            ) = ::std::mem::transmute(Self::DoActFire_ADDRESS);
            f(aim_direction, holder, dual_tether_fire, grapple_reel_fire)
        }
    }
    pub const OnPreSimUpdate_ADDRESS: usize = 0x140949F70;
    /// The pre-simulation hook update: dispatches the grapple input (fire, reel, dual tether,
    /// retract, release) and advances the reel and tether state.
    pub unsafe fn OnPreSimUpdate(&mut self, dt: f32) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self, dt: f32) = ::std::mem::transmute(
                Self::OnPreSimUpdate_ADDRESS,
            );
            f(self as *mut Self as _, dt)
        }
    }
    pub const OnPostCamUpdate_ADDRESS: usize = 0x14094ED60;
    /// The post-camera hook update. On the fire animation's release frame it creates the wire
    /// ([`CreateNewWire`](crate::equipment::grappling_hook::GrapplingHook::CreateNewWire)), pins the near end to the device, and starts the
    /// far end extending toward the latched target
    /// ([`WireEnd::SetExtend`](crate::equipment::wire_end::WireEnd::SetExtend)).
    pub unsafe fn OnPostCamUpdate(&mut self, dt: f32) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self, dt: f32) = ::std::mem::transmute(
                Self::OnPostCamUpdate_ADDRESS,
            );
            f(self as *mut Self as _, dt)
        }
    }
    pub const CreateNewWire_ADDRESS: usize = 0x140923BC0;
    /// Spawns a new wire at `spawn_pos` and returns it through `result`.
    pub unsafe fn CreateNewWire(
        &mut self,
        result: *mut crate::types::shared_ptr::SharedPtr<
            crate::equipment::grappling_hook::GrapplingHookWire,
        >,
        spawn_pos: *const crate::types::math::Vector3,
    ) -> *mut crate::types::shared_ptr::SharedPtr<
        crate::equipment::grappling_hook::GrapplingHookWire,
    > {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                result: *mut crate::types::shared_ptr::SharedPtr<
                    crate::equipment::grappling_hook::GrapplingHookWire,
                >,
                spawn_pos: *const crate::types::math::Vector3,
            ) -> *mut crate::types::shared_ptr::SharedPtr<
                crate::equipment::grappling_hook::GrapplingHookWire,
            > = ::std::mem::transmute(Self::CreateNewWire_ADDRESS);
            f(self as *mut Self as _, result, spawn_pos)
        }
    }
    pub const RetractActiveWire_ADDRESS: usize = 0x1408F7A10;
    /// Retracts the active wire: destroys its physics constraint, detaches the near end, and
    /// notifies the HUD.
    pub unsafe fn RetractActiveWire(&mut self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) = ::std::mem::transmute(
                Self::RetractActiveWire_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
    pub const IsHookAimingState_ADDRESS: usize = 0x1408C8EA0;
    /// Whether the hook is in its "aiming" state: an active wire that is not reeling in and not a
    /// custom wire; with the re-reel upgrade, reeling in also counts.
    pub unsafe fn IsHookAimingState(&self) -> bool {
        unsafe {
            let f: unsafe extern "system" fn(this: *const Self) -> bool = ::std::mem::transmute(
                Self::IsHookAimingState_ADDRESS,
            );
            f(self as *const Self as _)
        }
    }
    pub const ShouldShowAttachedUI_ADDRESS: usize = 0x1408C94C0;
    /// Whether the HUD shows the attached-wire part of the grapple reticle: an active, attached
    /// wire while the hook is not in [`GHS_REELING_IN`](crate::equipment::grappling_hook::GrapplingHookState::GHS_REELING_IN),
    /// [`GHS_REELED_ATTACHED`](crate::equipment::grappling_hook::GrapplingHookState::GHS_REELED_ATTACHED),
    /// [`GHS_REELED_HANG`](crate::equipment::grappling_hook::GrapplingHookState::GHS_REELED_HANG), or
    /// [`GHS_REELED_UPSIDEDOWN`](crate::equipment::grappling_hook::GrapplingHookState::GHS_REELED_UPSIDEDOWN).
    pub unsafe fn ShouldShowAttachedUI(&self) -> bool {
        unsafe {
            let f: unsafe extern "system" fn(this: *const Self) -> bool = ::std::mem::transmute(
                Self::ShouldShowAttachedUI_ADDRESS,
            );
            f(self as *const Self as _)
        }
    }
    pub const GetActiveWireAttachmentGraphicsWorldPosition_ADDRESS: usize = 0x1408D7B30;
    /// Writes the interpolated graphics world position of the active wire's attached far end for
    /// the render interpolation factor `dtf`. Returns false without an attached active wire.
    pub unsafe fn GetActiveWireAttachmentGraphicsWorldPosition(
        &self,
        dtf: f32,
        gfx_pos_out: *mut crate::types::math::Vector3,
    ) -> bool {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                dtf: f32,
                gfx_pos_out: *mut crate::types::math::Vector3,
            ) -> bool = ::std::mem::transmute(
                Self::GetActiveWireAttachmentGraphicsWorldPosition_ADDRESS,
            );
            f(self as *const Self as _, dtf, gfx_pos_out)
        }
    }
    pub const PushWireDataToUI_ADDRESS: usize = 0x140902DC0;
    /// Pushes the wires' tension and attach-type data to the HUD (`CHUDUI::UpdateWireData`).
    pub unsafe fn PushWireDataToUI(&self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *const Self) = ::std::mem::transmute(
                Self::PushWireDataToUI_ADDRESS,
            );
            f(self as *const Self as _)
        }
    }
}
impl std::convert::AsRef<GrapplingHook> for GrapplingHook {
    fn as_ref(&self) -> &GrapplingHook {
        self
    }
}
impl std::convert::AsMut<GrapplingHook> for GrapplingHook {
    fn as_mut(&mut self) -> &mut GrapplingHook {
        self
    }
}
#[repr(i32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// The grappling hook's high-level state machine (`CGrapplingHook::m_State`). `SetState` is the
/// mutator; the reel tasks (`NStateTask_ReelIn`) and the reeled-in controller
/// (`NReeledInController`) drive the transitions. A reel-to-target starts in
/// [`GHS_REELING_IN`](crate::equipment::grappling_hook::GrapplingHookState::GHS_REELING_IN) -- during which
/// `NReeledInController::RotateToGrappleTarget` rotates the character's root toward the target as
/// the reel pulls them in -- and completes into one of the `GHS_REELED_*` attachment states
/// depending on the landing surface and pose. `GHS_CUSTOM_ACTIVE_WIRE` is a wire held open outside
/// the reel flow (tethers).
pub enum GrapplingHookState {
    GHS_INITIALIZING = 0isize as _,
    GHS_INACTIVE = 1isize as _,
    GHS_REELING_IN = 2isize as _,
    GHS_REELED_ATTACHED = 3isize as _,
    GHS_REELED_HANG = 4isize as _,
    GHS_REELED_UPSIDEDOWN = 5isize as _,
    GHS_REELED_STUNT = 6isize as _,
    GHS_CUSTOM_ACTIVE_WIRE = 7isize as _,
}
fn _GrapplingHookState_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], GrapplingHookState>([0u8; 0x4]);
    }
    unreachable!()
}
#[repr(C, align(8))]
/// A single grapple wire (`CGrapplingHookWire`, a `CGameObject`): two `CWireEnd`s (near/device and
/// far/hook), the physics constraint, and the wire rendering. Only referenced through
/// [`GrapplingHook::m_ActiveWire`](crate::equipment::grappling_hook::GrapplingHook::m_ActiveWire) here; the layout is unmapped.
pub struct GrapplingHookWire {}
impl GrapplingHookWire {}
impl std::convert::AsRef<GrapplingHookWire> for GrapplingHookWire {
    fn as_ref(&self) -> &GrapplingHookWire {
        self
    }
}
impl std::convert::AsMut<GrapplingHookWire> for GrapplingHookWire {
    fn as_mut(&mut self) -> &mut GrapplingHookWire {
        self
    }
}
#[repr(C, align(8))]
/// The in-air grapple fire motion state task (`NStateTask_InAirGrappleFireMotionTask`). Like the
/// other `NStateTask` functions, it runs against the character's state context.
pub struct NStateTask_InAirGrappleFireMotionTask {}
impl NStateTask_InAirGrappleFireMotionTask {
    pub const OnEnter_ADDRESS: usize = 0x140810B60;
    /// Enters the task, orienting the fire motion from the camera matrix
    /// (`CGameCameraManager::GetCameraMatrix`, read directly rather than through the aim slot) and
    /// the latched target ([`GrapplingHook::GetLastGrappleTargetPos`](crate::equipment::grappling_hook::GrapplingHook::GetLastGrappleTargetPos)).
    pub unsafe fn OnEnter(
        ctx: *mut crate::state::StateContext,
        p1: *mut ::std::ffi::c_void,
        p2: *mut ::std::ffi::c_void,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                ctx: *mut crate::state::StateContext,
                p1: *mut ::std::ffi::c_void,
                p2: *mut ::std::ffi::c_void,
            ) = ::std::mem::transmute(Self::OnEnter_ADDRESS);
            f(ctx, p1, p2)
        }
    }
}
impl std::convert::AsRef<NStateTask_InAirGrappleFireMotionTask>
for NStateTask_InAirGrappleFireMotionTask {
    fn as_ref(&self) -> &NStateTask_InAirGrappleFireMotionTask {
        self
    }
}
impl std::convert::AsMut<NStateTask_InAirGrappleFireMotionTask>
for NStateTask_InAirGrappleFireMotionTask {
    fn as_mut(&mut self) -> &mut NStateTask_InAirGrappleFireMotionTask {
        self
    }
}

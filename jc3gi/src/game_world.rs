#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// The game world interface (`IGameWorld`), reached through the `Base::CSingle<IGameWorld>`
/// instance pointer. The concrete object is `CGameWorld`, which owns the object manager, the
/// world file, and the player-teleport state machine; the interface subobject is what every
/// caller (bookmarks, teleport triggers, mission starts) goes through. Only the vtable is mapped.
pub struct GameWorld {
    vftable: *const crate::game_world::GameWorldVftable,
}
fn _GameWorld_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x8], GameWorld>([0u8; 0x8]);
    }
    unreachable!()
}
impl GameWorld {
    pub unsafe fn get() -> Option<&'static mut Self> {
        unsafe {
            let ptr: *mut Self = *(5418089680usize as *mut *mut Self);
            ptr.as_mut()
        }
    }
}
impl GameWorld {
    pub fn vftable(&self) -> *const crate::game_world::GameWorldVftable {
        self.vftable as *const crate::game_world::GameWorldVftable
    }
    /// Queues a deferred player teleport that `CGameWorld::UpdateTeleport` services over the
    /// following frames through its `TELEPORT_*` state machine. `world` is the destination
    /// transform (the translation row places the character; the rotation sets the facing and
    /// resets the camera behind it). `state` is an optional character state to enter on
    /// arrival (null keeps the neutral state). With `use_load_state` the game enters the load
    /// game state so the destination streams in first, with a load screen if
    /// `use_load_screen`; without it the world must already be resident (the engine logs a
    /// warning past 500 m from the active camera). `use_fade` fades the overlay UI to black for
    /// `fade_out_time` seconds and back in over `fade_in_time`. `flags` is a bit set of
    /// [`TELEPORT_FLAG_KEEP_MISSION`](crate::game_world::GameWorld::TELEPORT_FLAG_KEEP_MISSION) and
    /// [`TELEPORT_FLAG_NO_FADE`](crate::game_world::GameWorld::TELEPORT_FLAG_NO_FADE). `teleport_done_event`, when
    /// non-null, is sent once the arrival completes. Asserts when a teleport is already in
    /// flight ([`IsTeleporting`](crate::game_world::GameWorld::IsTeleporting)). Main thread only.
    pub unsafe fn TeleportPlayer(
        &mut self,
        world: *const crate::types::math::Matrix4,
        state: *const crate::hash::HashString,
        use_load_state: bool,
        use_fade: bool,
        fade_in_time: f32,
        fade_out_time: f32,
        flags: u32,
        teleport_done_event: *mut u8,
        use_load_screen: bool,
    ) {
        unsafe {
            let f = (&raw const (*self.vftable()).TeleportPlayer).read();
            f(
                self as *mut Self as _,
                world,
                state,
                use_load_state,
                use_fade,
                fade_in_time,
                fade_out_time,
                flags,
                teleport_done_event,
                use_load_screen,
            )
        }
    }
    /// Teleports the local player's character synchronously: initializes the character's
    /// transform to `world`, forces the neutral animation state, optionally enters `state`,
    /// resets the game camera behind the new transform, and re-enables the player and
    /// character. Fires `game_teleporting_initiated` and `game_teleporting_completed`, resets
    /// the current settlement, province, and region, and notifies the HUD. Aborts the active
    /// mission unless `flags` has
    /// [`TELEPORT_FLAG_KEEP_MISSION`](crate::game_world::GameWorld::TELEPORT_FLAG_KEEP_MISSION). No streaming: the
    /// destination must already be resident. Asserts when a deferred teleport is in flight.
    /// Main thread only.
    pub unsafe fn TeleportPlayerInstant(
        &mut self,
        world: *const crate::types::math::Matrix4,
        state: *const crate::hash::HashString,
        flags: u32,
    ) {
        unsafe {
            let f = (&raw const (*self.vftable()).TeleportPlayerInstant).read();
            f(self as *mut Self as _, world, state, flags)
        }
    }
    /// Whether a deferred [`TeleportPlayer`](crate::game_world::GameWorld::TeleportPlayer) is still being serviced.
    pub unsafe fn IsTeleporting(&mut self) -> bool {
        unsafe {
            let f = (&raw const (*self.vftable()).IsTeleporting).read();
            f(self as *mut Self as _)
        }
    }
}
impl GameWorld {
    /// Bit `0x1` of the teleport `flags`: keep the active mission running across the teleport.
    /// Without it the mission manager aborts the active mission as part of the teleport.
    pub const TELEPORT_FLAG_KEEP_MISSION: u32 = 1;
    /// Bit `0x2` of the teleport `flags` (deferred teleports only): skip the fade-to-black even
    /// when `use_fade` is set.
    pub const TELEPORT_FLAG_NO_FADE: u32 = 2;
}
impl std::convert::AsRef<GameWorld> for GameWorld {
    fn as_ref(&self) -> &GameWorld {
        self
    }
}
impl std::convert::AsMut<GameWorld> for GameWorld {
    fn as_mut(&mut self) -> &mut GameWorld {
        self
    }
}
#[repr(C, align(8))]
pub struct GameWorldVftable {
    _vfunc_0: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_1: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_2: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_3: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_4: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_5: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_6: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_7: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_8: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_9: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_10: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_11: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_12: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_13: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_14: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_15: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_16: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_17: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    _vfunc_18: unsafe extern "system" fn(this: *mut crate::game_world::GameWorld),
    /// Queues a deferred player teleport that `CGameWorld::UpdateTeleport` services over the
    /// following frames through its `TELEPORT_*` state machine. `world` is the destination
    /// transform (the translation row places the character; the rotation sets the facing and
    /// resets the camera behind it). `state` is an optional character state to enter on
    /// arrival (null keeps the neutral state). With `use_load_state` the game enters the load
    /// game state so the destination streams in first, with a load screen if
    /// `use_load_screen`; without it the world must already be resident (the engine logs a
    /// warning past 500 m from the active camera). `use_fade` fades the overlay UI to black for
    /// `fade_out_time` seconds and back in over `fade_in_time`. `flags` is a bit set of
    /// [`TELEPORT_FLAG_KEEP_MISSION`](crate::game_world::GameWorld::TELEPORT_FLAG_KEEP_MISSION) and
    /// [`TELEPORT_FLAG_NO_FADE`](crate::game_world::GameWorld::TELEPORT_FLAG_NO_FADE). `teleport_done_event`, when
    /// non-null, is sent once the arrival completes. Asserts when a teleport is already in
    /// flight ([`IsTeleporting`](crate::game_world::GameWorld::IsTeleporting)). Main thread only.
    pub TeleportPlayer: unsafe extern "system" fn(
        this: *mut crate::game_world::GameWorld,
        world: *const crate::types::math::Matrix4,
        state: *const crate::hash::HashString,
        use_load_state: bool,
        use_fade: bool,
        fade_in_time: f32,
        fade_out_time: f32,
        flags: u32,
        teleport_done_event: *mut u8,
        use_load_screen: bool,
    ),
    /// Teleports the local player's character synchronously: initializes the character's
    /// transform to `world`, forces the neutral animation state, optionally enters `state`,
    /// resets the game camera behind the new transform, and re-enables the player and
    /// character. Fires `game_teleporting_initiated` and `game_teleporting_completed`, resets
    /// the current settlement, province, and region, and notifies the HUD. Aborts the active
    /// mission unless `flags` has
    /// [`TELEPORT_FLAG_KEEP_MISSION`](crate::game_world::GameWorld::TELEPORT_FLAG_KEEP_MISSION). No streaming: the
    /// destination must already be resident. Asserts when a deferred teleport is in flight.
    /// Main thread only.
    pub TeleportPlayerInstant: unsafe extern "system" fn(
        this: *mut crate::game_world::GameWorld,
        world: *const crate::types::math::Matrix4,
        state: *const crate::hash::HashString,
        flags: u32,
    ),
    /// Whether a deferred [`TeleportPlayer`](crate::game_world::GameWorld::TeleportPlayer) is still being serviced.
    pub IsTeleporting: unsafe extern "system" fn(
        this: *mut crate::game_world::GameWorld,
    ) -> bool,
}
fn _GameWorldVftable_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0xB0], GameWorldVftable>([0u8; 0xB0]);
    }
    unreachable!()
}
impl GameWorldVftable {}
impl std::convert::AsRef<GameWorldVftable> for GameWorldVftable {
    fn as_ref(&self) -> &GameWorldVftable {
        self
    }
}
impl std::convert::AsMut<GameWorldVftable> for GameWorldVftable {
    fn as_mut(&mut self) -> &mut GameWorldVftable {
        self
    }
}

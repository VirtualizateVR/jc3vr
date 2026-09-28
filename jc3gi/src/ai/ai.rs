#![cfg_attr(any(), rustfmt::skip)]
pub const AIMechFireGravityGun_ADDRESS: usize = 0x1406231E0;
/// A behavior-tree action callback (`AIMechFireGravityGun`), dispatched from
/// `BtCallbackHandler`: fires the gravity gun (the magnet weapon component) of the mech the AI
/// entity drives. It picks nearby physics objects and bodies to attract (restricting the magnet
/// component to them) using perception, faction relations, and the mech's turret yaw. `tree` is the
/// `STreeInstance`, `registry` the `SAiEntityRegistry`, and `target` the target position; the
/// return value is the behavior-tree node result.
pub unsafe fn AIMechFireGravityGun(
    p0: u32,
    p1: *mut ::std::ffi::c_void,
    tree: *mut ::std::ffi::c_void,
    registry: *mut ::std::ffi::c_void,
    target: *const crate::types::math::Vector3,
    p5: f32,
    p6: f32,
) -> u32 {
    unsafe {
        let f: unsafe extern "system" fn(
            p0: u32,
            p1: *mut ::std::ffi::c_void,
            tree: *mut ::std::ffi::c_void,
            registry: *mut ::std::ffi::c_void,
            target: *const crate::types::math::Vector3,
            p5: f32,
            p6: f32,
        ) -> u32 = ::std::mem::transmute(AIMechFireGravityGun_ADDRESS);
        f(p0, p1, tree, registry, target, p5, p6)
    }
}
pub const AiGetClosestEntityInRange_ADDRESS: usize = 0x1405F97F0;
/// `_AiGetClosestEntityInRange`: finds the AI entity of `entity_type` (1-based category) closest
/// to `point` within `range`, accepted by `faction` (and, with `alive_only`, alive), skipping
/// `ignore_entity` and the entity owned by `owner_go`. Writes the found entity handle to
/// `found_entity` (0 when none) and returns whether one was found. IDA labels this
/// address `CAIPerception::GetCharacterWithMentalState`; it is `_AiGetClosestEntityInRange`.
pub unsafe fn AiGetClosestEntityInRange(
    point: *const crate::types::math::Vector3,
    entity_type: u32,
    faction: i32,
    alive_only: u32,
    range: f32,
    ignore_entity: u32,
    found_entity: *mut u32,
    registry: *mut ::std::ffi::c_void,
    owner_go: *mut ::std::ffi::c_void,
) -> bool {
    unsafe {
        let f: unsafe extern "system" fn(
            point: *const crate::types::math::Vector3,
            entity_type: u32,
            faction: i32,
            alive_only: u32,
            range: f32,
            ignore_entity: u32,
            found_entity: *mut u32,
            registry: *mut ::std::ffi::c_void,
            owner_go: *mut ::std::ffi::c_void,
        ) -> bool = ::std::mem::transmute(AiGetClosestEntityInRange_ADDRESS);
        f(
            point,
            entity_type,
            faction,
            alive_only,
            range,
            ignore_entity,
            found_entity,
            registry,
            owner_go,
        )
    }
}

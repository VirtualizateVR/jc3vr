#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// A scored aim candidate (`CTarget`) registered with the target system: a weak reference to its
/// game object, its local- and world-space aim positions, per-[`TargetType`](crate::aim::target::TargetType) fitness and validity,
/// and a visibility raycast. Characters, vehicles, vehicle parts, rigid objects, and destruction
/// objects each own one. The layout is unmapped.
pub struct Target {}
impl Target {}
impl std::convert::AsRef<Target> for Target {
    fn as_ref(&self) -> &Target {
        self
    }
}
impl std::convert::AsMut<Target> for Target {
    fn as_mut(&mut self) -> &mut Target {
        self
    }
}
#[repr(i32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// The aim purpose a target or aim slot serves (`CTarget::ETargetType`). The target system scores
/// every candidate once per purpose, and the player's aim control keeps one slot per purpose in each
/// of its per-purpose arrays (see [`PlayerAimControl`](crate::aim::aim::PlayerAimControl)), indexed by
/// this value.
pub enum TargetType {
    TARGET_WEAPONS = 0isize as _,
    TARGET_STICKY_AIM = 1isize as _,
    TARGET_GRAPPLE = 2isize as _,
    TARGET_MELEE = 3isize as _,
    TARGET_GRENADE = 4isize as _,
    TARGET_SNAP = 5isize as _,
    NOF_TARGET_TYPES = 6isize as _,
}
fn _TargetType_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], TargetType>([0u8; 0x4]);
    }
    unreachable!()
}

#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// The controller behind a character (`CAvatar`): the player object for the player, a behavior
/// tree avatar for NPCs. [`Character::m_Avatar`](crate::character::character::Character::m_Avatar) points
/// at it. Only the mental-state members are bound.
pub struct Avatar {
    _field_0: [u8; 108],
    /// The current mental state.
    pub m_MentalState: crate::character::avatar::MentalState,
    /// How long, in seconds, the current mental state holds before a non-forced
    /// [`SetMentalState`](crate::character::character::Character::SetMentalState) may replace it.
    pub m_MentalStateTimer: f32,
    _field_74: [u8; 4],
}
fn _Avatar_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x78], Avatar>([0u8; 0x78]);
    }
    unreachable!()
}
impl Avatar {}
impl std::convert::AsRef<Avatar> for Avatar {
    fn as_ref(&self) -> &Avatar {
        self
    }
}
impl std::convert::AsMut<Avatar> for Avatar {
    fn as_mut(&mut self) -> &mut Avatar {
        self
    }
}
#[repr(i32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
/// A character's mental (alertness) state (`CAvatar::EMentalState`). The player's is driven each
/// frame by `CPlayer::UpdateMentalState` from its weapon use, and written through
/// [`Character::SetMentalState`](crate::character::character::Character::SetMentalState).
pub enum MentalState {
    E_RELAXED = 0isize as _,
    E_CONCERNED = 1isize as _,
    E_PANIC = 2isize as _,
    E_READY_FOR_COMBAT = 3isize as _,
    E_COMBAT = 4isize as _,
    E_SEARCH = 5isize as _,
}
fn _MentalState_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], MentalState>([0u8; 0x4]);
    }
    unreachable!()
}

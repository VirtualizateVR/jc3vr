#![cfg_attr(any(), rustfmt::skip)]
#[repr(u32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// `CGamePauseUtility::EValidActionMapsToWaitFor`: which action map the awaited action is read
/// from.
pub enum ActionMapToWaitFor {
    PlayerActionMap = 0isize as _,
    UIActionMap = 1isize as _,
}
fn _ActionMapToWaitFor_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], ActionMapToWaitFor>([0u8; 0x4]);
    }
    unreachable!()
}
#[repr(C, align(8))]
/// `CGamePauseUtility` (a `Base::CSingle` singleton): the script facility that pauses the game
/// until an action is pressed, used by tutorials.
pub struct GamePauseUtility {
    /// The current state.
    pub m_state: crate::ui::game_pause_utility::PauseState,
    /// Whether the pause was entered through the slow-down ramp.
    pub m_used_slow_down: bool,
    _field_5: [u8; 3],
    pub m_action_map: crate::ui::game_pause_utility::ActionMapToWaitFor,
    /// The action id that ends the pause, or [`NO_ACTION`](crate::ui::game_pause_utility::GamePauseUtility::NO_ACTION).
    pub m_action: i32,
    /// The action id that must be re-clicked, or [`NO_ACTION`](crate::ui::game_pause_utility::GamePauseUtility::NO_ACTION).
    pub m_reclick_action: i32,
    _field_14: [u8; 4],
}
fn _GamePauseUtility_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x18], GamePauseUtility>([0u8; 0x18]);
    }
    unreachable!()
}
impl GamePauseUtility {
    pub unsafe fn get() -> Option<&'static mut Self> {
        unsafe {
            let ptr: *mut Self = *(5418205816usize as *mut *mut Self);
            ptr.as_mut()
        }
    }
}
impl GamePauseUtility {
    pub const SetState_ADDRESS: usize = 0x1409BA270;
    /// Transitions the state machine: entering [`E_STATE_SLOW_DOWN`](crate::ui::game_pause_utility::PauseState::E_STATE_SLOW_DOWN)
    /// starts a slow-motion ease-in whose callback moves to
    /// [`E_STATE_PAUSED`](crate::ui::game_pause_utility::PauseState::E_STATE_PAUSED); entering
    /// [`E_STATE_SPEED_UP`](crate::ui::game_pause_utility::PauseState::E_STATE_SPEED_UP) ramps the slow motion back out.
    pub unsafe fn SetState(&mut self, state: crate::ui::game_pause_utility::PauseState) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                state: crate::ui::game_pause_utility::PauseState,
            ) = ::std::mem::transmute(Self::SetState_ADDRESS);
            f(self as *mut Self as _, state)
        }
    }
}
impl GamePauseUtility {
    /// The [`m_action`](crate::ui::game_pause_utility::GamePauseUtility::m_action) and [`m_reclick_action`](crate::ui::game_pause_utility::GamePauseUtility::m_reclick_action) value
    /// meaning no action. `Update` resets both to it once the awaited action is clicked or
    /// re-clicked.
    pub const NO_ACTION: i32 = -1;
}
impl std::convert::AsRef<GamePauseUtility> for GamePauseUtility {
    fn as_ref(&self) -> &GamePauseUtility {
        self
    }
}
impl std::convert::AsMut<GamePauseUtility> for GamePauseUtility {
    fn as_mut(&mut self) -> &mut GamePauseUtility {
        self
    }
}
#[repr(u32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// `CGamePauseUtility::ESlowDownState`: the pause utility's state machine.
pub enum PauseState {
    E_STATE_NONE = 0isize as _,
    E_STATE_SLOW_DOWN = 1isize as _,
    E_STATE_PAUSED = 2isize as _,
    E_STATE_SPEED_UP = 3isize as _,
}
fn _PauseState_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], PauseState>([0u8; 0x4]);
    }
    unreachable!()
}

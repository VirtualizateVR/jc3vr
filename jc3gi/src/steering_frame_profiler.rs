#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// The input steering (`CSteering`) that binds the frame profiler's `"frameprofiler"` input action
/// map: the actions that navigate, sort, pause, and close the on-screen frame profiler. Created on
/// first use by [`GetFrameProfilerInputs`](crate::steering_frame_profiler::GetFrameProfilerInputs), which registers the instance.
pub struct SteeringFrameProfiler {
    pub __vftable: u64,
    _field_8: [u8; 56],
}
fn _SteeringFrameProfiler_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x40], SteeringFrameProfiler>([0u8; 0x40]);
    }
    unreachable!()
}
impl SteeringFrameProfiler {
    pub unsafe fn get() -> Option<&'static mut Self> {
        unsafe {
            let ptr: *mut Self = *(5418223464usize as *mut *mut Self);
            ptr.as_mut()
        }
    }
}
impl SteeringFrameProfiler {
    pub const Constructor_ADDRESS: usize = 0x140E98200;
    /// Constructs the steering, loads its action map (disabled), and registers the instance.
    pub unsafe fn Constructor(
        &mut self,
    ) -> *mut crate::steering_frame_profiler::SteeringFrameProfiler {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
            ) -> *mut crate::steering_frame_profiler::SteeringFrameProfiler = ::std::mem::transmute(
                Self::Constructor_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
    pub const LoadActionMap_ADDRESS: usize = 0x140E847F0;
    /// Creates (or, when it already exists, unmaps) the `"frameprofiler"` input action map for the
    /// [`Gamepad`](crate::input::input_device_manager::DeviceType::Gamepad) device and maps the frame
    /// profiler's ten actions to it, from
    /// [`PROFILER_UP`](crate::input::input_action_map::Action::PROFILER_UP) through
    /// [`PROFILER_TOGGLE_INPUT_FOCUS_B`](crate::input::input_action_map::Action::PROFILER_TOGGLE_INPUT_FOCUS_B).
    pub unsafe fn LoadActionMap(&mut self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) = ::std::mem::transmute(
                Self::LoadActionMap_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
}
impl std::convert::AsRef<SteeringFrameProfiler> for SteeringFrameProfiler {
    fn as_ref(&self) -> &SteeringFrameProfiler {
        self
    }
}
impl std::convert::AsMut<SteeringFrameProfiler> for SteeringFrameProfiler {
    fn as_mut(&mut self) -> &mut SteeringFrameProfiler {
        self
    }
}
pub const GetFrameProfilerInputs_ADDRESS: usize = 0x140E98580;
/// Reads the frame profiler's input actions into the out flags (creating the
/// [`SteeringFrameProfiler`](crate::steering_frame_profiler::SteeringFrameProfiler) on first use): up and down as held, left, right, close, change-sorting,
/// show-empty-timers, and pause as clicked, and `toggle_input_focus` set on the frame both focus
/// buttons become held together.
pub unsafe fn GetFrameProfilerInputs(
    up_pressed: *mut bool,
    down_pressed: *mut bool,
    left_pressed: *mut bool,
    right_pressed: *mut bool,
    close_pressed: *mut bool,
    change_sorting_pressed: *mut bool,
    show_empty_timers_pressed: *mut bool,
    pause_pressed: *mut bool,
    toggle_input_focus: *mut bool,
) {
    unsafe {
        let f: unsafe extern "system" fn(
            up_pressed: *mut bool,
            down_pressed: *mut bool,
            left_pressed: *mut bool,
            right_pressed: *mut bool,
            close_pressed: *mut bool,
            change_sorting_pressed: *mut bool,
            show_empty_timers_pressed: *mut bool,
            pause_pressed: *mut bool,
            toggle_input_focus: *mut bool,
        ) = ::std::mem::transmute(GetFrameProfilerInputs_ADDRESS);
        f(
            up_pressed,
            down_pressed,
            left_pressed,
            right_pressed,
            close_pressed,
            change_sorting_pressed,
            show_empty_timers_pressed,
            pause_pressed,
            toggle_input_focus,
        )
    }
}
pub const FrameProfilerOverridePlayerInputs_ADDRESS: usize = 0x140DF69D0;
/// Resets the player and debug steerings' input action maps, so input consumed by the frame
/// profiler does not also drive the player.
pub unsafe fn FrameProfilerOverridePlayerInputs() {
    unsafe {
        let f: unsafe extern "system" fn() = ::std::mem::transmute(
            FrameProfilerOverridePlayerInputs_ADDRESS,
        );
        f()
    }
}

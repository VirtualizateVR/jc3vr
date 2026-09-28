#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
pub struct Clock {
    pub __vftable: u64,
    pub m_HasBeenInitialized: bool,
    _field_9: [u8; 7],
    pub m_FPS: f32,
    pub m_SPF: f32,
    pub m_RealFPS: f32,
    pub m_RealSPF: f32,
    pub m_UpdateSpeed: f32,
    pub m_ForceToThisFPS: f32,
    pub m_ForceToThisSPF: f32,
    pub m_Stop: bool,
    pub m_ForceToFps: bool,
    _field_2e: [u8; 2],
    /// Real elapsed milliseconds, advanced by every [`Update`](crate::clock::Clock::Update) regardless of pause.
    pub m_Time: u32,
    /// Real elapsed milliseconds as of the last [`Update`](crate::clock::Clock::Update), advanced regardless of
    /// pause.
    pub m_RealTimeAtUpdate: u32,
    /// The game-side counterpart of [`m_RealTimeAtUpdate`](crate::clock::Clock::m_RealTimeAtUpdate).
    pub m_RealTimeAtUpdateGame: u32,
    /// The value of [`m_RealTimeAtUpdate`](crate::clock::Clock::m_RealTimeAtUpdate) latched when the clock
    /// enters pause; `GetRealTimeAtUpdate` returns it instead while paused.
    pub m_PauseTimeAtUpdate: u32,
    /// The value of [`m_RealTimeAtUpdateGame`](crate::clock::Clock::m_RealTimeAtUpdateGame) latched when the
    /// clock enters pause.
    pub m_PauseTimeAtUpdateGame: u32,
    /// Elapsed milliseconds that advance only while the clock is not stopped; `GetTime` returns it
    /// unless asked to ignore pause.
    pub m_PauseTime: u32,
    pub m_GameTimeMicro: u64,
    pub m_GameRunTimeMicro: u64,
    pub m_ElapsedTime: u64,
    pub m_LastCount: u64,
    pub m_CpuFrequency: u64,
}
fn _Clock_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x70], Clock>([0u8; 0x70]);
    }
    unreachable!()
}
impl Clock {
    pub unsafe fn get() -> Option<&'static mut Self> {
        unsafe {
            let ptr: *mut Self = *(5417799288usize as *mut *mut Self);
            ptr.as_mut()
        }
    }
}
impl Clock {
    pub const GetSPF_ADDRESS: usize = 0x140091C10;
    /// Returns [`PAUSED_SPF`](crate::clock::Clock::PAUSED_SPF) while stopped (unless `ignore_pause`), the forced
    /// SPF when forcing a frame rate, and `m_SPF` otherwise.
    pub unsafe fn GetSPF(&self, ignore_pause: bool) -> f32 {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                ignore_pause: bool,
            ) -> f32 = ::std::mem::transmute(Self::GetSPF_ADDRESS);
            f(self as *const Self as _, ignore_pause)
        }
    }
    pub const Pause_ADDRESS: usize = 0x140091BB0;
    /// Sets `m_Stop`; on the transition into pause, latches the real-time-at-update counters into
    /// their pause-time counterparts.
    pub unsafe fn Pause(&mut self, pause: bool) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self, pause: bool) = ::std::mem::transmute(
                Self::Pause_ADDRESS,
            );
            f(self as *mut Self as _, pause)
        }
    }
    pub const Update_ADDRESS: usize = 0x140093230;
    /// The per-frame tick. Measures the `QueryPerformanceCounter` delta and updates `m_SPF` (an
    /// exponential moving average), `m_RealSPF`, `m_FPS`, and `m_GameTimeMicro`.
    pub unsafe fn Update(&mut self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) = ::std::mem::transmute(
                Self::Update_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
    pub const IsPaused_ADDRESS: usize = 0x140091BA0;
    pub unsafe fn IsPaused(&self) -> bool {
        unsafe {
            let f: unsafe extern "system" fn(this: *const Self) -> bool = ::std::mem::transmute(
                Self::IsPaused_ADDRESS,
            );
            f(self as *const Self as _)
        }
    }
    pub const GetRealSPF_ADDRESS: usize = 0x140091CA0;
    pub unsafe fn GetRealSPF(&self, ignore_pause: bool) -> f32 {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                ignore_pause: bool,
            ) -> f32 = ::std::mem::transmute(Self::GetRealSPF_ADDRESS);
            f(self as *const Self as _, ignore_pause)
        }
    }
    pub const DisableForceToFPS_ADDRESS: usize = 0x140091D00;
    pub unsafe fn DisableForceToFPS(&mut self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) = ::std::mem::transmute(
                Self::DisableForceToFPS_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
}
impl Clock {
    /// The seconds-per-frame value [`GetSPF`](crate::clock::Clock::GetSPF) returns while the clock is stopped and
    /// `ignore_pause` is false.
    pub const PAUSED_SPF: f32 = 0.000001;
}
impl std::convert::AsRef<Clock> for Clock {
    fn as_ref(&self) -> &Clock {
        self
    }
}
impl std::convert::AsMut<Clock> for Clock {
    fn as_mut(&mut self) -> &mut Clock {
        self
    }
}

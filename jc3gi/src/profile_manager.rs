#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// `CProfileManager` (a `Base::CSingle` singleton): the player profile, including the upgrade
/// table. Only the upgrade lookup is mapped.
pub struct ProfileManager {}
impl ProfileManager {
    pub unsafe fn get() -> Option<&'static mut Self> {
        unsafe {
            let ptr: *mut Self = *(5418217896usize as *mut *mut Self);
            ptr.as_mut()
        }
    }
}
impl ProfileManager {
    pub const GetUpgradeInfo_ADDRESS: usize = 0x140C7E6C0;
    /// Binary-searches the upgrade table (sorted by id hash) for `id` and returns its
    /// [`UpgradeInfo`](crate::profile_manager::UpgradeInfo). When the id is absent it returns a static, zero-initialized default entry,
    /// never null, so the purchased and enabled tests read false.
    pub unsafe fn GetUpgradeInfo(
        &self,
        id: *const crate::hash::HashString,
    ) -> *const crate::profile_manager::UpgradeInfo {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *const Self,
                id: *const crate::hash::HashString,
            ) -> *const crate::profile_manager::UpgradeInfo = ::std::mem::transmute(
                Self::GetUpgradeInfo_ADDRESS,
            );
            f(self as *const Self as _, id)
        }
    }
}
impl std::convert::AsRef<ProfileManager> for ProfileManager {
    fn as_ref(&self) -> &ProfileManager {
        self
    }
}
impl std::convert::AsMut<ProfileManager> for ProfileManager {
    fn as_mut(&mut self) -> &mut ProfileManager {
        self
    }
}
#[repr(C, align(8))]
/// `SUpgradeInfo`: one gear-mod upgrade's definition and state, as returned by
/// `CProfileManager::GetUpgradeInfo`. Only the leading fields are mapped; the id hashes are stored
/// bare. The profile manager's sorted table stores each one after its 8-byte key, at a
/// 0x128-byte stride.
pub struct UpgradeInfo {
    pub m_Id: crate::hash::HashString,
    pub m_TaskGroupId: crate::hash::HashString,
    pub m_DlcId: u32,
    _field_c: [u8; 4],
    pub m_TaskGroup: *const ::std::ffi::c_void,
    pub m_StarThreshold: i32,
    pub m_IsResearched: bool,
    /// Whether the upgrade is purchased. Gameplay gates test it together with
    /// [`m_IsEnabled`](crate::profile_manager::UpgradeInfo::m_IsEnabled).
    pub m_IsPurchased: bool,
    /// Whether the player has the purchased upgrade switched on; set from the comm/skill UI.
    pub m_IsEnabled: bool,
    _field_1f: [u8; 257],
}
fn _UpgradeInfo_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x120], UpgradeInfo>([0u8; 0x120]);
    }
    unreachable!()
}
impl UpgradeInfo {}
impl std::convert::AsRef<UpgradeInfo> for UpgradeInfo {
    fn as_ref(&self) -> &UpgradeInfo {
        self
    }
}
impl std::convert::AsMut<UpgradeInfo> for UpgradeInfo {
    fn as_mut(&mut self) -> &mut UpgradeInfo {
        self
    }
}

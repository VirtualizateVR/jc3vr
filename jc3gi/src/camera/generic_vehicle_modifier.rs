#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// `CGenericVehicleModifier`: the camera modifier that drives the camera while in a vehicle.
/// Only the fields below are mapped.
pub struct GenericVehicleModifier {
    _field_0: [u8; 488],
    /// The camera family for the current vehicle.
    pub m_CameraState: crate::camera::generic_vehicle_modifier::VehicleCameraState,
    /// Whether the crash camera is in use (the vehicle is out of control, incapacitated,
    /// destroyed, or burning).
    pub m_IsUsingCrashCamera: bool,
    _field_1ed: [u8; 3],
}
fn _GenericVehicleModifier_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x1F0], GenericVehicleModifier>([0u8; 0x1F0]);
    }
    unreachable!()
}
impl GenericVehicleModifier {
    pub const UpdateCameraState_ADDRESS: usize = 0x140742100;
    /// Updates the in-water, in-air, and on-ground timers, picks
    /// [`m_CameraState`](crate::camera::generic_vehicle_modifier::GenericVehicleModifier::m_CameraState) from the vehicle type, and decides the crash
    /// camera.
    pub unsafe fn UpdateCameraState(&mut self, dt: f32) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self, dt: f32) = ::std::mem::transmute(
                Self::UpdateCameraState_ADDRESS,
            );
            f(self as *mut Self as _, dt)
        }
    }
}
impl std::convert::AsRef<GenericVehicleModifier> for GenericVehicleModifier {
    fn as_ref(&self) -> &GenericVehicleModifier {
        self
    }
}
impl std::convert::AsMut<GenericVehicleModifier> for GenericVehicleModifier {
    fn as_mut(&mut self) -> &mut GenericVehicleModifier {
        self
    }
}
#[repr(u32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// `CGenericVehicleModifier::EVehicleCameraState`: the vehicle camera family, chosen each update
/// from the vehicle type.
pub enum VehicleCameraState {
    eDRIVING_CAMERA = 0isize as _,
    ePLANE_CAMERA = 1isize as _,
    eHELICOPTER_CAMERA = 2isize as _,
}
fn _VehicleCameraState_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], VehicleCameraState>([0u8; 0x4]);
    }
    unreachable!()
}

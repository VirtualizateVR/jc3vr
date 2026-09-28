#![cfg_attr(any(), rustfmt::skip)]
#[repr(C, align(8))]
/// A game object's set of attached effects and sounds (`CAttachedEffectContainer`, a base of weapons,
/// the grappling hook, and other game objects), with the effect parameter sources that feed them.
pub struct AttachedEffectContainer {
    _field_0: [u8; 40],
    /// The head of the parameter-source list.
    pub m_ParameterSources: *mut ::std::ffi::c_void,
    /// The head of the playing-instance list.
    pub m_PlayingInstances: *mut crate::equipment::attached_effects::AttachedInstance,
    _field_38: [u8; 80],
    /// The container's accumulated time, advanced by [`UpdateAttachedEffects`](crate::equipment::attached_effects::AttachedEffectContainer::UpdateAttachedEffects).
    pub m_CurrentTime: f32,
    _field_8c: [u8; 4],
}
fn _AttachedEffectContainer_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x90], AttachedEffectContainer>([0u8; 0x90]);
    }
    unreachable!()
}
impl AttachedEffectContainer {
    pub const UpdateAttachedEffects_ADDRESS: usize = 0x140466F30;
    /// Advances the container by `dt`: for each playing instance, runs
    /// [`AttachedInstance::UpdateTransform`](crate::equipment::attached_effects::AttachedInstance::UpdateTransform) and the instance update, and frees instances whose
    /// effects and sounds have all finished; then re-evaluates the parameter sources and pushes
    /// changed values to their sound and effect outputs.
    pub unsafe fn UpdateAttachedEffects(&mut self, dt: f32) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self, dt: f32) = ::std::mem::transmute(
                Self::UpdateAttachedEffects_ADDRESS,
            );
            f(self as *mut Self as _, dt)
        }
    }
}
impl std::convert::AsRef<AttachedEffectContainer> for AttachedEffectContainer {
    fn as_ref(&self) -> &AttachedEffectContainer {
        self
    }
}
impl std::convert::AsMut<AttachedEffectContainer> for AttachedEffectContainer {
    fn as_mut(&mut self) -> &mut AttachedEffectContainer {
        self
    }
}
#[repr(C, align(16))]
/// One playing attached effect or sound (`NAttachedEffectContainer::SAttachedInstance`): an offset
/// and a transform function that resolves where it is attached (typically a bone or socket on the
/// owner's model, such as a weapon muzzle), the effect and sound instances, and the resolved
/// placement. The container links its playing instances through
/// [`m_NextPlaying`](crate::equipment::attached_effects::AttachedInstance::m_NextPlaying).
pub struct AttachedInstance {
    /// The instance's offset from its attachment point.
    pub m_Offset: crate::types::math::Matrix4,
    /// Resolves the attachment's world placement: called with the result buffer, the owning
    /// container, this instance, and [`m_TransformExtraData`](crate::equipment::attached_effects::AttachedInstance::m_TransformExtraData).
    pub m_TransformFunc: *mut ::std::ffi::c_void,
    _field_48: [u8; 4],
    /// How the instance's rotation follows the attachment.
    pub m_RotationMode: crate::equipment::attached_effects::RotationMode,
    /// How the instance's translation follows the attachment.
    pub m_TranslationMode: crate::equipment::attached_effects::TranslationMode,
    _field_54: [u8; 68],
    /// The next playing instance in the owning container's list, or null.
    pub m_NextPlaying: *mut crate::equipment::attached_effects::AttachedInstance,
    _field_a0: [u8; 80],
    /// The resolved placement, refreshed by [`UpdateTransform`](crate::equipment::attached_effects::AttachedInstance::UpdateTransform).
    pub m_TransformInfo: crate::equipment::attached_effects::EffectTransformInfo,
    /// The per-update velocity of the placement's translation: the position delta multiplied by
    /// 1/30, or zero for a transform that is not
    /// [`ETransformType_Transform3D`](crate::equipment::attached_effects::TransformType::ETransformType_Transform3D).
    pub m_MeasuredVelocity: crate::types::math::Vector3,
    _field_15c: [u8; 4],
    /// The opaque argument passed to [`m_TransformFunc`](crate::equipment::attached_effects::AttachedInstance::m_TransformFunc).
    pub m_TransformExtraData: u64,
    _field_168: [u8; 8],
}
fn _AttachedInstance_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x170], AttachedInstance>([0u8; 0x170]);
    }
    unreachable!()
}
impl AttachedInstance {
    pub const UpdateTransform_ADDRESS: usize = 0x140453D00;
    /// Re-resolves the instance's placement for `container`. When the rotation mode is
    /// [`ERotationMode_ATTACHED`](crate::equipment::attached_effects::RotationMode::ERotationMode_ATTACHED) or
    /// [`ERotationMode_ATTACHED_RANDOM_ROLL`](crate::equipment::attached_effects::RotationMode::ERotationMode_ATTACHED_RANDOM_ROLL), or
    /// the translation mode is
    /// [`ETranslationMode_ATTACHED`](crate::equipment::attached_effects::TranslationMode::ETranslationMode_ATTACHED), it calls [`m_TransformFunc`](crate::equipment::attached_effects::AttachedInstance::m_TransformFunc) and copies
    /// the [`EffectTransformInfo`](crate::equipment::attached_effects::EffectTransformInfo) it returns into
    /// [`m_TransformInfo`](crate::equipment::attached_effects::AttachedInstance::m_TransformInfo); it then applies the transform modes and derives
    /// [`m_MeasuredVelocity`](crate::equipment::attached_effects::AttachedInstance::m_MeasuredVelocity) from the translation delta.
    pub unsafe fn UpdateTransform(
        &mut self,
        container: *mut crate::equipment::attached_effects::AttachedEffectContainer,
    ) {
        unsafe {
            let f: unsafe extern "system" fn(
                this: *mut Self,
                container: *mut crate::equipment::attached_effects::AttachedEffectContainer,
            ) = ::std::mem::transmute(Self::UpdateTransform_ADDRESS);
            f(self as *mut Self as _, container)
        }
    }
}
impl std::convert::AsRef<AttachedInstance> for AttachedInstance {
    fn as_ref(&self) -> &AttachedInstance {
        self
    }
}
impl std::convert::AsMut<AttachedInstance> for AttachedInstance {
    fn as_mut(&mut self) -> &mut AttachedInstance {
        self
    }
}
#[repr(C, align(16))]
/// The world placement handed to a playing attached effect (`SEffectTransformInfo`): a transform,
/// the velocity of the attachment, and what the transform is used for. Resolved per instance each
/// update by the instance's transform function and copied whole into
/// [`AttachedInstance::m_TransformInfo`](crate::equipment::attached_effects::AttachedInstance::m_TransformInfo).
pub struct EffectTransformInfo {
    /// The world transform (in a union with a custom speaker mix for audio spatialization).
    pub m_Transform: crate::types::math::Matrix4,
    /// The attachment's velocity.
    pub m_Velocity: crate::types::math::Vector3,
    _field_4c: [u8; 4],
    /// What the transform drives.
    pub m_Type: crate::equipment::attached_effects::TransformType,
    _field_54: [u8; 12],
}
fn _EffectTransformInfo_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x60], EffectTransformInfo>([0u8; 0x60]);
    }
    unreachable!()
}
impl EffectTransformInfo {}
impl std::convert::AsRef<EffectTransformInfo> for EffectTransformInfo {
    fn as_ref(&self) -> &EffectTransformInfo {
        self
    }
}
impl std::convert::AsMut<EffectTransformInfo> for EffectTransformInfo {
    fn as_mut(&mut self) -> &mut EffectTransformInfo {
        self
    }
}
#[repr(i32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// How an attached instance's rotation follows its attachment
/// (`CAttachedEffectContainer::ERotationMode`).
pub enum RotationMode {
    ERotationMode_ATTACHED = 0isize as _,
    ERotationMode_FIRST_FRAME = 1isize as _,
    ERotationMode_ATTACHED_RANDOM_ROLL = 2isize as _,
    ERotationMode_FIRST_FRAME_RANDOM_ROLL = 3isize as _,
    ERotationMode_WORLD_UP_RANDOM = 4isize as _,
    ERotationMode_RANDOM = 5isize as _,
    ERotationMode_WORLD_UP = 6isize as _,
    ERotationMode_COUNT = 7isize as _,
}
fn _RotationMode_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], RotationMode>([0u8; 0x4]);
    }
    unreachable!()
}
#[repr(i32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// What an [`EffectTransformInfo`](crate::equipment::attached_effects::EffectTransformInfo) transform drives (`SEffectTransformInfo::ETransformType`).
pub enum TransformType {
    ETransformType_Transform3D = 0isize as _,
    ETransformType_AudioSpatialization = 1isize as _,
    ETransformType_ReverbSpatialization = 2isize as _,
    ETransformType_EchoSpatialization = 3isize as _,
    ETransformType_Invalid = 4isize as _,
}
fn _TransformType_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], TransformType>([0u8; 0x4]);
    }
    unreachable!()
}
#[repr(i32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// How an attached instance's translation follows its attachment
/// (`CAttachedEffectContainer::ETranslationMode`).
pub enum TranslationMode {
    ETranslationMode_ATTACHED = 0isize as _,
    ETranslationMode_FIRST_FRAME = 1isize as _,
    ETranslationMode_COUNT = 2isize as _,
}
fn _TranslationMode_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], TranslationMode>([0u8; 0x4]);
    }
    unreachable!()
}

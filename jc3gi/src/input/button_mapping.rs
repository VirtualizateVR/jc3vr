#![cfg_attr(any(), rustfmt::skip)]
#[repr(u32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// `CButtonMapping::EButtonInteraction`: how a mapping's actions must be pressed.
pub enum ButtonInteraction {
    BINT_SINGLE_BUTTON_TAP = 0isize as _,
    BINT_SINGLE_BUTTON_HOLD = 1isize as _,
    BINT_TWO_BUTTON_HOLD_AND_TAP = 2isize as _,
    BINT_SINGLE_BUTTON_RELEASE = 3isize as _,
}
fn _ButtonInteraction_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], ButtonInteraction>([0u8; 0x4]);
    }
    unreachable!()
}
#[repr(C, align(4))]
/// `CButtonMapping`: the fixed table of semantic button mappings the button-hint and
/// controls-screen UI resolve against, plus the action list for the bindings screen.
pub struct ButtonMapping {
    /// One entry per [`Mapping`](crate::input::button_mapping::Mapping), [`NUM_MAPPINGS`](crate::input::button_mapping::Mapping::NUM_MAPPINGS) in all.
    pub m_AllInfo: [crate::input::button_mapping::MappingInfo; 141],
}
fn _ButtonMapping_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0xF6C], ButtonMapping>([0u8; 0xF6C]);
    }
    unreachable!()
}
impl ButtonMapping {
    pub const Init_ADDRESS: usize = 0x140CA8640;
    /// Calls [`PopulateMappings`](crate::input::button_mapping::ButtonMapping::PopulateMappings), then rebuilds the controls-screen
    /// action list.
    pub unsafe fn Init(&mut self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) = ::std::mem::transmute(
                Self::Init_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
    pub const PopulateMappings_ADDRESS: usize = 0x140C3BBA0;
    /// Fills every [`m_AllInfo`](crate::input::button_mapping::ButtonMapping::m_AllInfo) entry from hardcoded data: the actions,
    /// interaction, `buttonhint_*` key hash, ability and upgrade gates, and flags. There is no data
    /// file behind the table.
    pub unsafe fn PopulateMappings(&mut self) {
        unsafe {
            let f: unsafe extern "system" fn(this: *mut Self) = ::std::mem::transmute(
                Self::PopulateMappings_ADDRESS,
            );
            f(self as *mut Self as _)
        }
    }
}
impl std::convert::AsRef<ButtonMapping> for ButtonMapping {
    fn as_ref(&self) -> &ButtonMapping {
        self
    }
}
impl std::convert::AsMut<ButtonMapping> for ButtonMapping {
    fn as_mut(&mut self) -> &mut ButtonMapping {
        self
    }
}
#[repr(u32)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
/// `CButtonMapping::EMapping`: a semantic button mapping, indexing
/// [`ButtonMapping::m_AllInfo`](crate::input::button_mapping::ButtonMapping::m_AllInfo). The `MAPPING_*` values are grouped into per-context sections
/// (generic, on foot, context-specific on foot, each vehicle class, UI, weaponized wingsuit, and
/// mech); each section ends with an `END_*` marker slot that stays at the constructor default, and
/// the `FIRST_*` / `LAST_*` constants give each section's inclusive bounds for the range queries.
/// [`ButtonMapping::PopulateMappings`](crate::input::button_mapping::ButtonMapping::PopulateMappings) hardcodes every entry.
pub enum Mapping {
    MAPPING_PAUSE = 0isize as _,
    MAPPING_COMMLINK = 1isize as _,
    END_GENERIC_MAPPINGS = 2isize as _,
    MAPPING_FIRE_GRAPPLING_HOOK = 3isize as _,
    MAPPING_RETRACT_TETHERS = 4isize as _,
    MAPPING_RELEASE_TETHERS_NON_CHORDED = 5isize as _,
    MAPPING_FIRE_RIGHT = 6isize as _,
    MAPPING_THROW_GRENADE = 7isize as _,
    MAPPING_DETONATE_EXPLOSIVE_GAMEPAD = 8isize as _,
    MAPPING_DETONATE_EXPLOSIVE_PC = 9isize as _,
    MAPPING_RELOAD = 10isize as _,
    MAPPING_JUMP = 11isize as _,
    MAPPING_OPEN_WINGSUIT = 12isize as _,
    MAPPING_OPEN_PARACHUTE = 13isize as _,
    MAPPING_HAMMER = 14isize as _,
    MAPPING_PRECISION_AIM = 15isize as _,
    MAPPING_CANCEL = 16isize as _,
    MAPPING_USE_ITEM = 17isize as _,
    MAPPING_SELECT_DUAL_WIELD = 18isize as _,
    MAPPING_SELECT_TWO_HANDED = 19isize as _,
    MAPPING_SELECT_TWO_HANDED_SPECIAL = 20isize as _,
    MAPPING_SELECT_EXPLOSIVE = 21isize as _,
    MAPPING_PREV_WEAPON = 22isize as _,
    MAPPING_NEXT_WEAPON = 23isize as _,
    MAPPING_MOVE_FORWARD = 24isize as _,
    MAPPING_MOVE_BACKWARD = 25isize as _,
    MAPPING_MOVE_LEFT = 26isize as _,
    MAPPING_MOVE_RIGHT = 27isize as _,
    MAPPING_WALK = 28isize as _,
    END_ON_FOOT_MAPPINGS = 29isize as _,
    MAPPING_RELEASE_TETHERS_WITH_RETRACT = 30isize as _,
    MAPPING_DETACH_MOUNTED_GUN = 31isize as _,
    MAPPING_CANCEL_REEL_IN = 32isize as _,
    MAPPING_SLINGSHOT_TO_REEL_IN = 33isize as _,
    MAPPING_PLANT_EXPLOSIVE = 34isize as _,
    MAPPING_HANG_JUMP = 35isize as _,
    MAPPING_SNIPER_ZOOM = 36isize as _,
    MAPPING_SKIP_CUTSCENE = 37isize as _,
    MAPPING_LOOK_AT = 38isize as _,
    MAPPING_AIR_BRAKE = 39isize as _,
    MAPPING_SLINGSUIT_TO_REEL_IN = 40isize as _,
    MAPPING_HOLSTER_WEAPON = 41isize as _,
    MAPPING_REEL_BOOST = 42isize as _,
    MAPPING_KEEP_RAGDOLLING = 43isize as _,
    END_CONTEXT_SPECIFIC_ON_FOOT_MAPPINGS = 44isize as _,
    MAPPING_LOOK_BACK_CAM = 45isize as _,
    MAPPING_RECENTER_CAM = 46isize as _,
    MAPPING_EXIT_VEHICLE = 47isize as _,
    MAPPING_ATTACHED_TO_PARACHUTE = 48isize as _,
    MAPPING_TOGGLE_MAGNET = 49isize as _,
    MAPPING_VEHICLE_RELEASE_GRAPPLE = 50isize as _,
    END_GENERIC_VEHICLE_MAPPINGS = 51isize as _,
    MAPPING_HANDBRAKE = 52isize as _,
    MAPPING_ACCELERATE_LAND_VEHICLE = 53isize as _,
    MAPPING_REVERSE_LAND_VEHICLE = 54isize as _,
    MAPPING_NITROUS_LAND_VEHICLE = 55isize as _,
    MAPPING_TURBO_JUMP_LAND_VEHICLE = 56isize as _,
    MAPPING_SOUND_HORN_SIREN_LAND_VEHICLE = 57isize as _,
    END_GENERIC_LAND_VEHICLE_MAPPINGS = 58isize as _,
    MAPPING_TURN_LEFT = 59isize as _,
    MAPPING_TURN_RIGHT = 60isize as _,
    MAPPING_ENTER_GUNNER_SEAT = 61isize as _,
    MAPPING_FIRE_VEHICLE_WEAPON_PRIMARY_CAR = 62isize as _,
    MAPPING_FIRE_VEHICLE_WEAPON_SECONDARY_CAR = 63isize as _,
    MAPPING_STUNT_JUMP_CAR = 64isize as _,
    END_CAR_MAPPINGS = 65isize as _,
    MAPPING_FIRE_WEAPON_MOTORCYCLE = 66isize as _,
    MAPPING_RELOAD_WEAPON_MOTORCYCLE = 67isize as _,
    MAPPING_HOLSTER_WEAPON_MOTORCYCLE = 68isize as _,
    MAPPING_TILT_FORWARD_MOTORCYCLE = 69isize as _,
    MAPPING_TILT_BACKWARD_MOTORCYCLE = 70isize as _,
    MAPPING_LEAN_LEFT_MOTORCYCLE = 71isize as _,
    MAPPING_LEAN_RIGHT_MOTORCYCLE = 72isize as _,
    END_MOTORCYCLE_MAPPINGS = 73isize as _,
    MAPPING_NITROUS_SEA_VEHICLE = 74isize as _,
    MAPPING_TURBO_JUMP_SEA_VEHICLE = 75isize as _,
    MAPPING_TURN_LEFT_SEA_VEHICLE = 76isize as _,
    MAPPING_TURN_RIGHT_SEA_VEHICLE = 77isize as _,
    MAPPING_SOUND_HORN_SIREN_SEA_VEHICLE = 78isize as _,
    END_GENERIC_SEA_VEHICLE_MAPPINGS = 79isize as _,
    MAPPING_FIRE_VEHICLE_WEAPON_PRIMARY_BOAT = 80isize as _,
    MAPPING_STUNT_JUMP_BOAT = 81isize as _,
    END_BOAT_MAPPINGS = 82isize as _,
    MAPPING_FIRE_WEAPON_JETSKI = 83isize as _,
    MAPPING_RELOAD_WEAPON_JETSKI = 84isize as _,
    MAPPING_HOLSTER_WEAPON_JETSKI = 85isize as _,
    END_JETSKI_MAPPINGS = 86isize as _,
    MAPPING_FIRE_VEHICLE_WEAPON_PRIMARY_AIR_VEHICLE = 87isize as _,
    MAPPING_FIRE_VEHICLE_WEAPON_SECONDARY_AIR_VEHICLE = 88isize as _,
    END_GENERIC_AIR_VEHICLE_MAPPINGS = 89isize as _,
    MAPPING_HELI_INC_ALTITUDE = 90isize as _,
    MAPPING_HELI_DEC_ALTITUDE = 91isize as _,
    MAPPING_HELI_ROLL_RIGHT = 92isize as _,
    MAPPING_HELI_ROLL_LEFT = 93isize as _,
    MAPPING_HELI_TURN_LEFT = 94isize as _,
    MAPPING_HELI_TURN_RIGHT = 95isize as _,
    MAPPING_HELI_FORWARD = 96isize as _,
    MAPPING_HELI_BACKWARD = 97isize as _,
    MAPPING_STUNT_JUMP_HELICOPTER = 98isize as _,
    MAPPING_NITROUS_HELICOPTER = 99isize as _,
    END_HELICOPTER_MAPPINGS = 100isize as _,
    MAPPING_PLANE_TURN_RIGHT = 101isize as _,
    MAPPING_PLANE_TURN_LEFT = 102isize as _,
    MAPPING_PLANE_ROLL_RIGHT = 103isize as _,
    MAPPING_PLANE_ROLL_LEFT = 104isize as _,
    MAPPING_PLANE_PITCH_UP = 105isize as _,
    MAPPING_PLANE_PITCH_DOWN = 106isize as _,
    MAPPING_PLANE_INC_THRUST = 107isize as _,
    MAPPING_PLANE_DEC_THRUST = 108isize as _,
    MAPPING_STUNT_JUMP_PLANE = 109isize as _,
    MAPPING_NITROUS_PLANE = 110isize as _,
    END_PLANE_MAPPINGS = 111isize as _,
    MAPPING_UI_OK = 112isize as _,
    MAPPING_UI_CANCEL = 113isize as _,
    MAPPING_UI_TAB_NEXT = 114isize as _,
    MAPPING_UI_TAB_PREV = 115isize as _,
    MAPPING_UI_FILTER = 116isize as _,
    MAPPING_UI_RECENTER = 117isize as _,
    MAPPING_UI_PAGE_NEXT = 118isize as _,
    MAPPING_UI_PAGE_PREV = 119isize as _,
    END_UI_MAPPINGS = 120isize as _,
    MAPPING_WEAPONIZED_WINGSUIT_TAKEOFF = 121isize as _,
    MAPPING_WEAPONIZED_WINGSUIT_BOOST = 122isize as _,
    MAPPING_WEAPONIZED_WINGSUIT_BRAKE = 123isize as _,
    MAPPING_WEAPONIZED_WINGSUIT_EVADE = 124isize as _,
    MAPPING_WEAPONIZED_WINGSUIT_PRIMARY_WEAPON = 125isize as _,
    MAPPING_WEAPONIZED_WINGSUIT_SECONDARY_WEAPON = 126isize as _,
    END_WEAPONIZED_WINGSUIT_MAPPINGS = 127isize as _,
    MAPPING_MECH_MOVE_FORWARD = 128isize as _,
    MAPPING_MECH_MOVE_BACKWARD = 129isize as _,
    MAPPING_MECH_MOVE_LEFT = 130isize as _,
    MAPPING_MECH_MOVE_RIGHT = 131isize as _,
    MAPPING_MECH_JUMP = 132isize as _,
    MAPPING_MECH_PUNCH = 133isize as _,
    MAPPING_MECH_GRAVITY_GUN_ATTRACT = 134isize as _,
    MAPPING_MECH_GRAVITY_GUN_DROP = 135isize as _,
    MAPPING_MECH_GRAVITY_GUN_THROW = 136isize as _,
    MAPPING_MECH_FIRE_RIGHT_HAND_WEAPON = 137isize as _,
    END_MECH_MAPPINGS = 138isize as _,
    MAPPING_ACTIVATE_BAVARIUM_SHIELD = 139isize as _,
    MAPPING_CHALLENGE_RESET_VEHICLE = 140isize as _,
    NUM_MAPPINGS = 141isize as _,
}
fn _Mapping_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x4], Mapping>([0u8; 0x4]);
    }
    unreachable!()
}
impl Mapping {
    pub const FIRST_BOAT_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_FIRE_VEHICLE_WEAPON_PRIMARY_BOAT;
    pub const FIRST_CAR_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_TURN_LEFT;
    pub const FIRST_CONTEXT_SPECIFIC_ON_FOOT_MAPPING: crate::input::button_mapping::Mapping = Mapping::MAPPING_RELEASE_TETHERS_WITH_RETRACT;
    pub const FIRST_GENERIC_AIR_VEHICLE_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_FIRE_VEHICLE_WEAPON_PRIMARY_AIR_VEHICLE;
    pub const FIRST_GENERIC_LAND_VEHICLE_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_HANDBRAKE;
    pub const FIRST_GENERIC_MAPPING: crate::input::button_mapping::Mapping = Mapping::MAPPING_PAUSE;
    pub const FIRST_GENERIC_SEA_VEHICLE_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_NITROUS_SEA_VEHICLE;
    pub const FIRST_GENERIC_VEHICLE_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_LOOK_BACK_CAM;
    pub const FIRST_HELICOPTER_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_HELI_INC_ALTITUDE;
    pub const FIRST_JETSKI_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_FIRE_WEAPON_JETSKI;
    pub const FIRST_MECH_MAPPING: crate::input::button_mapping::Mapping = Mapping::MAPPING_MECH_MOVE_FORWARD;
    pub const FIRST_MOTORCYCLE_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_FIRE_WEAPON_MOTORCYCLE;
    pub const FIRST_ON_FOOT_MAPPING: crate::input::button_mapping::Mapping = Mapping::MAPPING_FIRE_GRAPPLING_HOOK;
    pub const FIRST_PLANE_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_PLANE_TURN_RIGHT;
    pub const FIRST_UI_MAPPING: crate::input::button_mapping::Mapping = Mapping::MAPPING_UI_OK;
    pub const FIRST_WEAPONIZED_WINGSUIT_MAPPING: crate::input::button_mapping::Mapping = Mapping::MAPPING_WEAPONIZED_WINGSUIT_TAKEOFF;
    pub const LAST_BOAT_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_STUNT_JUMP_BOAT;
    pub const LAST_CAR_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_STUNT_JUMP_CAR;
    pub const LAST_CONTEXT_SPECIFIC_ON_FOOT_MAPPING: crate::input::button_mapping::Mapping = Mapping::MAPPING_KEEP_RAGDOLLING;
    pub const LAST_GENERIC_AIR_VEHICLE_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_FIRE_VEHICLE_WEAPON_SECONDARY_AIR_VEHICLE;
    pub const LAST_GENERIC_LAND_VEHICLE_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_SOUND_HORN_SIREN_LAND_VEHICLE;
    pub const LAST_GENERIC_MAPPING: crate::input::button_mapping::Mapping = Mapping::MAPPING_COMMLINK;
    pub const LAST_GENERIC_SEA_VEHICLE_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_SOUND_HORN_SIREN_SEA_VEHICLE;
    pub const LAST_GENERIC_VEHICLE_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_VEHICLE_RELEASE_GRAPPLE;
    pub const LAST_HELICOPTER_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_NITROUS_HELICOPTER;
    pub const LAST_JETSKI_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_HOLSTER_WEAPON_JETSKI;
    pub const LAST_MECH_MAPPING: crate::input::button_mapping::Mapping = Mapping::MAPPING_MECH_FIRE_RIGHT_HAND_WEAPON;
    pub const LAST_MOTORCYCLE_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_LEAN_RIGHT_MOTORCYCLE;
    pub const LAST_ON_FOOT_MAPPING: crate::input::button_mapping::Mapping = Mapping::MAPPING_WALK;
    pub const LAST_PLANE_MAPPINGS: crate::input::button_mapping::Mapping = Mapping::MAPPING_NITROUS_PLANE;
    pub const LAST_UI_MAPPING: crate::input::button_mapping::Mapping = Mapping::MAPPING_UI_PAGE_PREV;
    pub const LAST_WEAPONIZED_WINGSUIT_MAPPING: crate::input::button_mapping::Mapping = Mapping::MAPPING_WEAPONIZED_WINGSUIT_SECONDARY_WEAPON;
}
#[derive(Copy, Clone)]
#[repr(C, align(4))]
/// `CButtonMapping::SMappingInfo`: one semantic mapping's actions, hint text, gates, and flags.
/// The layout stores the two gate hashes as bare 32-bit hashes.
pub struct MappingInfo {
    pub m_Interaction: crate::input::button_mapping::ButtonInteraction,
    /// The primary action (the held one for a chord). The constructor default is
    /// [`Action::COUNT`](crate::input::input_action_map::Action::COUNT), an invalid action, which the `END_*` marker slots keep.
    pub m_Action1: crate::input::input_action_map::Action,
    /// The tapped action of a chord, or [`Action::COUNT`](crate::input::input_action_map::Action::COUNT) when unused.
    pub m_Action2: crate::input::input_action_map::Action,
    /// The hash of the `buttonhint_*` localization key.
    pub m_DescriptionKey: u32,
    /// The upgrade that must be purchased and enabled for the mapping to be active, or zero.
    pub m_RequiredUpgrade: crate::hash::HashString,
    /// The ability that must be enabled on the player for the mapping to be active, or zero.
    pub m_RequiredAbility: crate::hash::HashString,
    /// Whether the player can rebind the mapping.
    pub m_Customizable: bool,
    /// Whether the mapping is listed on the controls screen.
    pub m_ShowInUIMapping: bool,
    /// Whether the mapping applies only to keyboard and mouse.
    pub m_PCOnly: bool,
    /// Whether the mapping applies only to a gamepad.
    pub m_ControllerOnly: bool,
}
fn _MappingInfo_size_check() {
    unsafe {
        ::std::mem::transmute::<[u8; 0x1C], MappingInfo>([0u8; 0x1C]);
    }
    unreachable!()
}
impl MappingInfo {}
impl std::convert::AsRef<MappingInfo> for MappingInfo {
    fn as_ref(&self) -> &MappingInfo {
        self
    }
}
impl std::convert::AsMut<MappingInfo> for MappingInfo {
    fn as_mut(&mut self) -> &mut MappingInfo {
        self
    }
}

# Motion controllers and roomscale: plan of record

The plan for making Just Cause 3 playable with motion controllers throughout: controller input in every mode, hands owned by the player, per-hand aiming (the right hand aims the guns, the left hand aims the grapple, and dual-wielded pistols aim independently), physical weapon handling, and roomscale locomotion. The plan degrades to gaze aiming with a gamepad when controllers are not in use, and to a seated posture when the player is not standing. Issue #28 (independent motion-controller aiming) is one part of it.

The engine recon behind the plan is in [aim-state.md](../../engine/gameplay/aim-state.md) (the aim state, its consumers, and fire gating), [aim-pipeline.md](../../engine/gameplay/aim-pipeline.md), [grapple-pipeline.md](../../engine/gameplay/grapple-pipeline.md), [input.md](../../engine/gameplay/input.md), [hands-locomotion-and-state.md](../../engine/character/hands-locomotion-and-state.md), and [humanik.md](../../engine/character/humanik.md). The head and body model it extends is [head-and-body.md](../body/head-and-body.md).

## Principles

The virtual body behaves continuously and consistently with the player's body. The game switches the player's body between discrete behaviours: the aim state flips locomotion to strafing, raises the weapon, and runs the arm aim IK, and the combat mental state draws and holsters the weapon on its own. The mod dissolves these switches rather than mapping controller input onto them. A state change that the player did not perform with their own body is a defect in this model.

The mod never changes the player's view orientation unless the change is attached to a movement action the player took, such as a stick turn. Run-and-steer locomotion, where the body turns to face its direction of travel, is excluded for this reason, and is not offered as an option.

The hands belong to the player. An animation takes the hands only where a scripted state cannot yet work otherwise, and the hand tiers below record each such state and the work that would return the hands to the player.

The plan degrades in two directions. Without controllers, the mod keeps its current behaviour: gaze aiming and gamepad input. Without room to walk, the player's posture is standing or seated, and roomscale translation is disabled.

The mod's in-VR UI is an adapted egui panel, which is not a VR-native interface. Features whose main cost is new mod-specific UI are deferred until that interface is redesigned. The deferred list is at the end of the plan. The world-space overlay layer (described below) is scene rendering, not UI, and is in scope.

## Decisions

The following decisions are settled and the rest of the plan assumes them.

- Handedness is configurable. A `dominant_hand` setting mirrors the gun and grapple hands, the bindings, and the weapon attach bones. The grapple gauntlet stays on Rico's left arm model in the mirrored layout, so the grapple fires from the gauntlet on the non-dominant arm cosmetically.
- The aim state is pinned clear for the local player (see "The aim-state pin").
- The combat mental state keeps running for the AI, and its effects on the player's body are removed: no automatic draw and no automatic holster.
- Guns are sticky. A drawn gun stays in the hand until the player holsters it. Grip inputs differ between controllers (an analog grip on Touch, a force-sensing grip on Index, a binary grip button on older wands), and holding a grip for minutes is tiring on all of them, so the grip is used only as an edge: press to grab, press to grapple.
- The left grip fires the grapple, except inside a grab volume (a foregrip, a holster, a wheel), where it grabs. The grapple keeps the game's native tap and hold semantics, and a yank gesture on the left hand is an alternate zip trigger.
- Two-handed weapons are held by the dominant hand alone. The off-hand can grab the foregrip, and the line between the two hands then sets the weapon's direction.
- Dual-wielded pistols split onto both hands, each fired by its own trigger and aimed by its own hand.
- Guns show no reticle by default. A dot at the hit point and a laser line are options. The grapple shows a hook-point highlight and a gauntlet beam, each configurable.
- Aim assist is re-derived per hand ray, softened, and defaults to off.
- Driver-seat vehicle weapons aim along the gaze by default, with an option for the right-hand ray. Passenger seats, gunner seats, and mounted guns aim by hand.
- Recoil is an offset between the controller pose and the rendered weapon, driven by the game's per-weapon recoil state, with a spring return, a stiffer spring under a two-hand grip, and matching haptics.
- Haptics, physical melee, and the head-through-geometry fade are in scope.
- Vehicles use stick bindings by default. Physical controls (a wheel, handlebars, a helicopter cyclic and collective, a flight stick) are opt-in.
- The wingsuit and parachute use stick steering by default. Arm-pose steering is opt-in.
- Grenades are thrown physically.
- Weapons are drawn from body holsters. A radial menu is the seated and accessibility fallback, deferred with the rest of the mod UI.
- Reloads start from a button. A magazine gesture replaces the button later.
- Scoped weapons render a magnified image into a lens on the scope.
- Locomotion is head-relative by default, with an off-hand-relative option. Seated play has a crouch toggle.
- The player maps onto Rico's proportions by remapping, in the Bonelab style (see "Body scale").
- The controller scheme and the gamepad scheme switch automatically by the most recent input, and a setting can pin either one.

## What the engine provides

The game's architecture separates most of what the plan needs to separate. Each item links to the engine doc that owns the detail.

- Firing does not depend on the aim state; the remaining fire gates are listed in [aim-state.md, "Fire gating"](../../engine/gameplay/aim-state.md#fire-gating).
- Aim is kept per consumer. `CPlayerAimControl` stores its raycast in slots by target type, and every grapple consumer reads the grapple slot ([aim-pipeline.md, "The aim target"](../../engine/gameplay/aim-pipeline.md#the-aim-target-playeraimcontrol); [grapple-pipeline.md, "Readers of the grapple slot"](../../engine/gameplay/grapple-pipeline.md#4-readers-of-the-grapple-slot)).
- Fire direction is one stored point per weapon, and the shot origin is that weapon's muzzle ([aim-pipeline.md, "Fire direction, end to end"](../../engine/gameplay/aim-pipeline.md#fire-direction-end-to-end-getshotmatrix)).
- Wielded weapons ride hand attach bones through the `SetJoint` machinery, one bone per dual-wielded gun ([hands-locomotion-and-state.md §1](../../engine/character/hands-locomotion-and-state.md#1-weapon-to-hand-attachment-the-props-system)).
- The weapon's render transform and the hand mesh are both interpolated in the render phase, by separate paths ([hands-locomotion-and-state.md §1, "Where the weapon's world transform is computed per frame"](../../engine/character/hands-locomotion-and-state.md#where-the-weapons-world-transform-is-computed-per-frame)).
- Input can be injected by action ID, and every vehicle class and the wingsuit read analog values ([input.md, "The write API"](../../engine/gameplay/input.md#the-write-api); [input.md, "Analog axes read by vehicles and the wingsuit"](../../engine/gameplay/input.md#analog-axes-read-by-vehicles-and-the-wingsuit)).
- The grenade's solved launch velocity is stored where a post-hook can replace it ([aim-pipeline.md, "Grenades"](../../engine/gameplay/aim-pipeline.md#grenades)).
- The on-foot locomotion task writes the character proxy's velocity every tick, idle included, and the proxy resolves collision ([hands-locomotion-and-state.md §4.1](../../engine/character/hands-locomotion-and-state.md#41-the-locomotion-task-runs-at-idle-and-writes-the-proxy-input-directly)).

The limits are as follows. Native aim assist runs only on the gaze ray. The dual-wield slot is one weapon object with two alternating barrels that share one target. The aim state has many consumers ([aim-state.md, "Consumers"](../../engine/gameplay/aim-state.md#consumers)), and the pin has to cut the ones that affect the body. The sim runs at about 30 Hz, so anything posed in the sim phase lags the controllers unless it is updated again at render time.

## The body model

### The aim-state pin

The aim state is the set of flags [`AimState::m_AimingWeapon`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::character::AimState##flag-m_AimingWeapon) and [`AimState::m_AimingGrapple`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::character::AimState##flag-m_AimingGrapple) in [`Character::m_AimFlags`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::character::Character##field-m_AimFlags), and the state bit [`StateBitFlag::SB_IS_AIMING_WPN_OR_GRPL`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/state::StateBitFlag##variant-SB_IS_AIMING_WPN_OR_GRPL) that mirrors them. It selects between the no-aim and aim families of full-body animation states, switches the locomotion tasks between run-and-steer and aim-relative strafing, runs the aim IK tasks, and changes jump and hang facing.

The mod pins the aim state clear for the local player. The first approach is the graph state bit [`StateBitFlag::SB_NO_WEAPON_AIM`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/state::StateBitFlag##variant-SB_NO_WEAPON_AIM), which forces aim off natively. The fallback is to suppress the aim-flag writes in `HandleAimingInputPlayer` and `PreSimUpdateGrapple`. The existing FPS-movement shim keeps forcing the flags for the duration of each locomotion task's `Update`, so the locomotion tasks queue strafe acts while the animation graph stays in the no-aim family. This combination is what ships today outside of firing, so it has already been exercised in play.

The pin was chosen over a pin to the set state for the following reasons.

- `CPlayer::UpdateMentalState` sets the combat mental state whenever `m_AimingWeapon` is set. A permanently set aim state would keep the player permanently in combat, which changes how AI perception and civilians react, and would prevent the relaxed walk family from playing.
- The aim family brings the arm aim IK, the head look-at switch-off, a spine blend toward the character-level aim point, and aim-specific jump facing. Each would need to be cut separately.
- With the pin clear, firing still counts as weapon input and still moves the mental state into combat, so the combat state keeps its meaning: firing starts combat, walking does not.

The costs are small. The laser-sight weapon component draws its beam only while `m_AimingWeapon` is set; the mod draws its own laser. The motorcycle has aim sub-states that the pin disables, and the vehicle weapon work replaces them. The upper body holds the relaxed carry grip. With the arms driven by IK from the shoulders, only the fingers and the grip pose come from that animation, and a carry grip still holds the weapon. This needs checking in the headset.

The pin also needs:

- `m_aim_timer_override = 0` on held weapons, so that no aim tail follows a shot;
- the wall-penetration block cleared for the held weapon, because the rendered muzzle is at the controller rather than where the animation would put it;
- the player's `NAimIKTask`, `NRightArmAimIK`, `NGrapplingFireArmIK`, and `NReelInAimIK` skipped, because the controllers own the arms;
- the character-level aim point ([`Character::m_AimTargetPositionWeapons`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::character::Character##field-m_AimTargetPositionWeapons), and its copy on the control-parameters blackboard, [`ControlParameters::m_AimTargetWeapons`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::control_parameters::ControlParameters##field-m_AimTargetWeapons)) held at a neutral point ahead of the body, so that no residual aim blend twists the spine; the per-weapon targets carry the hand rays.

### The mental state

`CPlayer::UpdateMentalState` keeps running, because AI perception reads it. Its effects on the player's body are removed. [`StateBitFlag::SB_DISABLE_MENTAL_STATE_DRAW_WEAPON`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/state::StateBitFlag##variant-SB_DISABLE_MENTAL_STATE_DRAW_WEAPON) stops the automatic draw when combat starts, and the mod swallows `ACT_ON_UNEQUIP_WEAPON_SLOW`, which the relaxed transition queues as an automatic holster. The walk family stays the one the pin selects. Drawing and holstering happen only at the holsters.

### Locomotion and facing

Locomotion is aim-relative strafing in all on-foot cases, through the FPS-movement shim. The stick direction is relative to the head by default, with an option for the off-hand controller's pointing direction. The body's yaw is the existing scheme: stick turning (smooth or snap), plus the torso following the head through a deadzone. The torso-follow drives the visible body and the movement frame only; it never rotates the view. Physical turning is always honoured.

### Hands

The arms are driven by HumanIK from the shoulders to the controller poses at all times, except in states that the hand tiers assign otherwise. The wrist effectors ([`Effector::LEFT_WRIST`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/animation::ik::Effector##variant-LEFT_WRIST) and [`Effector::RIGHT_WRIST`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/animation::ik::Effector##variant-RIGHT_WRIST)) take position and rotation targets from the controllers, the weapon attach bone follows the dominant hand, and the game's own arm IK is skipped (see "The aim-state pin"). The existing body IK already queues shoulder, chest, and wrist targets on the `MAIN` pass; the hand targets extend it, and its `defer_while_aiming` rule no longer applies, because the aim state no longer changes.

Hands and weapons are updated again at render time. The sim-phase pose places them at the 30 Hz sim tick, and the engine's interpolation adds one to two ticks of lag behind the controllers. Two render-phase seams replace the stale transforms with ones built from the latest controller pose: the `CWeaponBase::UpdateRender` post-hook for the weapon model, and the existing `SetMatrixPalette` hook for the hand and forearm bones of the player's skin. Both seams are updated together from the same pose. Updating only the weapon would move the gun away from the hand mesh, which is why the sim-phase placement drives the attach bone rather than the weapon's own cached matrix. The muzzle flash, the attached effects, and the shot origin stay at the sim rate, so at high hand speed they trail the rendered muzzle slightly.

### Hand tiers

Each scripted state that wants the hands is assigned one of four tiers. A state can move up a tier later without changes to the rest of the design.

1. Free. The hand follows its controller and nothing is attached to it.
2. Action runs. The game performs the action, such as a reload timer and the weapon model's own magazine and bolt animation, while the hands stay on the controllers.
3. Anchored. The hand is attached to a world point, and its movement drives something in the game.
4. Yielded. The animation owns the hands, blending out from the controllers and back. A translucent ghost of the controller-driven hands stays visible in the world-space overlay, so that the player keeps sight of their real hands.

The starting assignments are as follows.

| State | Starting tier | Promotion |
|---|---|---|
| On foot, empty-handed or holding weapons | Free | none needed |
| Reload, pump, bolt, weapon draw from a holster | Action runs | the magazine gesture retires the reload button |
| Grapple fire | Free: the game's fire arm IK is skipped and the hook leaves the gauntlet | none needed |
| Reel-in flight (`GHS_REELING_IN`) | Anchored: the grapple hand holds the line where the controller is | none needed |
| Grapple hang, upside-down, and stunt positions (`GHS_REELED_*`) | Anchored: the grapple hand holds the line, and movement comes from the stick | hand displacement drives the hang movement inputs |
| Standing on a vehicle (stunt position) | Free | none needed |
| Ledge grabs, climbs, vaults, stunt step-up and step-down | Yielded | physical climbing, which would be a rebuild of the traversal controller |
| Vehicle entry, exit, and hijack | Yielded | the seat transitions in "Roomscale design" |
| Driving and flying with stick bindings | Free: the wheel animates without the hands | none needed |
| Driving and flying with physical controls | Anchored: the hands on the wheel, handlebars, or sticks | none needed |
| Mounted guns and turrets | Anchored: the hands on the handles, which aim the gun | none needed |
| Parachute, stick steering | Free | the toggles become anchored under opt-in arm steering |
| Wingsuit, stick steering | Yielded: the wing membrane is skinned between the arms and the body, and free arms may deform it | arm steering anchors the arms to the wing pose |
| Swimming | Yielded | arm-stroke swimming |
| Interactions (sabotage, planting, using objects) | Yielded | per interaction |
| Ragdoll, knockdown, death, and cutscenes | Yielded | none planned |

### Body scale

Rico is about 1.8 to 1.9 m tall. The mapping follows Bonelab's remapping rather than scaling the world. The world stays at true scale, because vehicle sizes, building sizes, and the sense of speed depend on it. The eyes sit at Rico's eye height. Hand offsets from the shoulders scale by the ratio of Rico's arm span to the player's. Vertical head travel (ducking and crouching) scales by the ratio of the two heights. Horizontal roomscale travel stays 1:1. Boneworks keeps motion 1:1 against a fixed body, and players report the mismatch as permanently bent knees or a misplaced floor; the remap avoids that.

The player's height and arm span are hardcoded in config for now. A calibration step (stand straight, then T-pose) is deferred with the rest of the mod UI.

### Posture

Three postures are supported.

- Seated: LOCAL space. The calibrated seated eye height maps to Rico's standing eye height, roomscale translation is off, and a button toggles between standing and crouched eye height.
- Standing: STAGE space for the true floor height. The player stands in place, roomscale translation is off, and crouching is physical.
- Roomscale: STAGE space, with the roomscale chase loop on.

OpenXR has no signal for whether the user is seated, and SteamVR's OpenXR runtime exposes only a standing basis. The mod can read two things. `xrGetReferenceSpaceBoundsRect` on the STAGE space returns a rectangle for a room-scale boundary, and returns `XR_SPACE_BOUNDS_UNAVAILABLE` or a very small rectangle for a standing-only or stationary setup, which separates standing from roomscale. The head height above the STAGE floor, compared with the calibrated standing height, indicates a seated player. The posture setting is `auto | seated | standing | roomscale`. In `auto`, the stage bounds select between standing and roomscale without asking. A prompt suggesting the seated posture when the head is low is deferred with the rest of the mod UI; until then, the seated posture is set explicitly.

## Input

### Action sets

OpenXR action sets `onfoot`, `vehicle`, `airborne`, and `ui` are active one at a time, selected by the per-tick mode detector (see "Mode detection"). Grip and aim pose actions exist for both hands. The mod ships suggested bindings for the Touch and Index interaction profiles. Per-user remapping is the runtime's own binding interface (SteamVR or xrizer), not a mod UI. Output goes through `ForceSetPressed` and `ForceSetClicked` after `InputDeviceManager::Update`, following the timing rules in [input.md](../../engine/gameplay/input.md).

The controller scheme and the gamepad scheme switch on the most recent input. Tracked controllers producing input select the controller scheme. Any gamepad or keyboard input selects the gamepad scheme, which restores gaze aiming, the gamepad bindings, and the animated hands. A setting can pin either scheme.

### Mode detection

The mode detector replaces the headpose sim's binary on-foot test (the `EvaluateCharacterOrientation` call counter), which reads a swimmer as not on foot. It reads the engine's own state signals, documented in [hands-locomotion-and-state.md §5](../../engine/character/hands-locomotion-and-state.md#5-per-tick-character-state-signals): the character state-bitflag words, the state-machine-hash predicates, the vehicle class predicates, the grapple hook state, and the pause and UI signals. It runs once per sim tick, in the same phase as the headpose sim: one read of the local character, one snapshot of the three state words, and a `GetVehiclePtr` call only when a vehicle state is indicated, with the pointer cached for the tick.

The first matching rule selects the action set. The order follows the engine's state overlaps ([§5.6](../../engine/character/hands-locomotion-and-state.md#56-state-overlap-and-precedence)).

1. A menu is up: `ui`. The signals are the physics-system pause byte, the `CGamePauseUtility` paused state, the mod's own panel, and, for the game's map and commlink screens, the UI steering focus, whose per-tick global is not yet pinned.
2. A teleport happened this tick: hold the current set and re-base the rig.
3. Seated in a vehicle (`IsInVehicleAttachState`, which spans the entry and exit animations): `vehicle`, with the class resolved in the order helicopter, plane, motorcycle, car, sea, and the driver seat selecting the driver bindings.
4. Grapple traversal (reeling in, pre-reel, hanging, or reeled in): the gameplay set stays (`onfoot` or `airborne`), and the reel or hang sub-state is flagged for the hand tiers.
5. Wingsuit: `airborne`, wingsuit variant.
6. Parachute, including the vehicle-parachute attachment: `airborne`, parachute variant.
7. Freefall, fall, or fastfall: `airborne`.
8. Swimming: `onfoot`, swim variant.
9. Otherwise: `onfoot`. The orientation-evaluation counter stays as a corroborating on-foot heartbeat.

A mode switch is debounced by one or two ticks, so a single-tick flicker during an animation blend does not switch the action set. The state-machine-hash predicates take precedence over raw bits during known transitions. A null `m_ControlParameters` or `m_InstanceData` during the first frames after spawn holds the previous mode. Boats and jetskis share one class until the jetski discriminator is pinned.

### Grip semantics

Guns are sticky, so the grip is an edge, never a hold. A grab volume is a region where a grip press grabs instead of performing its default action. The grab volumes are the holsters on the body, a two-handed weapon's foregrip, the grenade pouch, and, under the opt-in physical controls, the wheel, the handlebars, the flight sticks, and the turret handles. The world-space overlay highlights the volume the hand is in. The holster volumes are small and close to the body, so that a grapple fired in a hurry near the hip does not draw a weapon instead.

Holding a grenade is the exception to sticky grips: the grenade is held while the grip is held, and thrown when the grip is released.

### Per-mode bindings

The bindings are drafts for the Touch layout. Index shares the topology, and its extra inputs (the trackpad and finger curl) stay unbound until the gesture work. The tables follow the game's own mode partition and stay congruent with [the default gamepad keymap](../../engine/gameplay/input.md#the-default-gamepad-keymap) where the VR model allows, so that a player's gamepad habits carry over; the action IDs each mapping drives are in [the mapping to action table](../../engine/gameplay/input.md#the-mapping--action-table). The action IDs that serve several mappings (holster as hold-`RELOAD`, melee as `PUSH_GRAPPLE`, sniper zoom as `PRECISION_AIM`, and nitrous and turbo jump as `USE_VEHICLE_MOD`) are separated by the game through tap versus hold and through the mode context, so the injection reproduces the tap or hold timing of the mapping it stands for. Wherever a mapping is marked as a gesture, the button remains as the fallback.

#### On foot

| Action | VR binding |
|---|---|
| Move | L stick; the magnitude selects walk or run |
| Body turn | R stick, smooth by default, snap as an option |
| Fire the dominant-hand weapon (`FIRE_RIGHT`) | R trigger |
| Fire the left pistol while dual-wielding | L trigger (see "Dual-wield") |
| Fire grapple (`FIRE_GRAPPLE`: fire; tap while attached to zip; hold to tether) | L grip outside grab volumes; the yank gesture also zips |
| Retract tethers (`RETRACT_GRAPPLE`) | L trigger while the left hand is empty; the binding while dual-wielding is open |
| Release tethers and cancel (`RELEASE_GRAPPLE`, `CANCEL`) | Y |
| Jump, stunt, open parachute | A |
| Use item, enter vehicle, open wingsuit | X |
| Reload | B; later the magazine gesture |
| Holster | grip press at a holster |
| Weapon slot select | draw from a holster; until holsters exist, L stick click cycles `NEXT_WEAPON` |
| Throw grenade | grab from the grenade pouch and throw; R stick click is the fallback |
| Plant and detonate explosives | R stick click (context) |
| Melee (`PUSH_GRAPPLE` under the hammer ability) | a physical strike with either hand |
| Precision aim and sniper zoom | removed; scopes magnify optically (see "Scopes") |
| Reel-in context: cancel, boost, hang jump, slingshot | Y, hold L trigger, A, A |
| Pause, commlink | Menu tap, Menu hold |

#### Land vehicles (car; motorcycle in the last rows)

| Action | VR binding |
|---|---|
| Steer | L stick; opt-in wheel or handlebars |
| Accelerate and reverse | R trigger and L trigger |
| Handbrake | Y |
| Nitrous and turbo jump (`USE_VEHICLE_MOD`, upgrade-gated) | hold B and tap B |
| Fire vehicle weapons, primary and secondary | R grip and L grip |
| Enter gunner seat, roof stunt | hold X, A |
| Exit vehicle | X |
| Horn | L stick click |
| Look back, vehicle camera, recenter camera | removed; the neck replaces them, and F7 recenters |
| Motorcycle personal weapon (`MC_FIRE`) | R grip, aimed by the right hand |
| Motorcycle lean and tilt | L stick; opt-in handlebar lean |

#### Helicopter

| Action | VR binding |
|---|---|
| Collective up and down | R trigger and L trigger; opt-in collective lever |
| Cyclic (forward, back, and roll) | L stick; opt-in cyclic stick |
| Yaw | R stick X |
| Fire primary and secondary | R grip and L grip, aimed by the gaze |
| Exit, stunt, nitrous | X, A, hold B |

#### Plane

| Action | VR binding |
|---|---|
| Pitch and roll | L stick; opt-in flight stick |
| Rudder | R stick X |
| Thrust up and down | R trigger and L trigger; opt-in throttle |
| Fire primary and secondary | R grip and L grip, aimed by the gaze |
| Exit, stunt | X, A |

#### Boats and jetskis

| Action | VR binding |
|---|---|
| Steer | L stick; opt-in wheel or handlebars |
| Accelerate and reverse | R trigger and L trigger |
| Fire, and the jetski personal weapon | R grip, aimed by the gaze (boat) or the right hand (jetski) |
| Nitrous and turbo jump | hold B and tap B |
| Exit | X |

#### Wingsuit and parachute

| Action | VR binding |
|---|---|
| Steer (analog `MOVE_*`) | L stick; opt-in arm-pose steering |
| Air brake (`CANCEL` hold, upgrade-gated; `WINGSUIT_AIRBRAKE` when weaponized) | L trigger |
| Boost and evade (weaponized) | B; A with a stick flick |
| Fire weapons (weaponized wingsuit or parachute) | R trigger and R grip, aimed by the right hand |
| Open or close the parachute | A |
| Grapple (slingshot boost) | L grip |

#### UI

While a game menu is open, the `ui` action set replaces the gameplay bindings, matching the game's own `END_UI_MAPPINGS` boundary. The first version maps the game's UI actions like the gamepad: the stick navigates, A confirms (`GUI_OK`), B cancels (`GUI_CANCEL`), and the grips page and change tabs. A laser pointer on the floating panel is deferred with the rest of the mod UI. `payload/src/hud/pointer.rs` already isolates the pointer source for it.

## Aiming

### Guns

The dominant hand's ray is written into each held weapon's `m_AimTargetPosition` at the per-weapon aim update (`CWeaponBase::UpdateFromUserPostSim_Parallel`). The ray is the weapon's sight line, not the controller's own axis: the controller pose, times the weapon's grip transform, times a per-weapon sight offset. The target point is the ray's hit point, found with `CPhysicsSystem::CastRay` from the sim phase, and it is kept well beyond the near-muzzle distance at which `GetShotMatrix` falls back to the barrel direction. The per-weapon field is the seam because it is the only one that gives two weapons two directions: the character-level aim point (`m_AimTargetPositionWeapons`) fans out unchanged to every held weapon. The character-level write still runs every frame, with the neutral point from "The aim-state pin", because a frame without it sets no `WasSetThisFrame` flag and the game then aims every weapon 25 m ahead of the body ([aim-pipeline.md, "From aim control to the character"](../../engine/gameplay/aim-pipeline.md#from-aim-control-to-the-character-cplayerupdatepostcamera)).

The per-weapon sight offsets are derived and then tuned. The fire-point matrices (`m_InternalFirePositions`) give the muzzle origin, and probably the barrel axis if the matrices are oriented along it; that is an RE question. An offline pass extracts the weapon models from the game archives, finds the sight geometry along the top of the barrel axis, and emits a table keyed by weapon hash. A debug-UI tool overrides the weapons whose derived sights are wrong: the player lines up the sights on a marker and saves the offset.

Guns show no reticle by default. The options are a dot at the hit point, at the hit point's true depth so that it fuses in stereo, and a laser line from the muzzle to the hit point. Both are drawn by the world-space overlay layer.

### The grapple

A post-hook on [`PlayerAimControl::UpdateDirectAim`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/aim::aim::PlayerAimControl##func-UpdateDirectAim) casts a ray from the left controller and overwrites the grapple slot, index [`TargetType::TARGET_GRAPPLE`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/aim::target::TargetType##variant-TARGET_GRAPPLE), of [`m_AimPos`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/aim::aim::PlayerAimControl##field-m_AimPos), [`m_DirectPositions`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/aim::aim::PlayerAimControl##field-m_DirectPositions), [`m_DirectTargets`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/aim::aim::PlayerAimControl##field-m_DirectTargets), [`m_DirectHits`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/aim::aim::PlayerAimControl##field-m_DirectHits), and [`m_IsInRange`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/aim::aim::PlayerAimControl##field-m_IsInRange), and the last-valid grapple cache. Surface zips need only the ray hit. Entity attachment and tethering need the game's grapple target scoring run against the left-hand ray, so that the grapple slot of [`m_Target`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/aim::aim::PlayerAimControl##field-m_Target) and [`m_LastHitGrappleTargetGO`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/aim::aim::PlayerAimControl##field-m_LastHitGrappleTargetGO) are filled the way the game would fill them. Every grapple consumer reads the grapple slot, so the hook fire, the tether semantics, and `IsLastValidGrappleAimPosStillValid` follow the left hand without further changes. Two alternatives are rejected. Overwriting only the latched target at fire time (`CGrapplingHook::SetUpTarget`) corrects the hook but leaves every reader of the live slot on the gaze ray. Telling the grapple's `GetCameraMatrix` calls apart by return address does not work, because the one raycast inside `UpdateDirectAim` serves the weapon and grapple slots together.

The game's grapple reticle pins to the centre of the screen when the aim uses a direct position, so it cannot show a hand ray. The grapple readout is drawn by the world-space overlay layer instead: a highlight on the surface or object the grapple would attach to, shown only when the target is valid and in range, with a distinct style for targets that support tethering, and a gauntlet beam from the wrist to the target point. Each is configurable.

The yank gesture is a sharp pull of the left hand back toward the body while a line is attached. It issues the same `FIRE_GRAPPLE` tap that zips.

### Dual-wield

The dual-wield slot is one `CWeaponBase` with a `CWeaponData` component per gun. Each gun has its own attach bone, so gun 0 rides the dominant hand and gun 1 rides the other hand. On foot the game exposes one fire input and alternates the barrels internally, at the tail of `Fire_Projectile` (the component index [`WeaponBase::m_NextWeaponToFire`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/equipment::weapon::WeaponBase##field-m_NextWeaponToFire) and each component's fire-position cursor [`WeaponData::m_InternalFirePositionCurrentIndex`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/equipment::weapon::WeaponData##field-m_InternalFirePositionCurrentIndex)). Each trigger fires its own component: the mod pins both the cursor and the dispatched `CWeaponData` to that component, so that the muzzle origin and the flash stay consistent, and writes that hand's ray into `m_AimTargetPosition` before the shot is built. Ammunition, recoil, heat, and effects are counted per fire call, never per barrel, so nothing is counted twice.

The two guns share one weapon state, so a reload is one animation for both. Under the action-runs tier the hands stay on the controllers, and both guns stop firing for the reload's duration.

### Aim assist

Aim assist is re-derived per hand ray. The mod scores the game's own candidate targets against each hand ray and moves the written aim point toward the best candidate by a strength factor. The default strength is zero. The engine's shape casts suit the candidate query, and `CMagnetWeaponComponent::CastSphereAgainstStatic` is an existing consumer to follow, although that function is not yet located ([hands-locomotion-and-state.md §4.2](../../engine/character/hands-locomotion-and-state.md#42-raycast-and-scene-query-api-cphysicssystemcastray-and-related-functions)).

### Vehicle, mounted, and wingsuit weapons

[`Vehicle::SetAimTargetPosition`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/vehicle::vehicle::Vehicle##func-SetAimTargetPosition) writes the vehicle aim point ([`Vehicle::m_AimingTargetPosition`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/vehicle::vehicle::Vehicle##field-m_AimingTargetPosition)) only for the driver's seat, and every built-in vehicle weapon aims from it. The driver's vehicle weapons aim along the gaze by default, which is the current behaviour. An option overwrites `m_AimingTargetPosition` from the right-hand ray. Turrets turn toward the aim point at a limited rate, so a hand-aimed turret visibly trails the hand. Mounted guns, including a gunner's seat, use the inventory slot [`WeaponSlot::E_SLOT_HEAVY`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::inventory::WeaponSlot##variant-E_SLOT_HEAVY) and the per-weapon seam, and aim by hand. Weaponized wingsuit guns aim by the right hand through the per-weapon seam.

### Recoil and haptics

Each held weapon has a recoil offset between the controller pose and the rendered weapon. Every shot adds a kick to the offset, scaled from the game's per-weapon recoil accumulators ([`WeaponBase::m_CurrentScatterValue`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/equipment::weapon::WeaponBase##field-m_CurrentScatterValue) through [`WeaponBase::m_RecoilPitch`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/equipment::weapon::WeaponBase##field-m_RecoilPitch), advanced once per shot), and a spring returns the offset toward zero. The aim ray follows the rendered weapon, so the kick affects accuracy. A player who holds still sees the gun climb; a player who pulls against the climb keeps it on target, as with real recoil. When firing stops, the spring returns the weapon to the controller pose. A two-hand grip on the foregrip stiffens the spring and reduces the kick. The game's own scatter and spread stay in effect on top of the offset.

Haptic pulses come from the game's own feedback calls (`RumbleFire`, `CameraShakeFire`, and the damage and impact rumbles), routed to the hand that caused them. The recoil pulse scales with the kick. Grapple fire and attachment pulse on the grapple hand.

## Weapon handling

### Holsters

The holsters are grab volumes on the body: the hips for pistols, over the shoulders for two-handed and special weapons, and the chest for explosives. A grip press in a holster draws that slot's weapon with `SELECT_DUEL_WIELD`, `SELECT_TWO_HANDED`, `SELECT_TWO_HANDED_SPECIAL`, or `SELECT_EXPLOSIVES`. A grip press with the held weapon inside its holster holsters it. The draw plays under the action-runs tier. The radial menu fallback is deferred with the rest of the mod UI.

### Two-handed weapons

A two-handed weapon rides the dominant hand's attach bone and aims by that hand alone. The foregrip is a grab volume for the off-hand. While the off-hand holds the foregrip, the weapon's direction is the line from the dominant hand to the off-hand, the off-hand's wrist is IK-pinned to the foregrip, and the recoil spring stiffens. The off-hand releases the foregrip with a grip press or by moving well away from it. The engine's own secondary-hand grip IK (`UpdateSecondaryHandIKPass`) is suppressed for the off-hand, so that the engine does not pull the hand onto the foregrip on its own.

### Reloads

The reload button starts the native reload. The weapon cannot fire until the reload finishes, and the weapon model plays its own magazine and bolt animation while the hands stay on the controllers. The later magazine gesture moves the off-hand to a belt pouch and then to the weapon's magazine well, and retires the button.

### Grenades

A grip press on the grenade pouch puts a grenade in the hand; releasing the grip throws it. The throw needs three changes: triggering the throw without the wind-up animation, spawning the grenade at the controller's position (the game spawns it at the animated hand on an animation track event), and overwriting the solved launch velocity, [`Explosive::m_DeferredInitialVelocity`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/equipment::explosive::Explosive##field-m_DeferredInitialVelocity), with the controller's release velocity through a post-hook on [`Explosive::PropelToward`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/equipment::explosive::Explosive##func-PropelToward). The R stick click fallback throws along the hand ray with the game's own arc.

### Melee

A strike with either hand above a speed threshold triggers the melee attack toward the object the hand hits or points at. The melee target slot (the [`TargetType::TARGET_MELEE`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/aim::target::TargetType##variant-TARGET_MELEE) entry of [`PlayerAimControl::m_Target`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/aim::aim::PlayerAimControl##field-m_Target)) is written from the striking hand. The game gates melee on the hammer ability.

### Scopes

Precision aim adds a camera-tree FOV hint, shows a 2D sniper overlay, and hides the player's body, which also hides the hands. The mod suppresses all three. A scoped weapon instead renders a narrow-FOV view of the scene into a texture drawn on the scope's lens. To limit the cost of a third scene render, the lens renders only while the scope is near the eye, in mono, and at reduced resolution.

## The world-space overlay layer

A world-space overlay layer draws sprites and lines into the scene with depth, per eye. It carries the grapple highlight and beam, the optional gun dot and laser, the grab-volume highlights, the ghost hands of the yielded tier, and debug controller gizmos. The floating HUD panel cannot do this, because it draws a 2D texture on one plane, and the per-hand readouts need their own depths.

## Roomscale design

The design for physical locomotion is resolved against the engine answers in [hands-locomotion-and-state.md §4](../../engine/character/hands-locomotion-and-state.md#4-proxy-input-scene-queries-teleports-seat-pose-root-motion-capsule-and-pause). It follows the topology in [head-and-body.md](../body/head-and-body.md): the head is the ground truth and the body follows it. The camera is never derived from the body's motion, so the view stays 1:1 with the real head regardless of what the body does. The chase loop runs only in the roomscale posture.

### The chase loop

The camera is already placed from the measured HMD pose each frame (the anchor plus the cockpit offset), so the loop does not track an offset to consume. It measures the offset again from live poses each tick, and any movement the body fails to make leaves the offset nonzero. Per sim tick:

1. Measure the body-frame XZ offset from the character to the player's physical head (the cockpit position the pose path already computes).
2. Outside a deadzone of about 10 cm, add a chase velocity `clamp(offset / dt, max_chase_speed)` to the on-foot locomotion result. The seam is a post-call hook on the locomotion task, adding to the world velocity it writes to the character proxy ([`CharacterInput::wantedVelocity`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/physics::CharacterInput##field-wantedVelocity), in [`PfxCharacterInstance::m_Input`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/physics::PfxCharacterInstance##field-m_Input)). The task runs every on-foot tick including idle, so walking from a standstill works. `dt` is the sim tick, never the render frame. The engine's own displacement evaluation uses either root motion or code-driven displacement, never both, so the add does not double-count against it.
3. The proxy resolves collision, stairs, and slopes. The next tick's measurement reflects what happened, so no read-back step exists. A wall between the player and the body leaves a persistent offset: the view stays with the real head, and a `CastRaySimple` probe from the last unobstructed head position to the current one drives a fade when the head goes deep into geometry. The probe runs in the sim phase, never from a render hook, because scene queries run against the live physics world.
4. Physical walking and stick walking share one channel. The chase velocity scales down with the stick magnitude, so deliberate stick locomotion takes precedence, and both are expressed as the same body-local move vector.

The chase applies only through the post-call hook on the on-foot locomotion task. Every other state that moves the character proxy (grapple, jump, stunt, and ragdoll) is driven by a different task, which the hook does not touch, and a seated character follows its vehicle through the seat attachment, so neither receives the chase; the loop also stands down while `IsInVehicleAttachState` holds. Publishing the offset as the game's code-driven displacement direction is an alternative seam, rejected because it interacts with the game's own code-driven displacement users.

Physical translation never rotates the body, consistent with the rule that orientation changes only with a movement action. The chase velocity is body-local. The aim-relative strafe blend space, which the FPS-movement shim already selects, animates sidesteps and backpedals, and the body-follows-head deadzone scheme owns the torso yaw.

A teleport re-bases the cockpit baseline for that tick instead of feeding a chase spike. The game has no reliable per-character teleport flag: the character field once believed to be one is the stunt auto-exit setting ([`AbilityFlags::m_IsStuntAutoExitEnabled`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::character::AbilityFlags##flag-m_IsStuntAutoExitEnabled) in [`Character::m_AbilityFlags`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::character::Character##field-m_AbilityFlags)), and the per-character frames-since-teleport counter is never reset. Detection therefore combines a hook on the game world's instant teleport, the `game_teleporting_initiated` and `game_teleporting_completed` events, and a per-tick distance-delta fallback for any writer those miss ([hands-locomotion-and-state.md §4.3](../../engine/character/hands-locomotion-and-state.md#43-teleport-writers-and-teleport-signals)).

Recentering on foot re-bases yaw and height only. The chase loop keeps the position converged, so no position needs recentering, and a position recenter could move the body through a wall that the chase could not cross. Position re-basing exists only in seat transitions.

### Crouch, in stages

1. Visual crouch, first and behind config: a low head target with the hips-translation and floor-contact solve steps enabled folds the body through the same HumanIK pass the head target already uses. The feet stay planted and the hips drop.
2. Capsule height: the engine swaps the proxy capsule per state itself (`SetProxyState`, which calls `hknpWorld::setBodyShape`), so a shorter crouch capsule uses the engine's own mechanism, on the sim thread as the engine does. The crouch swap defers to the engine's own proxy-state changes for swimming, vehicles, and ragdoll. This stage is gated on the visual crouch working.
3. AI response: whether enemy perception responds to a lowered head is measured in the headset.

In the seated posture, the crouch toggle moves the eye height between the standing and crouched presets, and drives the same visual crouch.

### Seat transitions

JC3 vehicles are entered through animations in which Rico crosses metres of world, opens doors, and hijacks drivers. Under a head-anchored camera the entry is a forced camera movement that ends at an arbitrary yaw, which is a strong vection source. Bonelab avoids the problem (its vehicles are mount-in-place with trigger-volume seats and no entry animation), and LukeRoss's R.E.A.L. GTA V mod, a seated camera-only port, advises players to accept the ride or close their eyes. The mode enum below shares one seat re-base mechanism: the seated head reference is the character's own head and eye bones, which ride the vehicle through `m_AttachBone` and `m_AttachOffset`.

- Instant, the seated default and the roomscale default until Ceremony exists: at seat lock the cockpit re-bases immediately and the world stays live. The player's real posture is tolerated, and the player can recenter with F7. The entry animation is hidden by a fade.
- Fade: fade out when the entry animation takes the camera, and fade in seated with the re-base complete. Likely folded into Instant rather than kept as a separate setting.
- Ride: the head stays anchored through the full animation with nothing hidden. This is what the implementation does before the other modes exist, and it stays as the intense option.
- Ceremony, deferred with the rest of the mod UI because it needs a confirmation prompt: the entry animation plays; at seat lock the game clock freezes and the floating panel asks the player to take their seat and confirm; on confirmation the mod re-bases, unfreezes, and enables the `vehicle` action set. The freeze is a mod-side clock freeze through the hooked `CClock::Update` (zero game dt, skipped sim tick), never the game's own pause, because `UpdateRenderPaused` does not run `CameraTree::UpdateRenderContexts`, and a real pause freezes the camera under head motion. Exit reverses the sequence. Whether the frozen world dims is decided in the headset.

The roomscale chase is disengaged while seated in every mode.

## Milestones

The milestones follow a vertical slice: the riskiest part of the design (the aim-state pin, controller-owned hands, and per-hand aiming) is built and tested on foot first, and breadth follows. The only developer and user for the foreseeable future is the project author, so milestone 1 hardcodes the author's body dimensions in config rather than waiting for calibration.

### 0. Bindings

Author the pyxis definitions for the seams milestone 1 uses, and verify each offset independently against the IDB, because prose transcription has no compile check. Definitions go in `20206564`. The list:

- `CPlayerAimControl`'s slot arrays (`m_AimPos`, `m_DirectPositions`, `m_DirectTargets`, `m_DirectHits`, `m_IsInRange`, `m_Target`) and the grapple cache;
- `CWeaponBase`'s `m_AimTargetPosition`, `GetShotMatrix`, the recoil accumulators, the component vector, `GetCurrentBoneAttachement`, `ChangeBoneAttachement`, `UpdateRender`, and the wall-penetration flag;
- `CWeaponData`'s T0 and T1 matrices, the fire-position array and cursor, and the model-instance pointer, plus `CModelInstance::UpdateMatrix`;
- `CPhysicsSystem::CastRay` and `CastRaySimple`, and `CRaycastFilter`;
- the character state-bitflag words ([`Character::m_ControlParameters`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::character::Character##field-m_ControlParameters) and the [`UserVM::m_CurrentStateBitFlags0`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::control_parameters::UserVM##field-m_CurrentStateBitFlags0) words), the vehicle-class predicates, and `GetVehiclePtr`, for the mode detector;
- `HandleAimingInputPlayer`, `PreSimUpdateGrapple`, `UpdateMentalState`, `NAimIKTask::Update`, `NRightArmAimIK::Update`, `NGrapplingFireArmIK::Update`, and `NReelInAimIK::Update`;
- corrections to the existing `Character` flag bytes: [`AimState::m_InCargoBay`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::character::AimState##flag-m_InCargoBay) in [`Character::m_AimFlags`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::character::Character##field-m_AimFlags) is the in-cargo-bay flag, and [`DamageStateFlags::WEAPONS_ENABLED`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::character::DamageStateFlags##flag-WEAPONS_ENABLED) in [`Character::m_DamageFlags`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/character::character::Character##field-m_DamageFlags) is the weapons-enabled flag, not a damage flag.

### 1. The on-foot slice

- The OpenXR action sets and pose actions, the mode detector, and the scheme hot-swap. The `onfoot` set uses the bindings above; the other sets are gamepad-shaped so that the rest of the game stays playable.
- The aim-state pin and the mental-state cuts.
- Controller-owned hands: wrist IK to the controllers, the weapon on the dominant hand's attach bone, the game's arm IK skipped, and the render-time late update for the weapon and the hand bones. The body dimensions come from config.
- Gun aiming from the dominant hand, with sight axes derived from the fire-point matrices only (the model-extraction pass comes later).
- Grapple aiming from the left hand, with the slot-2 overwrite and the grapple target scoring.
- The world-space overlay layer, with the grapple highlight and beam, the optional gun dot and laser, and debug controller gizmos.
- The free and yielded hand tiers; every scripted state starts as yielded unless its table entry is free. Ghost hands can follow later in the milestone.
- Weapon slots cycle from the L stick click until holsters exist.

### 2. Posture and roomscale

- The posture setting and STAGE-space support, the stage-bounds check, and the seated crouch toggle.
- The Bonelab-style remap, still driven by config dimensions.
- The chase loop, the teleport re-base, and the head-through-geometry fade.
- The visual crouch, then the capsule swap.
- The Instant, Fade, and Ride seat-transition modes.

### 3. Holding

- Holsters and grab volumes, with the overlay highlights.
- The two-handed foregrip.
- Recoil and haptics.
- The dual-wield split.
- The action-runs tier for reloads and draws, and the anchored tier for grapple lines and hangs.
- The yank gesture.

### 4. Vehicles and flight

- Per-class vehicle bindings, playtested class by class.
- Vehicle weapon aiming (the gaze by default, with the hand option), and hand-aimed mounted guns and turrets.
- The opt-in physical controls: wheel, handlebars, cyclic and collective, flight stick, and throttle.
- The opt-in arm steering for the wingsuit and parachute, and the removal of the head look-steer from the wingsuit's steering.

### 5. Breadth

- The physical grenade throw.
- Physical melee.
- Scope lenses.
- Per-hand aim assist.
- The magazine gesture for reloads.
- The model-extraction pass for sight offsets, and the sight-tuning tool.
- Promotions in the hand-tier table, in order of how often the states occur in play.

## Reverse-engineering work

These questions are open. Each is listed with the milestone that needs its answer.

- Milestone 1: which systems read `SB_NO_WEAPON_AIM` besides the aim logic, whether the FPS-movement shim still selects the strafe branch with the bit set, and where a state bit can be set so that it holds across the character's graph evaluation.
- Milestone 1: whether skipping the player's aim IK tasks leaves any blackboard state (the aim-IK weight key [`ObjectBlackboard::AIM_IK_WEIGHT_ID`](https://ferrobrew.github.io/pyxis/#/JustCause3_Steam_20206564/item/blackboard::ObjectBlackboard::AIM_IK_WEIGHT_ID) on the character blackboard) that other tasks read.
- Milestone 1: the bones the hand and forearm late update must replace in the skinning palette, and whether the fingers are in the same batches.
- Milestone 1: whether the fire-point matrices are oriented along the barrel.
- Milestone 1: whether the grapple wire's near end follows the gauntlet when the left arm is IK-driven, or is pinned to an animated bone.
- Milestone 1: the `UpdateMentalState` behaviour to keep (the transitions the AI reads) and the transitions that affect the body.
- Milestone 3: the inputs the grapple hang reads for movement, for the anchored hang promotion.
- Milestone 3: how `UpdateSecondaryHandIKPass` can be suppressed per hand.
- Milestone 4: the wingsuit's look-steer path, which cannot be hooked in the wingsuit task's `Update` because the function contains an anti-tamper branch.
- Milestone 4: the jetski discriminator (`m_VehicleType` and the jetski-specific type), if boats and jetskis need different action sets.
- Milestone 5: triggering a grenade throw without the wind-up animation, and moving the spawn point.
- Milestone 5: the melee trigger path under the hammer ability.
- Milestone 5: a render path for the scope lens that reuses the engine's scene render at a narrow FOV.

## Risks

1. The pin may not hold cleanly. The animation graph's transition wiring is hashed and was not decoded, so a state that reads the aim bit through a path the recon did not find can still switch the body. The mitigation is to test the pin first, in milestone 1, before other work depends on it.
2. The controller-owned arms work against the authored upper-body animation. Recoil, reload, and traversal animations expect to move the arms, and the hand tiers decide per state which one wins. Tuning the blends is the largest body of iteration in the plan.
3. The late update separates the rendered weapon from the sim-rate muzzle. At high hand speeds the muzzle flash and the shot origin trail the rendered muzzle by up to a tick.
4. Unreliable sights make aiming unreliable. Hitman 3's PC VR release had a drifting offset between the controller and the in-game hand that made its aiming unreliable. The derived-then-tuned sight table is the mitigation, and the tuning tool is the fallback for each weapon.
5. The grapple target scoring has to match the game's. If the re-run scoring differs from the native one, entity attachment and tethering behave differently from the flatscreen game.
6. Vehicle classes each need their own playtest. The helicopter, plane, and boat each have their own axis set.

## Deferred with the mod UI

These features wait for a VR-native redesign of the mod's in-VR interface.

- The posture prompt that suggests the seated posture when the head is low.
- The height and arm-span calibration flow (the dimensions are in config until then).
- The radial weapon menu, the seated and accessibility fallback for holsters.
- The Ceremony seat-transition mode and its confirmation prompt.
- The laser pointer for the floating panel and the game's menus.

# Skeleton, pose, and bone override

JC3 characters are posed via a Havok `hkaPose`. This covers how the pose is stored and updated, how to read and override individual bones, and where in the frame a change must happen to reach both the camera and the rendered mesh.

## Bone store: hkaPose, model-space

The animation controller (`AnimationController`) holds a pose producer wrapping two `hkaPose` objects (current + a secondary/ragdoll pose). Each `hkaPose` keeps a local-space (parent-relative) and a model-space (character-root-relative) transform buffer (an `hkQsTransform` per bone — translation, rotation quat, scale), per-bone dirty flags, and a parent-index table. Those internals aren't needed if you use the Joint API below.

Bones are **model-space, not parent-local**: a bone's world transform is `characterWorld · boneModelSpace`.

## Read / write API (Joint)

The controller exposes a model-space `Joint` API, already in the bindings:
- `AnimationController::GetBoneMatrix` builds a `Matrix4` from the model-space joint. `Character::GetSafeBoneMatrix` resolves through a hash→index table to this.
- `AnimationController::GetJoint` reads the model-space `Joint`.
- `AnimationController::SetJoint` writes a model-space `Joint` and **propagates to all descendants** (recomputes their model space); it marks the bone local-dirty.

`Joint` is `{ Translation; Orientation; Scale }`, model-space; the quaternion is Havok order (x, y, z, w).

So overriding a bone is a `SetJoint`, not a buffer patch. To place the head at a desired world transform relative to the body: `desiredHeadModel = inverse(characterWorldT1) · desiredHeadWorld`, then `SetJoint(HEAD, desiredHeadModel)`. `SetJoint` re-derives descendants but not ancestors — fine for a head/eyes override (the neck above is left as the animation set it).

## Hierarchy and root

The root for "head relative to body" is the character world matrix `m_WorldMatrixT0/T1`. The chain is SPINE → SPINE1 → SPINE2 → STERNUM → NECK → HEAD, with `fLeftEye`/`fRightEye` as facial children of HEAD (looked up by name hash via `GetBoneIndex`). Because everything is model-space, you don't walk the chain to place a bone — one `inverse(characterWorld)` multiply suffices.

Bone rest frames are not aligned with the model axes. An absolute model-space orientation written to a bone therefore does not describe a neutral pose, and an orientation change is composed onto the animated orientation instead. `SetJoint(HEAD)` carries the facial children along, including `fLeftEye` and `fRightEye`, as verified in-game.

## Frame order, and where to override

The pose is finalized in the SIM phase and consumed (camera, then skinning) in the RENDER phase:

    SIM:    UpdatePassFinalizePose_Parallel
              -> HumanIK pass, SyncPoses, CalculateModelSpacePose   => model-space pose finalized
              -> Character::UpdatePropEffects                       <- last call
    RENDER: GameCameraManager::UpdateRender
              -> CameraTree::UpdateRenderContexts
              -> Camera::UpdateRender
            ... KickSkinningJob -> render submit

`Character::UpdatePropEffects` is the last call in `UpdatePassFinalizePose_Parallel`. It runs in the SIM phase, after the model-space pose is built. A `SetJoint` made when it returns therefore reaches both RENDER-phase consumers of the model-space buffer: the camera update and the skinning job.

## Matrix basis convention

The engine matrices are D3D-style: row-major and row-vector (`clip = p · M`, `VP = View · Proj`). A transform's basis vectors are therefore its rows: `data[0..2]` is right (+X), `data[4..6]` is up (+Y), `data[8..10]` is the +Z basis (forward is -`data[8..10]`), and `data[12..14]` is the translation. The space is right-handed and Y-up (rendering §2.6). An orientation built from a look direction takes `right = normalize(cross(up, fwd))` and `trueUp = cross(fwd, right)`, and places `right`, `trueUp`, `-fwd`, and `pos` in rows 0 to 3.

## Relation to HumanIK

The engine's HumanIK pass consumes effector targets inside `UpdatePassFinalizePose_Parallel`, before `CalculateModelSpacePose`. A target queued at `Character::UpdatePropEffects` is therefore too late for the current frame's solve. The HumanIK solve blends with the animated pose, whereas `SetJoint` after finalization replaces the model-space joint outright. The solver, its gate, and the effector ids are in `humanik.md`.

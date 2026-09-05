# Trailer tooling

Tools for directing and capturing a first-person trailer from inside the headset. Two groups: directing tools in the debug menu (spawn, teleport, heat, time, cinematic mode), and an in-process capture path that records one eye at full resolution with audio. This doc is the working todo list for that work; each item becomes normal mod code and docs as it lands, and the item here is ticked off.

## Decisions

- Capture records the raw eye frame, uncropped, at the render resolution. Framing, FOV, and the vertical crop happen in post; recording the whole eye keeps that latitude. The per-frame HMD pose and projection go to a sidecar so post can also do a rotation-only reprojection stabilisation, which is a homography against ground truth rather than an optical-flow estimate.
- Capture is in-process. OBS against the desktop mirror is assumed not to work well enough, so it is not the fallback. The payload reads the eye back, converts it on the GPU, and streams raw frames to a host-side ffmpeg, which encodes, segments into 10-minute chunks, and muxes audio.
- Frames are recorded at the source rate (the Bigscreen Beyond at 72 Hz), with a timestamp per frame. Any temporal rescaling is decided later in post, not at capture time.
- Audio comes from the host side first (the game's pipewire stream), synced by a shared start marker. An in-process FMOD capture DSP is the fallback if drift or sync turns out unacceptable.
- No third smoothed spectator camera. The stabilisation sidecar covers the same need at no GPU cost.

## Todo

### Directing tools without RE

- [x] Time control in the debug menu: a time-scale slider and a pause toggle, with hotkeys. The rate goes through the slow-motion manager's normal game speed rather than the clock field, which that manager rewrites every frame.
- [x] Spawner: a tag field, presets (car, bike, boat, helicopter, jet, tank, soldier, barrel), a distance slider, and a "face the player" toggle. Fuel tanks are world geometry, not spawn definitions, so there is no preset for them; the mech is DLC content the shipped tables do not list.
- [x] Spawn definition picker: the shipped `settings/spawn_*_defs.bin` tables, extracted with `tools/spawn_defs/extract_spawn_defs.py` and embedded, listed by pool with a text filter. A definition's full tag set selects it outright.
- [x] Cinematic mode: one toggle that hides the HUD quad, the floating panel, and the overlays together, with a hotkey. With the HUD redirect off (flatscreen) the game's own HUD stays.
- [x] Take markers: a hotkey and a labelled button that append a line with the wallclock and the game-run time to `markers.jsonl` in the session folder. The capture path is to fold these into its sidecar.
- [x] Bindable keymap: `director_keymap.json` beside the DLL, defaulting to the numeric keypad, fed from the window procedure. Controller chords are not bound yet; the in-headset panel covers that case.

### Directing tools that need RE

- [x] Teleport with bookmarks: the full world matrix saved under a name to `director_bookmarks.json`, jumped to through the game world's instant teleport within 400 m and its streamed teleport beyond, plus a coordinate entry.
- [x] Heat: set, clear, lock, and cap through the heat manager, with the lock and cap re-asserted every frame.
- [x] Actor safety: god mode and unlimited ammo through the character's own setters, re-asserted every frame. Unlimited tethers and vehicle damage are not covered; the tether count is an upgrade-system value and vehicle invulnerability lives on the vehicle's damageable, both still to map.

### Capture

- [ ] Frame source: after both eyes are captured, copy the chosen eye into a ring of three staging textures and map them a few frames late, so the immediate context never stalls. The screenshot module's notes on the render-thread critical section apply.
- [ ] GPU conversion: a pixel shader that writes the eye as NV12 (or a planar format ffmpeg reads directly) before the readback, so the bytes crossing the readback and the socket are a third of BGRA. Compiled with `shadergen` like the other payload shaders.
- [ ] Transport: a loopback TCP connection from the payload to a host-side listener. Wine cannot sensibly spawn a host process, so a script on the host starts ffmpeg listening and the payload connects. Frame header carries the frame index, the wallclock, and the game-time timestamp.
- [ ] Host script: ffmpeg reading raw frames from the socket, encoding with the hardware encoder available, segmenting at 10 minutes, and recording audio from the game's pipewire stream into the same segments. Emits a start marker both sides log for sync.
- [ ] Pose sidecar: per frame, the HMD pose, the per-eye projection (tangents), and the eye resolution, written alongside the video so post can crop, adjust FOV, and stabilise from ground truth.
- [ ] Debug menu and hotkey: start and stop recording, which eye, current segment and dropped-frame count, and an indicator visible in the headset.
- [ ] Post tool: a small host tool that takes a segment plus its sidecar and produces a cropped, FOV-adjusted, optionally stabilised flat video. Can be an ffmpeg filter chain driven from the sidecar or a tiny standalone program; decide when the sidecar format exists.
- [ ] Audio fallback, only if host-side audio fails: an FMOD capture DSP on the master channel group streaming PCM with the video's timestamps. FMOD Studio is the game's audio engine (`fmod_studio_F.dll`).

## Shot list this serves

From the draft script, the tools each shot needs:

- Cliff at golden hour, twice: time of day, weather pinned sunny, teleport bookmark, cinematic mode.
- Coast road convertible, bike, boat, tank: spawner, heat lock at zero.
- Wingsuit into chute into grapple onto a moving car: spawner for the car, actor safety, slow motion for the landing.
- Base assault, fuel tanks, chopper, sniper tower, gravity gun: spawner (fuel tanks, mech), heat lock, slow motion, actor safety.
- Jet barrel roll and wing walk: spawner, actor safety.
- UI beat: cinematic mode off for two seconds.

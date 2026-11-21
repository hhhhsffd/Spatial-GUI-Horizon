# Unified `.hmod` migration (0.2.5)

This project follows the November 20 [Horizon mod format](https://github.com/horizon-rust/horizon/blob/0a875da91dbb0fdcb7ba8c42c11341aa229af51a/docs/modding/src/reference/manifest.md).

- `horizon-mod.toml` is the sole mod manifest and owns identity, metadata and the native pins.
- `[[native.module]]` and `[[native.mixin]]` replace the old tool declarations.
- All payload source paths start with `src/`. Root Cargo files and a native hook factory are included.
- The archive is flat; `horizon-native.json`, hand-written tool manifests and nested mod-id folders are absent.
- Source, README, MIT license and notices are packaged with deterministic paths, timestamps and checksum.

## Package compatibility and build compatibility

The pinned official `cargo horizon-mod check <file> --json` validates the archive as a native mod. That check validates package structure, metadata and payloads; it does not compile native-route declarations.

Horizon's new native route compiles a function-replacement library against a prebuilt game. Its support-module generator only accepts files in `horizon_client`. Spatial GUI requires these three modules in `horizon_client_render`:

| Row | Destination | Why it is needed |
| --- | --- | --- |
| `settings_ui` | `crates/horizon-client-render/src/options_ui.rs` | Exact settings-layout hit tests |
| `geometry` | `crates/horizon-client-render/src/hud.rs` | Shared panel geometry |
| `compositor` | `crates/horizon-client-render/src/hud.rs` | Native GUI textures, world depth, player masks and GPU pipelines |

The documented refusal for unsupported destinations is `E_MIXIN_MODULE_DESTINATION`. A probe on November 18 reached an invalid mirror and failed with `E_MIXIN_SOURCE: ... no module horizon_client::hud`. The November 20 generator still inserts support modules into the `horizon_client` mirror. Renderer function targets may also lack replacement slots. Moving declarations into the game crate does not move the private renderer types or install the missing renderer changes. The factory alone supplies no spatial rendering.

The restriction is explicit in [Support modules](https://github.com/horizon-rust/horizon/blob/0a875da91dbb0fdcb7ba8c42c11341aa229af51a/docs/modding/src/concepts/native-route/5-support-modules.md) and implemented in `horizon-mod-api-gen/src/patch.rs::support_module` in that build. A compatible official launcher build needs renderer support-module destinations and suitable renderer replacement targets or an equivalent GPU integration API. They are not provided by this mod release.

## Working build paths

1. On Windows x64, import the precompiled game ZIP from this release. It contains the full mod and retains the existing native configuration/editor UI.
2. For development, clone the repository and use `tools/port.py build` with the pinned Horizon source and horizon-mixins tool. This reads the unified declarations, writes the older tool's staging manifest only under `target/generated-mixins/`, and compiles the transformed whole game.
3. To rebuild or relink the distributed executable offline, use the same release's corresponding-source archive and `rebuild.py`.

The `.hmod` is a migrated source distribution, **not a working install through the unmodified launcher's native Build on this pin**. The release preserves the full behavior instead of dropping the renderer to make the new route accept a reduced mod. It neither patches the installed launcher nor overwrites player resources.

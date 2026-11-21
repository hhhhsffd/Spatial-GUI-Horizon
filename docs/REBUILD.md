# Rebuilding the Spatial GUI 0.2.5 Windows game

This archive is the complete corresponding source for the optional Windows x64 game ZIP.
It includes the exact transformed Horizon source (`horizon/`) and locked dependency source
(`vendor/`), including licenses. Horizon is LGPL-3.0-or-later; Spatial GUI's original code is MIT.
The normal `.hmod` remains source-only and does not contain Horizon.

Base: Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`, Mixins
`e59da9094f0c2fdda6c627fe9eff22dec6a906f0`, Minecraft data version 26.3.
Source preparation materialized generated module lists and inlined moved creature modules.
The mod's 52 injections and 11 support modules are already applied; no mixin tool is needed
to rebuild this archive. Original source and transformation tools are also available at:

- https://github.com/horizon-rust/horizon/tree/0a875da91dbb0fdcb7ba8c42c11341aa229af51a
- https://github.com/hhhhsffd/Spatial-GUI-Horizon/tree/v0.2.5

## Windows GNU rebuild

1. Extract to a short path without spaces (for example `D:\spatial-source`).
2. Install Rust **1.99.1**, target/host **x86_64-pc-windows-gnu**, a MinGW-w64 C linker
   (the published build used w64devkit), and Python 3.11+ for this developer helper.
   Put the Rust toolchain and linker `bin` directories on PATH.
3. Run `python rebuild.py` beside this file. It builds:

   ```text
   cargo build --offline --locked --profile fast -p horizon-official --bin horizon-official
   ```

   The helper adds Rust's bundled GNU libraries to `LIBRARY_PATH` if present, uses `target/`
   for compiler outputs, copies the result to `horizon-client.exe` and appends the documented
   level-zero import-compatibility trailer for older launchers. The trailer does not alter code
   or claim old WASM API support. No private key is required.
4. Import the resulting executable through Horizon's Import Build, on the exact pinned nightly,
   with `--trust-unsigned`. Resources still come from your own Minecraft installation.

Python is only a build helper, never required by the game. You can instead run the Cargo command
in `horizon/` yourself and use a launcher supporting Horizon's new executable identity, without
the legacy trailer. You are free to edit this source and rebuild/relink your modified game.

Dependency sources are local; installing the compiler/linker may need the network. The archive
does not contain a compiler, proprietary SDK, game jar, prepared game data or account credentials.
Build paths/tool versions can affect the exact executable bytes; this is source/relink availability,
not a claim that arbitrary machines produce a bit-identical PE.

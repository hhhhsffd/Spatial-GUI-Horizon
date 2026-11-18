# Spatial GUI 0.2.4 — Windows x64

This is an **unsigned, independently modified Horizon game**, not an official Horizon release.
Spatial GUI's native code is already compiled in. It runs with your account's permissions; use it
only if you trust the source and publisher. Windows x64 only. No Java or Python is needed to play.

1. Install Horizon `nightly-2026-11-18-0b14f8a` and prepare its resources from your own Minecraft
   installation through the Horizon launcher.
2. Extract this ZIP. In the launcher's Versions page, use **Import Build** and pick
   `horizon-client.exe`, using that nightly as its base. Or run in PowerShell:

   ```powershell
   & "D:\horizon\horizon-launcher.exe" install nightly-2026-11-18-0b14f8a
   & "D:\horizon\horizon-launcher.exe" import-build ".\horizon-client.exe" --version nightly-2026-11-18-0b14f8a --trust-unsigned
   ```

   Adjust the launcher path if yours is elsewhere. The import selects the new build. If your
   instance is pinned to another version, choose the new `imported-...` version for that instance.
3. Play. Press **F7** (or use **Options > Mods**), select **Spatial GUI**, and choose **Config**.
   Use **F8** for the visual editor and **Backtick** for profiles.

This bypasses a source-staging bug in the published launcher/mixin tool; it does not repair or
modify your launcher or source caches. Do not press Build expecting the `.hnmod` alone to work
on this nightly. Other native mods are **not** compiled into this build.

The executable carries Horizon's current game identity plus a legacy **level-zero import identity**
for published launchers. That compatibility trailer contains no executable code and does not
promise support for the old WASM mod API. `tools/rebuild.py` reproduces it.

The importer copies only the executable from a picked file/ZIP; the extra `native-mods.json` and
license files in this download are informative and should be kept with your download.

## Source, modification and licensing

Spatial GUI's original code is MIT. This combined game includes Horizon under LGPL-3.0-or-later.
See `HORIZON-LICENSE.txt`, `GPL-3.0.txt` and the third-party notices included here.

The same release provides `spatial_gui-0.2.4-corresponding-source.tar.gz`: the exact transformed
Horizon source, all locked Cargo dependency sources and licenses, and offline rebuild instructions.
You may modify and rebuild it, including relinking with your modified Horizon; no private signing
key or activation is needed. Reverse engineering for debugging modifications is not restricted.

No Minecraft game code, game assets, prepared resources, worlds or account information are included.

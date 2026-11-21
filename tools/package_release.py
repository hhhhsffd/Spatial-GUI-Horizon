#!/usr/bin/env python3
"""Package the tested Windows game and its complete, vendored corresponding source."""
import argparse
import hashlib
import io
import json
from pathlib import Path
import tarfile
import tomllib
import zipfile

from rebuild import IMPORT_MARK

ROOT = Path(__file__).resolve().parents[1]
VERSION = tomllib.loads((ROOT / "horizon-mod.toml").read_text())["version"]
NATIVE = tomllib.loads((ROOT / "horizon-mod.toml").read_text())["native"]
COMPAT = dict(game=NATIVE["game"], horizon_revision=NATIVE["horizon"],
              horizon_mixins_revision=NATIVE["mixins"])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--stage", type=Path, required=True)
    parser.add_argument("--vendor", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    stage = args.stage.resolve()
    if (stage / ".horizon-revision").read_text().strip() != COMPAT["horizon_revision"]:
        parser.error("Staged source has the wrong Horizon revision")
    binary = args.binary.read_bytes()
    if not binary.startswith(b"MZ") or b"horizon-game-executable;" not in binary:
        parser.error("Expected the compiled Windows Horizon game")
    binary += IMPORT_MARK
    metadata = json.dumps({
        "horizon": COMPAT["horizon_revision"],
        "mods": [{"id": "spatial_gui", "version": VERSION}],
    }, indent=2) + "\n"
    source_name = f"spatial_gui-{VERSION}-corresponding-source.tar.gz"
    notices = (ROOT / "THIRD_PARTY_NOTICES.md").read_bytes()
    with zipfile.ZipFile(args.out / f"spatial_gui-{VERSION}-windows-x64.zip", "w",
                         compression=zipfile.ZIP_DEFLATED) as archive:
        for name, data in {
            "horizon-client.exe": binary,
            "native-mods.json": metadata.encode(),
            "SPATIAL-GUI-LICENSE.txt": (ROOT / "LICENSE").read_bytes(),
            "HORIZON-LICENSE.txt": (stage / "LICENSE").read_bytes(),
            "GPL-3.0.txt": (stage / "LICENSES/GPL-3.0.txt").read_bytes(),
            "HORIZON-THIRD-PARTY-NOTICES.md": (stage / "THIRD_PARTY_NOTICES.md").read_bytes(),
            "THIRD_PARTY_NOTICES.md": notices,
            "INSTALL.md": (ROOT / "docs/INSTALL-PREBUILT.md").read_bytes(),
            "SOURCE.txt": (
                f"Complete corresponding source: {source_name}\n"
                f"https://github.com/hhhhsffd/Spatial-GUI-Horizon/releases/tag/v{VERSION}\n"
                "You may modify, rebuild and relink this game; reverse engineering for "
                "debugging modifications is not restricted. No signing key is needed to "
                "import a modified build with --trust-unsigned.\n"
            ).encode(),
        }.items():
            archive.writestr(name, data)
        for tree, label in [(args.vendor, "dependencies"), (stage / "third_party", "horizon-third-party")]:
            for path in sorted(tree.rglob("*")):
                if path.is_file() and path.name.upper().startswith(
                        ("LICENSE", "LICENCE", "COPYING", "COPYRIGHT", "NOTICE", "AUTHORS")):
                    archive.writestr("licenses/" + label + "/" + path.relative_to(tree).as_posix(),
                                     path.read_bytes())
    # Stage holds public Horizon source with the mod's transformations already applied.
    # Never include player resources, build outputs, caches, or per-machine stage metadata.
    excluded = {".git", ".local", "target", "node_modules", "__pycache__"}
    prefix = f"spatial_gui-{VERSION}-source"
    with tarfile.open(args.out / source_name, "w:gz") as archive:
        def add_bytes(name, data):
            item = tarfile.TarInfo(f"{prefix}/{name}")
            item.size = len(data)
            item.mode = 0o644
            archive.addfile(item, io.BytesIO(data))
        count = 0
        for path in sorted(stage.rglob("*")):
            rel = path.relative_to(stage)
            if any(part in excluded for part in rel.parts):
                continue
            if rel.parts[0].startswith(".horizon") or rel.as_posix() == "native-post-build.json":
                continue
            if path.is_symlink():
                raise SystemExit(f"Unexpected source link: {path}")
            if path.is_file():
                if path.name in {"horizon-data.bin", "horizon-resources.json"}:
                    raise SystemExit(f"Prepared game data must not be distributed: {path}")
                data = path.read_bytes()
                if rel.as_posix() == ".cargo/config.toml":
                    data += b'\n[source.crates-io]\nreplace-with = "vendored-sources"\n[source.vendored-sources]\ndirectory = "../vendor"\n'
                add_bytes("horizon/" + rel.as_posix(), data)
                count += 1
        for path in sorted(args.vendor.rglob("*")):
            if path.is_symlink():
                raise SystemExit(f"Unexpected vendor link: {path}")
            if path.is_file():
                add_bytes("vendor/" + path.relative_to(args.vendor).as_posix(), path.read_bytes())
                count += 1
        add_bytes("rebuild.py", (ROOT / "tools/rebuild.py").read_bytes())
        add_bytes("REBUILD.md", (ROOT / "docs/REBUILD.md").read_bytes())
        add_bytes("SPATIAL-GUI-LICENSE.txt", (ROOT / "LICENSE").read_bytes())
        add_bytes("THIRD_PARTY_NOTICES.md", notices)
    print(f"Included {count} source/dependency files.")
    for path in sorted(args.out.glob(f"spatial_gui-{VERSION}-*")):
        if path.suffix == ".sha256":
            continue
        with path.open("rb") as file:
            sha = hashlib.file_digest(file, "sha256").hexdigest()
        path.with_name(path.name + ".sha256").write_text(f"{sha}  {path.name}\n")
        print(f"{path.name}: {path.stat().st_size} bytes, SHA-256 {sha}")


if __name__ == "__main__":
    main()

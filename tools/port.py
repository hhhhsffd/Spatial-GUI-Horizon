#!/usr/bin/env python3
"""Test, stage/check/build against Horizon, or deterministically package the .hnmod."""
import argparse
import filecmp
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tomllib
import uuid
import zipfile

ROOT = Path(__file__).resolve().parents[1]
COMPAT = json.loads((ROOT / "compatibility.json").read_text())


def run(command, **kwargs):
    print("Running:", " ".join(map(str, command)), flush=True)
    subprocess.run(list(map(str, command)), check=True, **kwargs)


def sync_stage(fresh, stage):
    """Keep unchanged mtimes for Cargo; Windows cannot atomically replace a stage."""
    allowed = (ROOT / "target").resolve()
    for path in (fresh, stage):
        if not path.resolve().is_relative_to(allowed) or path.resolve() == allowed:
            raise SystemExit(f"Stage must be inside {allowed}: {path}")
    for path in fresh.rglob("*"):
        dest = stage / path.relative_to(fresh)
        if path.is_dir():
            dest.mkdir(parents=True, exist_ok=True)
        elif not dest.is_file() or not filecmp.cmp(path, dest, shallow=False):
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(path, dest)
    for path in sorted(stage.rglob("*"), reverse=True):
        if not (fresh / path.relative_to(stage)).exists():
            if path.is_dir():
                path.rmdir()
            else:
                path.unlink()
    shutil.rmtree(fresh)


def package():
    bundle = ROOT / "mixins/spatial_gui"
    manifest = tomllib.loads((bundle / "horizon-mixin.toml").read_text())
    version = manifest["mod"]["version"]
    assert manifest["compatibility"]["horizon_revision"] == COMPAT["horizon_revision"]
    # Deliberately omit `hooks`: private integration needs this exact revision.
    metadata = dict(schema=1, id="spatial_gui", version=version, game=COMPAT["game"],
                    horizon=COMPAT["horizon_revision"], mixins=COMPAT["horizon_mixins_revision"])
    files = {"horizon-native.json": (json.dumps(metadata, indent=2, sort_keys=True) + "\n").encode()}
    for path in sorted(bundle.rglob("*")):
        if path.is_file() and path.suffix in (".toml", ".rs"):
            files[f"mixins/spatial_gui/{path.relative_to(bundle).as_posix()}"] = path.read_bytes()
    files["mixins/spatial_gui/LICENSE"] = (ROOT / "LICENSE").read_bytes()
    out = ROOT / f"dist/spatial_gui-{version}.hnmod"
    out.parent.mkdir(exist_ok=True)
    with zipfile.ZipFile(out, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        for name, data in sorted(files.items()):
            info = zipfile.ZipInfo(name, (2026, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            archive.writestr(info, data)
    with zipfile.ZipFile(out) as archive:
        assert archive.testzip() is None
        assert json.loads(archive.read("horizon-native.json")) == metadata
        for entry in manifest["module"] + manifest["injection"]:
            name = entry.get("source", entry.get("helper"))
            assert f"mixins/spatial_gui/{name}" in archive.namelist()
    digest = hashlib.sha256(out.read_bytes()).hexdigest()
    out.with_suffix(".hnmod.sha256").write_text(f"{digest}  {out.name}\n")
    print(json.dumps(dict(file=str(out), size=out.stat().st_size, sha256=digest), indent=2))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=["test", "plan", "check", "build", "package"])
    parser.add_argument("--horizon-source", type=Path)
    parser.add_argument("--mixins", type=Path)
    parser.add_argument("--target-dir", type=Path)
    parser.add_argument("--profile", default="fast", choices=["dev", "fast", "release"])
    args = parser.parse_args()
    if args.action == "package":
        package()
        return
    env = dict(os.environ)
    appdata = Path(os.environ.get("APPDATA", Path.home() / ".local/share"))
    launcher = appdata / "horizon-launcher"
    rust = launcher / "rust/1.99.1-x86_64-pc-windows-gnu/bin"
    if not shutil.which("cargo", path=env.get("PATH")) and rust.is_dir():
        env["PATH"] = str(rust) + os.pathsep + env.get("PATH", "")
    cargo = shutil.which("cargo", path=env.get("PATH"))
    if not cargo:
        parser.error("Install Rust or add cargo to PATH")
    if os.name == "nt":
        rustc = str(Path(cargo).with_name("rustc.exe"))
        sysroot = Path(subprocess.check_output([rustc, "--print", "sysroot"], text=True, env=env).strip())
        libs = sysroot / "lib/rustlib/x86_64-pc-windows-gnu/lib/self-contained"
        if libs.is_dir():
            env["LIBRARY_PATH"] = str(libs) + os.pathsep + env.get("LIBRARY_PATH", "")
    if args.action == "test":
        run([cargo, "test", "--locked"], cwd=ROOT, env=env)
        return
    source = args.horizon_source or launcher / "native/source" / COMPAT["horizon_revision"]
    if not (source / "Cargo.toml").is_file():
        parser.error("Pass --horizon-source PATH to the pinned Horizon source")
    source = source.resolve()
    revision_file = source / ".horizon-revision"
    if revision_file.is_file():
        assert revision_file.read_text().strip() == COMPAT["horizon_revision"], "Wrong Horizon revision"
    mixins = args.mixins or launcher / "native/tools" / COMPAT["horizon_mixins_revision"][:12] / "bin/horizon-mixins.exe"
    if not mixins.is_file():
        found = shutil.which("horizon-mixins")
        if not found:
            parser.error("Pass --mixins PATH to horizon-mixins")
        mixins = Path(found)
    target = ROOT / "target"
    target.mkdir(exist_ok=True)
    if args.action == "plan":
        run([mixins, "plan", "--workspace", source, "--mods", ROOT / "mixins", "--out", target / "plan.json"], env=env)
        return
    stage = target / "horizon-stage"
    fresh = target / f"horizon-stage-new-{uuid.uuid4().hex}"
    run([mixins, "apply", "--workspace", source, "--mods", ROOT / "mixins", "--out", fresh], env=env)
    sync_stage(fresh, stage)
    env["CARGO_TARGET_DIR"] = str((args.target_dir or target / "horizon-cargo").resolve())
    run([cargo, args.action, "--locked", "--profile", args.profile,
         "-p", "horizon-official", "--bin", "horizon-official"], cwd=stage, env=env)


if __name__ == "__main__":
    main()

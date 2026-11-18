#!/usr/bin/env python3
"""Test, stage/check/build against Horizon, or deterministically package the unified .hmod."""
import argparse
import filecmp
import hashlib
import json
import os
import re
from pathlib import Path
import shutil
import subprocess
import tomllib
import uuid
import zipfile

ROOT = Path(__file__).resolve().parents[1]
NATIVE = tomllib.loads((ROOT / "horizon-mod.toml").read_text())["native"]
COMPAT = dict(game=NATIVE["game"], horizon_revision=NATIVE["horizon"],
              horizon_mixins_revision=NATIVE["mixins"])


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



def prepare_source(source, target, cargo, env):
    """Give the legacy mixin tool a disposable, ordinary-module source view.

    Horizon's launcher materializes OUT_DIR module lists too. Its current
    November 18 source also has moved creature modules whose parent paths the
    pinned mixin tool rejects. Inlining those modules is a developer-build
    workaround; it does not make unmodified launchers accept the bundle.
    """
    game = source / "crates/horizon-client/src/game.rs"
    if '"/game_modules.rs"' not in game.read_text(encoding="utf-8"):
        return source
    prepared = target / f"horizon-input-{COMPAT['horizon_revision'][:7]}"
    for path in source.rglob("*"):
        if path.is_symlink() and not path.resolve().is_relative_to(source):
            raise SystemExit(f"Source link leaves its checkout: {path}")
    shutil.copytree(source, prepared, dirs_exist_ok=True,
                    ignore=shutil.ignore_patterns(".git", "target", ".local", "node_modules"))
    helper = target / "source-prep"
    (helper / "src").mkdir(parents=True, exist_ok=True)
    dependency = (source / "crates/horizon-mod-api-gen").as_posix()
    manifest = ('[package]\nname="spatial-source-prep"\nversion="0.0.0"\nedition="2024"\n'
                '[workspace]\n[dependencies]\nhorizon-mod-api-gen = { path = '
                + json.dumps(dependency) + ' }\n')
    (helper / "Cargo.toml").write_text(manifest, encoding="utf-8")
    shutil.copy2(ROOT / "tools/prepare_source.rs", helper / "src/main.rs")
    run([cargo, "build", "--release", "--manifest-path", helper / "Cargo.toml"], env=env)
    binary = Path(env["CARGO_TARGET_DIR"]) / "release" / ("spatial-source-prep.exe" if os.name == "nt" else "spatial-source-prep")
    subprocess.run([str(binary), str(prepared)], check=True, env=env, stdout=subprocess.DEVNULL)
    # This one area was physically moved while its logical Rust module names
    # stayed the same. Its externally declared files contain no further
    # external module declarations, so this preserves their module scopes.
    creatures = prepared / "crates/horizon-client/src/game/mobs/creatures/mod.rs"
    if creatures.is_file():
        pattern = r'#\[path\s*=\s*"([^"\n]+)"\]\s*((?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*);'
        def inline(match):
            path = (creatures.parent / match[1]).resolve()
            if not path.is_relative_to(prepared.resolve()):
                raise SystemExit(f"Module leaves prepared source: {path}")
            body = path.read_text(encoding="utf-8")
            if re.search(r'^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;', body, re.MULTILINE):
                raise SystemExit(f"Moved module needs recursive source preparation: {path}")
            body = re.sub(r'^//!([^\n]*)', r'//\1', body, flags=re.MULTILINE)
            return match[2] + " {\n" + body + "\n}"
        creatures.write_text(re.sub(pattern, inline, creatures.read_text(encoding="utf-8")), encoding="utf-8")
    return prepared


def package():
    """Pack the flat source project; no generated legacy manifests enter the archive."""
    manifest = tomllib.loads((ROOT / "horizon-mod.toml").read_text())
    version = manifest["version"]
    assert tomllib.loads((ROOT / "Cargo.toml").read_text())["package"]["version"] == version
    files = {}
    for name in ("horizon-mod.toml", "Cargo.toml", "Cargo.lock", "LICENSE", "README.md"):
        files[name] = (ROOT / name).read_bytes()
    files["NOTICE-THIRD-PARTY.md"] = (ROOT / "THIRD_PARTY_NOTICES.md").read_bytes()
    for path in sorted((ROOT / "src").rglob("*")):
        if path.is_file() and path.suffix in (".toml", ".rs"):
            files[path.relative_to(ROOT).as_posix()] = path.read_bytes()
    for entry in manifest["native"]["module"] + manifest["native"]["mixin"]:
        assert entry.get("source", entry.get("helper")) in files
    out = ROOT / f"dist/spatial_gui-{version}.hmod"
    out.parent.mkdir(exist_ok=True)
    with zipfile.ZipFile(out, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        for name, data in sorted(files.items()):
            info = zipfile.ZipInfo(name, (2026, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            archive.writestr(info, data)
    with zipfile.ZipFile(out) as archive:
        assert archive.testzip() is None
        assert tomllib.loads(archive.read("horizon-mod.toml").decode()) == manifest
    digest = hashlib.sha256(out.read_bytes()).hexdigest()
    out.with_suffix(".hmod.sha256").write_text(f"{digest}  {out.name}\n")
    print(json.dumps(dict(file=str(out), size=out.stat().st_size, sha256=digest), indent=2))


def legacy_stage(target):
    """Translate the unified manifest only for a disposable full-source developer build.

    The published launcher's function-replacement route cannot host this mod's
    renderer support modules. This deliberately uses the original source compiler.
    """
    text = (ROOT / "horizon-mod.toml").read_text()
    manifest = tomllib.loads(text)
    header = (f'[mod]\nid = "{manifest["id"]}"\nversion = "{manifest["version"]}"\npriority = 100\n'
              f'\n[compatibility]\nhorizon_revision = "{manifest["native"]["horizon"]}"\n')
    rows = text[text.index("[[native.module]]"):]
    rows = rows.replace("[[native.module]]", "[[module]]").replace("[[native.mixin]]", "[[injection]]")
    mods = target / "generated-mixins"
    bundle = mods / manifest["id"]
    bundle.mkdir(parents=True, exist_ok=True)
    (bundle / "horizon-mixin.toml").write_text(header + rows, encoding="utf-8")
    shutil.copytree(ROOT / "src", bundle / "src", dirs_exist_ok=True)
    return mods


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
    env["CARGO_TARGET_DIR"] = str((args.target_dir or target / "horizon-cargo").resolve())
    source = prepare_source(source, target, cargo, env)
    mods = legacy_stage(target)
    if args.action == "plan":
        run([mixins, "plan", "--workspace", source, "--mods", mods, "--out", target / "plan.json"], env=env)
        return
    stage = target / "horizon-stage"
    fresh = target / f"horizon-stage-new-{uuid.uuid4().hex}"
    run([mixins, "apply", "--workspace", source, "--mods", mods, "--out", fresh], env=env)
    sync_stage(fresh, stage)
    run([cargo, args.action, "--locked", "--profile", args.profile,
         "-p", "horizon-official", "--bin", "horizon-official"], cwd=stage, env=env)


if __name__ == "__main__":
    main()

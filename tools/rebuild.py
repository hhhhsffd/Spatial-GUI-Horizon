#!/usr/bin/env python3
"""Rebuild the optional Windows game from an extracted corresponding-source archive."""
import os
from pathlib import Path
import shutil
import subprocess

IMPORT_MARK = b"\0Spatial GUI legacy launcher import identity only\0horizon-mod-api-level:0000;\0"


def main():
    root = Path(__file__).resolve().parent
    if not (root / "horizon/Cargo.toml").is_file():
        raise SystemExit("Run rebuild.py from the extracted corresponding-source release archive.")
    cargo = shutil.which("cargo")
    rustc = shutil.which("rustc")
    if not cargo or not rustc:
        raise SystemExit("Add Rust 1.99.1 and the platform C linker to PATH first.")
    env = dict(os.environ)
    env["CARGO_TARGET_DIR"] = str(root / "target")
    if os.name == "nt":
        sysroot = Path(subprocess.check_output([rustc, "--print", "sysroot"], text=True).strip())
        libs = sysroot / "lib/rustlib/x86_64-pc-windows-gnu/lib/self-contained"
        if libs.is_dir():
            env["LIBRARY_PATH"] = str(libs) + os.pathsep + env.get("LIBRARY_PATH", "")
    subprocess.run([cargo, "build", "--offline", "--locked", "--profile", "fast",
                    "-p", "horizon-official", "--bin", "horizon-official"],
                   cwd=root / "horizon", env=env, check=True)
    suffix = ".exe" if os.name == "nt" else ""
    output = root / ("horizon-client" + suffix)
    shutil.copyfile(root / ("target/fast/horizon-official" + suffix), output)
    with output.open("ab") as file:
        file.write(IMPORT_MARK)
    output.chmod(output.stat().st_mode | 0o111)
    print(f"Built {output}. Import it on the pinned Horizon nightly; see REBUILD.md.")


if __name__ == "__main__":
    main()

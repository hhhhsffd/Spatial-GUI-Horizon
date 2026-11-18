"""Exercise release packaging with a synthetic game and source/dependency tree."""
import contextlib
import io
import json
import os
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest.mock import patch
import zipfile

import package_release


class ReleasePackaging(unittest.TestCase):
    def test_importable_game_and_relocatable_source_exclude_player_data(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            def put(name, content):
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(content)
                return path
            binary = put("game.exe", b"MZcompiled-game\0horizon-game-executable;")
            put("source/.horizon-revision", b"0123456789abcdef")
            put("source/.cargo/config.toml", b"[alias]\n")
            put("source/Cargo.toml", b"[workspace]\n")
            put("source/LICENSE", b"Horizon LGPL license")
            put("source/LICENSES/GPL-3.0.txt", b"GPL text")
            put("source/THIRD_PARTY_NOTICES.md", b"Horizon notices")
            put("source/third_party/example/LICENSE", b"Local dependency license")
            put("source/.local/horizon-client/26.3/horizon-resources.json", b"private-data")
            put("source/target/build-output.exe", b"private-build")
            put("source/.horizon-mixins-stage.json", b"local-stage-metadata")
            put("vendor/example-1.0/src/lib.rs", b"pub fn example() {}")
            license_file = put("vendor/example-1.0/LICENSE-MIT", b"Dependency license")
            # Cargo's source archives can carry timestamps older than ZIP's 1980 epoch.
            os.utime(license_file, (1, 1))
            put("LICENSE", b"Spatial GUI MIT license")
            put("THIRD_PARTY_NOTICES.md", b"Mod notices")
            put("docs/INSTALL-PREBUILT.md", b"Installation instructions")
            put("docs/REBUILD.md", b"Rebuild instructions")
            put("tools/rebuild.py", b"# Developer rebuild helper\n")
            args = ["package_release.py", "--binary", str(binary), "--stage",
                    str(root / "source"), "--vendor", str(root / "vendor"),
                    "--out", str(root / "release")]
            with patch.object(package_release, "ROOT", root), \
                    patch.object(package_release, "VERSION", "0.2.4"), \
                    patch.object(package_release, "COMPAT", {"horizon_revision": "0123456789abcdef"}), \
                    patch("sys.argv", args), contextlib.redirect_stdout(io.StringIO()):
                package_release.main()
            with zipfile.ZipFile(root / "release/spatial_gui-0.2.4-windows-x64.zip") as archive:
                game = archive.read("horizon-client.exe")
                self.assertTrue(game.startswith(binary.read_bytes()))
                self.assertTrue(game.endswith(b"horizon-mod-api-level:0000;\0"))
                self.assertEqual(json.loads(archive.read("native-mods.json"))["mods"],
                                 [{"id": "spatial_gui", "version": "0.2.4"}])
                self.assertEqual(archive.read("licenses/dependencies/example-1.0/LICENSE-MIT"),
                                 b"Dependency license")
                self.assertEqual(archive.read("licenses/horizon-third-party/example/LICENSE"),
                                 b"Local dependency license")
            with tarfile.open(root / "release/spatial_gui-0.2.4-corresponding-source.tar.gz") as archive:
                names = archive.getnames()
                self.assertFalse(any("/.local/" in p or "/target/" in p or ".horizon-" in p for p in names))
                prefix = "spatial_gui-0.2.4-source/"
                config = archive.extractfile(prefix + "horizon/.cargo/config.toml").read()
                self.assertIn(b'directory = "../vendor"', config)
                self.assertEqual(archive.extractfile(prefix + "vendor/example-1.0/src/lib.rs").read(),
                                 b"pub fn example() {}")
                self.assertEqual(archive.extractfile(prefix + "rebuild.py").read(),
                                 b"# Developer rebuild helper\n")


if __name__ == "__main__":
    unittest.main()

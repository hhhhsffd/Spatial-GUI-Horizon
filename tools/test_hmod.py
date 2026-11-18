"""Regression checks for the unified package and the full-source build adapter."""
import contextlib
import hashlib
import io
from pathlib import Path
import shutil
import tempfile
import tomllib
import unittest
from unittest.mock import patch
import zipfile

import port


class UnifiedPackage(unittest.TestCase):
    def test_flat_archive_is_deterministic_complete_and_has_legal_notices(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            for name in ("horizon-mod.toml", "Cargo.toml", "Cargo.lock", "LICENSE", "README.md", "THIRD_PARTY_NOTICES.md"):
                shutil.copy2(port.ROOT / name, root / name)
            shutil.copytree(port.ROOT / "src", root / "src")
            with patch.object(port, "ROOT", root), contextlib.redirect_stdout(io.StringIO()):
                port.package()
                archive = next((root / "dist").glob("*.hmod"))
                first = archive.read_bytes()
                port.package()
                self.assertEqual(first, archive.read_bytes())
            self.assertTrue(archive.with_suffix(".hmod.sha256").read_text().startswith(hashlib.sha256(first).hexdigest()))
            with zipfile.ZipFile(archive) as bundle:
                names = bundle.namelist()
                self.assertIsNone(bundle.testzip())
                self.assertEqual(bundle.read("LICENSE"), (root / "LICENSE").read_bytes())
                self.assertEqual(bundle.read("NOTICE-THIRD-PARTY.md"), (root / "THIRD_PARTY_NOTICES.md").read_bytes())
                self.assertFalse(any(n.startswith(("mixins/", ".horizon/", "target/")) or n.endswith(("horizon-native.json", "horizon-mixin.toml")) for n in names))
                manifest = tomllib.loads(bundle.read("horizon-mod.toml").decode())
                self.assertEqual(manifest["id"], "spatial_gui")
                for table, key in (("module", "source"), ("mixin", "helper")):
                    for row in manifest["native"][table]:
                        self.assertTrue(row[key].startswith("src/"))
                        self.assertEqual(bundle.read(row[key]), (root / row[key]).read_bytes())
                self.assertIn("src/lib.rs", names)
                self.assertIn(b"#[mod_api::horizon_mod]", bundle.read("src/lib.rs"))

    def test_source_build_adapter_keeps_every_native_row_and_binding(self):
        manifest = tomllib.loads((port.ROOT / "horizon-mod.toml").read_text())
        with tempfile.TemporaryDirectory() as temp:
            mods = port.legacy_stage(Path(temp))
            bundle = mods / manifest["id"]
            old = tomllib.loads((bundle / "horizon-mixin.toml").read_text())
            self.assertEqual(old["mod"]["version"], manifest["version"])
            self.assertEqual(old["compatibility"]["horizon_revision"], manifest["native"]["horizon"])
            self.assertEqual(old["module"], manifest["native"]["module"])
            self.assertEqual(old["injection"], manifest["native"]["mixin"])
            for row in old["module"] + old["injection"]:
                name = row.get("source", row.get("helper"))
                self.assertEqual((bundle / name).read_bytes(), (port.ROOT / name).read_bytes())


if __name__ == "__main__":
    unittest.main()

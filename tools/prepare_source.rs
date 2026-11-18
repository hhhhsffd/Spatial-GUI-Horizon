// Run only on the build helper's disposable copy of the pinned source.
fn main() {
    let source = std::env::args().nth(1).expect("prepared source path");
    horizon_mod_api_gen::materialize_game_modules(std::path::Path::new(&source))
        .expect("materializing Horizon's generated module lists");
}

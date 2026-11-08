fn spatial_hand(game: &crate::game::Game<'_>, stack: Option<&horizon_client::items::ItemStack>, canceled: &mut bool) {
    crate::game::spatial_integration::hide_hand(game,stack,canceled);
}

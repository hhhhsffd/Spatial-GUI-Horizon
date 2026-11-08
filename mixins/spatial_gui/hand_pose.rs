fn spatial_hand_pose(game: &crate::game::Game<'_>, hand: horizon_client::first_person::Hand, z: &mut f32, pitch: &mut f32) {
    crate::game::spatial_integration::hand_pose(game,hand,z,pitch);
}

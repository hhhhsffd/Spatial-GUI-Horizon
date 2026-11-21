fn spatial_hand(game: &crate::game::Game<'_>, render: &mut mod_api::HandRender<'_>, canceled: &mut bool) {
    crate::game::spatial_integration::hide_hand(game,render.stack.is_some_and(|s| mod_api::__private::item_key_id(s.item()) == Some("minecraft:shield")),canceled); let hand = if render.hand == mod_api::Hand::Main { horizon_client::first_person::Hand::Main } else { horizon_client::first_person::Hand::Off }; crate::game::spatial_integration::hand_pose(game,hand,&mut render.translation[2],&mut render.rotation[0]);
}

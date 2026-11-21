"""Generate the declarative integration manifest and its small typed adapters."""
from pathlib import Path
import json
import tomllib

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "src/native"
VERSION = tomllib.loads((ROOT / "horizon-mod.toml").read_text())["version"]
NATIVE = tomllib.loads((ROOT / "horizon-mod.toml").read_text())["native"]
COMPAT = dict(game=NATIVE["game"], horizon_revision=NATIVE["horizon"],
              horizon_mixins_revision=NATIVE["mixins"])
parts = [f'''id = "spatial_gui"
name = "Spatial GUI"
version = "{VERSION}"
description = "World-space menus, inverse pointer mapping, profiles and visual editor"
authors = ["Spatial GUI Horizon contributors", "tastytrash (original mod)"]
license = "MIT"
homepage = "https://github.com/hhhhsffd/Spatial-GUI-Horizon"
issues = "https://github.com/hhhhsffd/Spatial-GUI-Horizon/issues"
api = "0.2"
side = "client"
capabilities = ["registries"]

[native]
game = "{COMPAT['game']}"
horizon = "{COMPAT['horizon_revision']}"
mixins = "{COMPAT['horizon_mixins_revision']}"
''']


def module(id, file, name, source, public=False):
    parts.append(f'\n[[native.module]]\nid = "{id}"\nfile = "{file}"\nname = "{name}"\nsource = "src/native/{source}"\n' + ('visibility = "pub"\n' if public else ''))

def hook(id, target, args, body, result=None, cancel=False, kind="HEAD"):
    q = json.dumps
    parts.append(f'\n[[native.mixin]]\nid = "{id}"\nkind = "{kind}"\ntarget = "{target}"\nhelper = "src/native/{id}.rs"\n' + ('cancellable = true\n' if cancel else '') + 'bindings = [\n' + ''.join(f'  {{ name = {q(n)}, ty = {q(t)}, value = {q(v)} }},\n' for n,t,v in args) + ']\n')
    output = f" -> {result}" if result else ""
    (BUNDLE / f"{id}.rs").write_text(f'fn spatial_{id}(' + ', '.join(f'{n}: {t}' for n,t,_ in args) + f'){output} {{\n    {body}\n}}\n')

game_file = "crates/horizon-client/src/game.rs"
hud_file = "crates/horizon-client-render/src/hud.rs"
module('settings_ui', 'crates/horizon-client-render/src/options_ui.rs', 'spatial_settings_input', 'settings_ui.rs')
for id, name, source in [('config','spatial_config','config.rs'), ('editor','spatial_editor','editor.rs'), ('integration','spatial_integration','integration.rs'), ('controls','spatial_controls','controls.rs'), ('compat','spatial_compat','compat.rs')]:
    module(id, game_file, name, source)
module('geometry',hud_file,'spatial_math','geometry.rs',True)
module('avatar',game_file,'spatial_avatar','avatar.rs')
module('motion',game_file,'spatial_motion','motion.rs')
module('surface',game_file,'spatial_surface','surface.rs')
module('compositor',hud_file,'spatial_gui','compositor.rs',True)
G=('game',"&crate::game::Game<'_>",'&*self')
GM=('game',"&mut crate::game::Game<'_>",'&mut *self')
B=('batch','&mut horizon_client::hud::HudBatch','&mut *batch')
I='crate::game::spatial_integration'
C='crate::game::spatial_controls'
base="horizon_client::game::Game<'_>::"
native="horizon_client::game::native_hooks::base::Game<'_>::"
hook('frame',"horizon_client::game::native_hooks::render_frame_pre::Game<'_>::native_render_frame_pre",[GM],f'{C}::flush(game); {I}::frame(game);')
hook('input',base+'handle_window_event_inner',[GM,('event','&mut winit::event::WindowEvent','&mut event')],f'if {C}::input(game, event) {{ core::ops::ControlFlow::Break(()) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<()>',True)
hook('relative',"horizon_client::game::window_handler::<Game<'_> as ApplicationHandler>::device_event",[G,('event','&winit::event::DeviceEvent','&event')],f'{C}::relative(game,event);')
hook('camera',"horizon_client::game::perspective::Game<'_>::camera_view",[GM,('partial','f32','partial')],f'if let Some(camera) = {I}::camera(game,partial) {{ core::ops::ControlFlow::Break(Some(camera)) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<Option<(glam::DVec3,f32,f32)>>',True)
hook('orientation',base+'camera_orientation',[],f'if let Some(rotation) = {I}::orientation() {{ core::ops::ControlFlow::Break(rotation) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<glam::Quat>',True)
hook('pause',"horizon_client::game::pointer_capture::Game<'_>::pauses_game",[G],f'if {I}::pauses(game) {{ core::ops::ControlFlow::Break(true) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<bool>',True)
hook('key',native+'native_screen_key_pressed_pre',[G,('key','&str','key'),('repeat','bool','repeat'),('canceled','&mut bool','&mut *canceled')],f'{C}::screen_key(game,key,repeat,canceled);')
hook('key_release',native+'native_screen_key_released_pre',[G,('key','&str','key')],f'{C}::released(game,key,false);')
hook('begin_hud',base+'hud_layers',[G],f'{I}::begin_hud(game);')
screens=[('', 'inventory','inventory'),('', 'creative','creative'),('options','options','options'),('screens','pause_menu','paused'),('screens','death_screen','dead'),('stats_screen','stats','stats'),('advancements_screen','advancements','advancements'),('lan_screen','world_options','world_options'),('friends','friends_screen','friends'),('social','social_screen','social'),('server_links','server_links','server_links'),('server_links','confirm_link','confirm_link'),('server_links','dialog_warning','dialog_warning'),('command_blocks','command_block','command_block'),('block_editor','block_editor','block_editor'),('debug_options','debug_options','debug_options'),('chat','chat_screen','chat'),('gamemode_switcher','gamemode_switcher','game_mode_switcher'),('books','book','book'),('sign_edit','sign_editor','sign'),('dialogs','dialog','dialog'),('server_packs','pack_prompt','pack_confirm'),('win','win_screen','win'),('code_of_conduct','code_of_conduct','code_of_conduct'),('chat','link_prompt','link_prompt'),('remote_chat','command_prompt','command_prompt')]
for mod,fn,id in screens:
    if fn == 'friends_screen':
        # Feature-gated modules cannot be targeted by declarative mixins.
        # This page uses capture's standard batch.blur_at fallback.
        continue
    if fn == 'creative':
        mod = 'creative_screen'
    if fn == 'inventory':
        mod = 'inventory_slots'
    target=f"horizon_client::game::{mod+'::' if mod else ''}Game<'_>::draw_{fn}"
    if fn == 'options':
        hook('begin_'+fn,target,[GM,B],f'crate::game::spatial_compat::refresh(game); {I}::begin_screen(game,batch,"{id}");')
    else:
        hook('begin_'+fn,target,[G,B],f'{I}::begin_screen(game,batch,"{id}");')
hook('mod_begin',native+'native_screen_render_post',[G,('batch','&mut horizon_client::hud::HudBatch','&mut *draw')],f'{I}::begin_screen(game,batch,"mod");')
hook('screen_end',"horizon_client::game::titles::Game<'_>::draw_saving_indicator",[B],f'{I}::screen_drawn(batch);')
hook('draw',"horizon_client::game::narrator::Game<'_>::draw_narrator_toast",[G,B],f'{I}::capture(game,batch);')
hook('pad_cursor',"horizon_client::game::pad_menu::Game<'_>::draw_pad_cursor",[G,B],f'if {I}::pad_cursor(game,batch) {{ core::ops::ControlFlow::Break(()) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<()>',True)
hook('blur',"horizon_client::game::screen_backgrounds::Game<'_>::menu_blur",[],f'if {I}::suppress_blur() {{ core::ops::ControlFlow::Break(None) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<Option<f32>>',True)
hook('fov',native+'native_compute_fov',[G,('degrees','&mut f32','&mut *degrees')],f'{I}::fov(game,degrees);')
hook('hand',"horizon_client::game::native_hooks::render_hand::Game<'_>::native_render_hand",[G,('render',"&mut mod_api::HandRender<'_>",'&mut *render'),('canceled','&mut bool','&mut *canceled')],f'{I}::hide_hand(game,render.stack.is_some_and(|s| mod_api::__private::item_key_id(s.item()) == Some("minecraft:shield")),canceled); let hand = if render.hand == mod_api::Hand::Main {{ horizon_client::first_person::Hand::Main }} else {{ horizon_client::first_person::Hand::Off }}; {I}::hand_pose(game,hand,&mut render.translation[2],&mut render.rotation[0]);')
hook('hud',native+'native_gui_layer_pre',[G,('layer','&str','layer'),('canceled','&mut bool','&mut *canceled')],f'{I}::hide_hud(game,layer,canceled);')
hook('settings',"horizon_client::game::options::Game<'_>::options_mouse",[('value','Result<()>','$value'),GM], 'crate::game::spatial_compat::refresh(game); value', 'Result<()>',kind='RETURN')
hook('settings_key',"horizon_client::game::options::Game<'_>::options_key",[('value','Result<()>','$value'),GM], 'crate::game::spatial_compat::refresh(game); value', 'Result<()>',kind='RETURN')
hook('list',"horizon_client::game::options::Game<'_>::open_options_at",[('value','()','$value'),GM], 'crate::game::spatial_compat::refresh(game); value','()',kind='RETURN')
hook('setting_label','horizon_client_render::options_ui::mod_setting_label',[('setting','&crate::mod_list::NativeSetting','setting')], 'if setting.key.starts_with("spatial_gui/button/") { core::ops::ControlFlow::Break(crate::text::Text::literal(setting.label.clone())) } else { core::ops::ControlFlow::Continue(()) }','core::ops::ControlFlow<Text>',True)
R='crate::hud::spatial_gui'
hook('gui_batch','horizon_client_render::hud::HudBatch::new',[('requested','u32','$value')],f'{R}::gui_scale(requested)','u32',kind='MODIFY_ARG')
parts[-1] += 'selector = "horizon_client_input::window::GuiSize::new"\nrequire = 1\nordinal = 0\nargument = 1\n'

hook('render_needs','horizon_client_render::render_slots::needs',[('needs','RenderNeeds','$value')],f'let mut needs = needs; {R}::needs(&mut needs); needs','RenderNeeds',kind='RETURN')
hook('world_drawn','horizon_client_render::render_slots::world_drawn',[('world',"&mut crate::render_slots::WorldDrawn<'_>",'&mut *world')],f'{R}::world_drawn(world);')
hook('hand_drawn','horizon_client_render::render_slots::hand_drawn',[('hand',"&mut crate::render_slots::HandDrawn<'_>",'&mut *hand')],f'{R}::hand_drawn(hand);')
hook('render_hooked','horizon_client_render::render_slots::world_drawn_hooked',[],f'if {R}::active() {{ core::ops::ControlFlow::Break(true) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<bool>',True)
hook('render','horizon_client_render::hud::HudRenderer::draw_range',[
    ('renderer','&crate::hud::HudRenderer','&*self'),('encoder','&mut wgpu::CommandEncoder','&mut *encoder'),('target','&wgpu::TextureView','target'),('coverage','Option<&wgpu::TextureView>','coverage'),('depth','&wgpu::TextureView','depth'),('batch','&crate::hud::HudBatch','batch'),('size','[u32;2]','size'),('vertices','&std::ops::Range<u32>','&vertices')],f'if {R}::draw(renderer,encoder,target,coverage,depth,batch,size,vertices.clone()) {{ core::ops::ControlFlow::Break(()) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<()>',True)
(ROOT / 'horizon-mod.toml').write_text(''.join(parts), encoding='utf-8')

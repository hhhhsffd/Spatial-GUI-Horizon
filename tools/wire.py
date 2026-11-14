"""Generate the declarative integration manifest and its small typed adapters."""
from pathlib import Path
import json

BUNDLE = Path(__file__).resolve().parents[1] / "mixins/spatial_gui"
ROOT = BUNDLE.parents[1]
COMPAT = json.loads((ROOT / "compatibility.json").read_text())
parts = [f'''[mod]
id = "spatial_gui"
version = "0.2.3"
priority = 100

[compatibility]
horizon_revision = "{COMPAT['horizon_revision']}"
''']

def module(id, file, name, source, public=False):
    parts.append(f'\n[[module]]\nid = "{id}"\nfile = "{file}"\nname = "{name}"\nsource = "{source}"\n' + ('visibility = "pub"\n' if public else ''))

def hook(id, target, args, body, result=None, cancel=False, kind="HEAD"):
    q = json.dumps
    parts.append(f'\n[[injection]]\nid = "{id}"\nkind = "{kind}"\ntarget = "{target}"\nhelper = "{id}.rs"\n' + ('cancellable = true\n' if cancel else '') + 'bindings = [\n' + ''.join(f'  {{ name = {q(n)}, ty = {q(t)}, value = {q(v)} }},\n' for n,t,v in args) + ']\n')
    output = f" -> {result}" if result else ""
    (BUNDLE / f"{id}.rs").write_text(f'fn spatial_{id}(' + ', '.join(f'{n}: {t}' for n,t,_ in args) + f'){output} {{\n    {body}\n}}\n')

game_file = "crates/horizon-client/src/game.rs"
hud_file = "crates/horizon-client-render/src/hud.rs"
module('settings_ui', 'crates/horizon-client-render/src/options_ui.rs', 'spatial_settings_input', 'settings_ui.rs')
for id, name, source in [('config','spatial_config','config.rs'), ('editor','spatial_editor','editor.rs'), ('integration','spatial_integration','integration.rs'), ('controls','spatial_controls','controls.rs')]:
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
native="horizon_client::game::native_hooks::Game<'_>::"
hook('frame',native+'native_frame',[GM],f'{C}::flush(game); {I}::frame(game);')
hook('input',base+'handle_window_event_inner',[GM,('event','&mut winit::event::WindowEvent','&mut event')],f'if {C}::input(game, event) {{ core::ops::ControlFlow::Break(()) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<()>',True)
hook('relative',"horizon_client::game::window_handler::<Game<'_> as ApplicationHandler>::device_event",[G,('event','&winit::event::DeviceEvent','&event')],f'{C}::relative(game,event);')
hook('camera',"horizon_client::game::perspective::Game<'_>::camera_view",[GM,('partial','f32','partial')],f'if let Some(camera) = {I}::camera(game,partial) {{ core::ops::ControlFlow::Break(Some(camera)) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<Option<(glam::DVec3,f32,f32)>>',True)
hook('orientation',base+'camera_orientation',[],f'if let Some(rotation) = {I}::orientation() {{ core::ops::ControlFlow::Break(rotation) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<glam::Quat>',True)
hook('pause',"horizon_client::game::pointer_capture::Game<'_>::pauses_game",[G],f'if {I}::pauses(game) {{ core::ops::ControlFlow::Break(true) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<bool>',True)
hook('key',native+'native_screen_key_pressed_pre',[G,('key','&str','key'),('repeat','bool','repeat'),('canceled','&mut bool','&mut *canceled')],f'{C}::screen_key(game,key,repeat,canceled);')
hook('key_release',native+'native_key_event',[G,('key','winit::keyboard::KeyCode','key'),('pressed','bool','pressed')],f'{C}::released(game,key,pressed);')
hook('begin_hud',base+'hud_layers',[G],f'{I}::begin_hud(game);')
screens=[('', 'inventory','inventory'),('', 'creative','creative'),('options','options','options'),('screens','pause_menu','paused'),('screens','death_screen','dead'),('stats_screen','stats','stats'),('advancements_screen','advancements','advancements'),('lan_screen','world_options','world_options'),('friends','friends_screen','friends'),('social','social_screen','social'),('server_links','server_links','server_links'),('server_links','confirm_link','confirm_link'),('server_links','dialog_warning','dialog_warning'),('command_blocks','command_block','command_block'),('block_editor','block_editor','block_editor'),('debug_options','debug_options','debug_options'),('chat','chat_screen','chat'),('gamemode_switcher','gamemode_switcher','game_mode_switcher'),('books','book','book'),('sign_edit','sign_editor','sign'),('dialogs','dialog','dialog'),('server_packs','pack_prompt','pack_confirm'),('win','win_screen','win'),('code_of_conduct','code_of_conduct','code_of_conduct'),('chat','link_prompt','link_prompt'),('remote_chat','command_prompt','command_prompt')]
for mod,fn,id in screens:
    if fn == 'friends_screen':
        # Feature-gated modules cannot be targeted by declarative mixins.
        # This page uses capture's standard batch.blur_at fallback.
        continue
    if fn == 'creative':
        mod = 'creative_screen'
    target=f"horizon_client::game::{mod+'::' if mod else ''}Game<'_>::draw_{fn}"
    hook('begin_'+fn,target,[G,B],f'{I}::begin_screen(game,batch,"{id}");')
hook('mod_begin',native+'native_screen_render_post',[G,B],f'{I}::begin_screen(game,batch,"mod");')
hook('screen_end',"horizon_client::game::titles::Game<'_>::draw_saving_indicator",[B],f'{I}::screen_drawn(batch);')
hook('draw',"horizon_client::game::narrator::Game<'_>::draw_narrator_toast",[G,B],f'{I}::capture(game,batch);')
hook('pad_cursor',"horizon_client::game::pad_menu::Game<'_>::draw_pad_cursor",[G,B],f'if {I}::pad_cursor(game,batch) {{ core::ops::ControlFlow::Break(()) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<()>',True)
hook('blur',"horizon_client::game::screen_backgrounds::Game<'_>::menu_blur",[],f'if {I}::suppress_blur() {{ core::ops::ControlFlow::Break(None) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<Option<f32>>',True)
hook('fov',native+'native_compute_fov',[G,('degrees','&mut f32','&mut *degrees')],f'{I}::fov(game,degrees);')
hook('hand',native+'native_render_hand',[G,('stack','Option<&horizon_client::items::ItemStack>','stack'),('canceled','&mut bool','&mut *canceled')],f'{I}::hide_hand(game,stack,canceled);')
hook('hand_pose',native+'native_render_hand_pose',[G,('hand','horizon_client::first_person::Hand','hand'),('z','&mut f32','&mut *z'),('pitch','&mut f32','&mut *pitch')],f'{I}::hand_pose(game,hand,z,pitch);')
hook('hud',native+'native_gui_layer_pre',[G,('layer','&str','layer'),('canceled','&mut bool','&mut *canceled')],f'{I}::hide_hud(game,layer,canceled);')
hook('settings',native+'native_settings',[G,('mod_id','&str','mod_id'),('buttons','&mut Vec<String>','&mut *buttons')],f'if mod_id == {I}::ID {{ buttons.extend({C}::settings(game)); }}')
hook('click',native+'native_settings_click',[GM,('mod_id','&str','mod_id'),('button','usize','button')],f'if mod_id == {I}::ID {{ {C}::settings_click(game,button); }}')
keys=[['editor','Toggle visual editor','key.keyboard.f8'],['profiles','Choose a profile','key.keyboard.grave.accent'],['config','Open configuration','key.keyboard.f7'],['reload','Reload configuration','key.keyboard.f6'],['mouse_free','Toggle free cursor','key.keyboard.tab']]
key_expr='vec!['+','.join('['+','.join(json.dumps(s)+'.into()' for s in k)+']' for k in keys)+']'
hook('list',native+'native_mods',[('mods','&mut Vec<crate::game::native_hooks::NativeMod>','&mut *mods')],f'mods.push(crate::game::native_hooks::NativeMod {{ id: {I}::ID.into(), title: format!("Spatial GUI {{}}",{I}::VERSION), description: "World-space menus, camera transitions, curved input and visual editor. Rust port of tastytrash/Spatial-GUI.".into(), keys: {key_expr} }});')
R='crate::hud::spatial_gui'
hook('render_needs','horizon_client_render::gfx::native_render::native_render_needs',[('needs','&mut crate::gfx::native_render::RenderNeeds','&mut *needs')],f'{R}::needs(needs);')
hook('world_drawn','horizon_client_render::gfx::native_render::native_world_drawn',[('world',"&mut crate::gfx::native_render::WorldDrawn<'_>",'&mut *world')],f'{R}::world_drawn(world);')
hook('hand_drawn','horizon_client_render::gfx::native_render::native_hand_drawn',[('hand',"&mut crate::gfx::native_render::HandDrawn<'_>",'&mut *hand')],f'{R}::hand_drawn(hand);')
hook('render','horizon_client_render::hud::HudRenderer::draw_range',[
    ('renderer','&crate::hud::HudRenderer','&*self'),('encoder','&mut wgpu::CommandEncoder','&mut *encoder'),('target','&wgpu::TextureView','target'),('coverage','Option<&wgpu::TextureView>','coverage'),('depth','&wgpu::TextureView','depth'),('batch','&crate::hud::HudBatch','batch'),('size','[u32;2]','size'),('vertices','&std::ops::Range<u32>','&vertices')],f'if {R}::draw(renderer,encoder,target,coverage,depth,batch,size,vertices.clone()) {{ core::ops::ControlFlow::Break(()) }} else {{ core::ops::ControlFlow::Continue(()) }}','core::ops::ControlFlow<()>',True)
(BUNDLE/'horizon-mixin.toml').write_text(''.join(parts))

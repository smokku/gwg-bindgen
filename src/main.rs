extern crate gwg as ggez;
#[macro_use]
extern crate cfg_if;

use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::{graphics, Context, GameResult};

use log::info;

struct MainState {}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {};
        Ok(s)
    }
}
impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        info!("update");
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        info!("draw");
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        info!("key_down_event {:?} {:?} {:?}", keycode, keymod, repeat);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        info!("key_up_event {:?} {:?}", keycode, keymod);
    }

    fn resize_event(&mut self, context: &mut Context, w: f32, h: f32) {
        let coordinates = graphics::Rect::new(0., 0.0, w, h);
        graphics::set_screen_coordinates(context, coordinates).expect("Can't resize the window");
        info!("resize_event {}x{}", w, h);
    }
}

pub fn main() -> GameResult {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            web_logger::custom_init(web_logger::Config { level: log::Level::Info });
        } else {
            simple_logger::init_with_level(log::Level::Info).expect("A logger was already initialized");
        }
    }

    ggez::start(
        ggez::conf::Conf {
            loading: ggez::conf::Loading::Embedded,
            ..Default::default()
        },
        |mut context| Box::new(MainState::new(&mut context).unwrap()),
    )
}

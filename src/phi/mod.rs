use self::gfx::Sprite;
use ::sdl2::render::Renderer;
use ::sdl2::pixels::Color;
use ::std::collections::HashMap;
use ::std::path::Path;

#[macro_use]
mod events;
pub mod gfx;
pub mod data;

struct_events!{
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_space: Space,
        key_left: Left,
        key_right: Right,
        key_return: Return
    },
    else: {
        quit: Quit { .. }
    }
}

pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
    
    cached_fonts: HashMap<(&'static str, i32), ::sdl2_ttf::Font>,
}

impl<'window> Phi<'window> {
    fn new(events: Events, renderer: Renderer<'window>) -> Phi<'window> {
        ::sdl2_image::init(::sdl2_image::INIT_PNG);

        Phi {
            events: events,
            renderer: renderer,

            cached_fonts: HashMap::new(),
        }
    }

    pub fn output_size(&self) -> (f64, f64) {
        let (w, h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
    }

    pub fn ttf_str_sprite(&mut self, text: &str, font_path: &'static str, size: i32, color: Color) -> Option<Sprite> {
        if let Some(font) = self.cached_fonts.get(&(font_path, size)) {
            return font.render(text, ::sdl2_ttf::blended(color)).ok()
                .and_then(|surface| self.renderer.create_texture_from_surface(&surface).ok()).map(Sprite::new);
        }

        let font = ::sdl2_ttf::Font::from_file(Path::new(font_path), size).ok().unwrap();
        let surface = font.render(text, ::sdl2_ttf::blended(color)).ok().unwrap();
        Some(Sprite::new(self.renderer.create_texture_from_surface(&surface).ok().unwrap()))
    }
}

impl<'window> Drop for Phi<'window> {
    fn drop(&mut self) {
        ::sdl2_image::quit();
    }
}

pub enum ViewAction {
    None,
    Quit,
    ChangeView(Box<View>),
}

pub trait View {
    fn resume(&mut self, _context: &mut Phi) {}

    fn pause(&mut self, _context: &mut Phi) {}

    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}

pub fn spawn<F>(title: &str, init: F)
where F: Fn(&mut Phi) -> Box<View> {
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let _ttf_context = ::sdl2_ttf::init();

    let window = video.window(title, 800, 600)
        .position_centered().opengl().resizable()
        .build().unwrap();

    let mut context = Phi::new(
        Events::new(sdl_context.event_pump().unwrap()),
        window.renderer().accelerated()
            .build().unwrap()
        );

    let mut current_view = init(&mut context);
    current_view.resume(&mut context);

    let interval = 1000/60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    loop {
        let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1000.;

        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }

        context.events.pump(&mut context.renderer);

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => {
                current_view.pause(&mut context);
                break;
            },
            ViewAction::ChangeView(new_view) => {
                current_view.pause(&mut context);
                current_view = new_view;
                current_view.resume(&mut context);
            },
        }
    }
}


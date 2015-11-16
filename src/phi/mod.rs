use ::sdl2::render::Renderer;

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
        key_right: Right
    },
    else: {
        quit: Quit { .. }
    }
}

pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>,
}

impl<'window> Phi<'window> {
    fn new(events: Events, renderer: Renderer<'window>) -> Phi<'window> {
        ::sdl2_image::init(::sdl2_image::INIT_PNG);

        Phi {
            events: events,
            renderer: renderer,
        }
    }

    pub fn output_size(&self) -> (f64, f64) {
        let (w, h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
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
    let mut timer_subs = sdl_context.timer().unwrap();

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
    let mut before = timer_subs.ticks();
    let mut last_second = timer_subs.ticks();
    let mut fps = 0u16;

    loop {
        let now = timer_subs.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1000.;

        if dt < interval {
            timer_subs.delay(interval - dt);
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


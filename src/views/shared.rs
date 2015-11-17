use ::phi::gfx::{Sprite, CopySprite};
use ::phi::data::Rectangle;
use ::sdl2::render::Renderer;

#[derive(Clone)]
pub struct BgSet {
    pub bg_back: Background,
    pub bg_middle: Background,
    pub bg_front: Background,
}

impl BgSet {
    pub fn new(renderer: &mut Renderer, bg_path: &'static str, mid_path: &'static str, fg_path: &'static str) -> BgSet {
        BgSet {
            bg_back: Background {
                pos: 0.,
                vel: 20.,
                sprite: Sprite::load(renderer, bg_path).unwrap(),
            },

            bg_middle: Background {
                pos: 0.,
                vel: 40.,
                sprite: Sprite::load(renderer, mid_path).unwrap(),
            },

            bg_front: Background {
                pos: 0.,
                vel: 60.,
                sprite: Sprite::load(renderer, fg_path).unwrap(),
            },
        }
    }

    pub fn _render(&mut self, renderer: &mut Renderer, elapsed: f64) {
        self.render_bg(renderer, elapsed);
        self.render_fg(renderer, elapsed);
    }

    pub fn render_bg(&mut self, renderer: &mut Renderer, elapsed: f64) {
        self.bg_back.render(renderer, elapsed);
        self.bg_middle.render(renderer, elapsed);
    }

    pub fn render_fg(&mut self, renderer: &mut Renderer, elapsed: f64) {
        self.bg_front.render(renderer, elapsed);
    }
}

#[derive(Clone)]
pub struct Background {
    pos: f64,
    vel: f64,
    sprite: Sprite,
}

impl Background {
    pub fn render(&mut self, renderer: &mut Renderer, elapsed: f64) {
        let size = self.sprite.size();
        self.pos += self.vel * elapsed;

        if self.pos > size.0 {
            self.pos -= size.0;
        }

        let (win_w, win_h) = renderer.output_size().unwrap();
        let scale = win_h as f64 / size.1;

        let mut physical_left = -self.pos * scale;

        while physical_left < win_w as f64 {
            renderer.copy_sprite(&self.sprite, Rectangle {
                x: physical_left,
                y: 0.,
                w: size.0 * scale,
                h: win_h as f64,
            });

            physical_left += size.0 * scale;
        }
    }
}


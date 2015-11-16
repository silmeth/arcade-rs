use ::phi::data::Rectangle;
use ::std::cell::RefCell;
use ::std::path::Path;
use ::std::rc::Rc;
use ::sdl2::render::{Renderer, Texture};
use ::sdl2_image::LoadTexture;

#[derive(Clone)]
pub struct Sprite {
    tex: Rc<RefCell<Texture>>,
    src: Rectangle,
}

impl Sprite {
    pub fn new(texture: Texture) -> Sprite {
        let tex_query = texture.query();

        Sprite {
            tex: Rc::new(RefCell::new(texture)),
            src: Rectangle {
                x: 0.,
                y: 0.,
                w: tex_query.width as f64,
                h: tex_query.height as f64,
            },
        }
    }

    pub fn load(renderer: &Renderer, path: &str) -> Option<Sprite> {
        renderer.load_texture(Path::new(path)).ok().map(Sprite::new)
    }

    pub fn region(&self, rect: Rectangle) -> Option<Sprite> {
        let new_src = Rectangle {
            x: self.src.x + rect.x,
            y: self.src.y + rect.y,
            // syntax sugar for:
            // w: rect.w,
            // h: rect.h,
            ..rect
        };

        if self.src.contains(rect) {
            Some(Sprite {
                src: new_src,
                tex: self.tex.clone(),
            })
        } else {
            None
        }
    }

    pub fn size(&self) -> (f64, f64) {
        (self.src.w, self.src.h)
    }

    pub fn render(&self, renderer: &mut Renderer, dest: Rectangle) {
        renderer.copy(&mut self.tex.borrow_mut(), self.src.to_sdl(), dest.to_sdl());
    }
}

pub trait CopySprite {
    fn copy_sprite(&mut self, sprite: &Sprite, dest: Rectangle);
}

impl<'window> CopySprite for Renderer<'window> {
    fn copy_sprite(&mut self, sprite: &Sprite, dest:Rectangle) {
        sprite.render(self, dest);
    }
}

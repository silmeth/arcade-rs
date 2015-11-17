use ::phi::{Phi, View, ViewAction};
use ::phi::data::Rectangle;
use ::phi::gfx::{Sprite, CopySprite};
use ::sdl2::pixels::Color;
use ::sdl2::render::Renderer;
use ::views::shared::BgSet;

const PLAYER_SPEED: f64 = 180.;
const SHIP_W: f64 = 43.;
const SHIP_H: f64 = 39.;
const DEBUG: bool = false;

#[derive(Clone, Copy)]
enum ShipFrame {
    UpNorm = 0,
    UpFast = 1,
    UpSlow = 2,
    MidNorm = 3,
    MidFast = 4,
    MidSlow = 5,
    DownNorm = 6,
    DownFast = 7,
    DownSlow = 8,
}

struct Ship {
    rect: Rectangle,
    sprites: Vec<Sprite>,
    current: ShipFrame,
}

pub struct ShipView {
    player: Ship,

    bg_set: BgSet,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        let bg_set = BgSet::new(&mut phi.renderer, "assets/starBG.png", "assets/starMG.png", "assets/starFG.png");
        ShipView::with_backgrounds(phi, bg_set)
    }

    pub fn with_backgrounds(phi: &mut Phi, bg_set: BgSet) -> ShipView {
        let spritesheet = Sprite::load(&mut phi.renderer, "assets/spaceship.png").unwrap();
        let mut sprites = Vec::with_capacity(9);

        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                    x: SHIP_W * x as f64,
                    y: SHIP_H * y as f64,
                    w: SHIP_W,
                    h: SHIP_H,
                }).unwrap());
            }
        }


        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.,
                    y: 64.,
                    w: SHIP_W, // width as f64 / 3.,
                    h: SHIP_H, // height as f64 / 3.,
                },
                sprites: sprites,
                current: ShipFrame::MidNorm,
            },
            
            bg_set: bg_set,
        }
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit {
            return ViewAction::Quit;
        }
        
        if phi.events.now.key_escape == Some(true) {
            return ViewAction::ChangeView(Box::new(
                    ::views::main_menu::MainMenuView::with_backgrounds(phi, self.bg_set.clone())
            ));
        }

        let diagonal = (phi.events.key_up ^ phi.events.key_down) &&
                       (phi.events.key_left ^ phi.events.key_right);

        let moved =
            if diagonal { 1./2f64.sqrt() }
            else { 1. } * PLAYER_SPEED * elapsed;

        let dx = match (phi.events.key_left, phi.events.key_right) {
            (true, true) | (false, false) => 0.,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (phi.events.key_up, phi.events.key_down) {
            (true, true) | (false, false) => 0.,
            (true, false) => -moved,
            (false, true) => moved,
        };

        self.player.rect.x += dx;
        self.player.rect.y += dy;

        let movable_region = Rectangle {
            x: 0.,
            y: 0.,
            w: phi.output_size().0 * 0.7,
            h: phi.output_size().1,
        };

        self.player.rect = self.player.rect.move_inside(movable_region).unwrap();

        self.player.current =
            if dx == 0. && dy < 0. { ShipFrame::UpNorm }
            else if dx > 0. && dy < 0. { ShipFrame::UpFast }
            else if dx < 0. && dy < 0. { ShipFrame::UpSlow }
            else if dx == 0. && dy == 0. { ShipFrame::MidNorm }
            else if dx > 0. && dy == 0. { ShipFrame::MidFast }
            else if dx < 0. && dy == 0. { ShipFrame::MidSlow }
            else if dx == 0. && dy > 0. { ShipFrame::DownNorm }
            else if dx > 0. && dy > 0. { ShipFrame::DownFast }
            else if dx < 0. && dy > 0. { ShipFrame::DownSlow }
            else { unreachable!() };

        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        self.bg_set.render_bg(&mut phi.renderer, elapsed);
        
        if DEBUG {
            phi.renderer.set_draw_color(Color::RGB(20, 240, 180));
            phi.renderer.fill_rect(self.player.rect.to_sdl().unwrap());
        }

        phi.renderer.copy_sprite(&self.player.sprites[self.player.current as usize], self.player.rect);

        self.bg_set.render_fg(&mut phi.renderer, elapsed);

        ViewAction::None
    }
}



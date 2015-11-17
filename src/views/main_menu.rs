use ::phi::{Phi, View, ViewAction};
use ::phi::data::Rectangle;
use ::phi::gfx::{Sprite, CopySprite};
use ::views::shared::BgSet;
use ::sdl2::pixels::Color;

struct Action {
    func: Box<Fn(&mut Phi, BgSet) -> ViewAction>,
    idle_sprite: Sprite,
    hover_sprite: Sprite,
}

impl Action {
    fn new(phi: &mut Phi, label: &'static str, func: Box<Fn(&mut Phi, BgSet) -> ViewAction>) -> Action {
        Action {
            func: func,
            idle_sprite: phi.ttf_str_sprite(label, "assets/belligerent.ttf", 32, Color::RGB(220, 220, 220)).unwrap(),
            hover_sprite: phi.ttf_str_sprite(label, "assets/belligerent.ttf", 38, Color::RGB(255, 255, 255)).unwrap(),
        }
    }
}

pub struct MainMenuView {
    actions: Vec<Action>,
    selected: i8,

    bg_set: BgSet,
}

impl MainMenuView {
    pub fn new(phi: &mut Phi) -> MainMenuView {
        let bg_set = BgSet::new(&mut phi.renderer, "assets/starBG.png", "assets/starMG.png", "assets/starFG.png");
        MainMenuView::with_backgrounds(phi, bg_set)
    }

    pub fn with_backgrounds(phi: &mut Phi, bg_set: BgSet) -> MainMenuView {
        MainMenuView {
             actions: vec![
                Action::new(phi, "New game", Box::new(|phi, bg| {
                    ViewAction::ChangeView(Box::new(::views::game::ShipView::with_backgrounds(phi, bg)))
                })),
                Action::new(phi, "Quit", Box::new(|_, _| {
                    ViewAction::Quit
                }))
            ],
            selected: 0,

            bg_set: bg_set,
        }
    }
}

impl View for MainMenuView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if phi.events.now.key_space == Some(true) || phi.events.now.key_return == Some(true) {
            return (self.actions[self.selected as usize].func)(phi, self.bg_set.clone());
        }

        if phi.events.now.key_up == Some(true) {
            self.selected -= 1;
            if self.selected < 0 {
                self.selected = self.actions.len() as i8 - 1;
            }
        }

        if phi.events.now.key_down == Some(true) {
            self.selected += 1;
            if self.selected >= self.actions.len() as i8 {
                self.selected = 0;
            }
        }

        phi.renderer.set_draw_color(Color::RGB(0, 0, 0));
        phi.renderer.clear();

        self.bg_set.render_bg(&mut phi.renderer, elapsed);

        let (win_w, win_h) = phi.output_size();
        let label_h: f64 = 50.;
        let border_width: f64 = 3.;
        let box_w: f64 = 360.;
        let box_h: f64 = self.actions.len() as f64 * label_h;
        let margin_h: f64 = 10.;

        // border
        phi.renderer.set_draw_color(Color::RGB(70, 15, 70));
        phi.renderer.fill_rect(Rectangle {
            x: (win_w - box_w) / 2. - border_width,
            y: (win_h - box_h) / 2. - border_width - margin_h,
            w: box_w + border_width * 2.,
            h: box_h + border_width * 2. + margin_h * 2.,
        }.to_sdl().unwrap());

        // menu box
        phi.renderer.set_draw_color(Color::RGB(140, 30, 140));
        phi.renderer.fill_rect(Rectangle {
            x: (win_w - box_w) / 2.,
            y: (win_h - box_h) / 2. - margin_h,
            w: box_w,
            h: box_h + margin_h * 2.,
        }.to_sdl().unwrap());

        for (i_action, action) in self.actions.iter().enumerate() {
            if i_action == self.selected as usize {
                let (w, h) = action.hover_sprite.size();
                phi.renderer.copy_sprite(&action.hover_sprite, Rectangle {
                    x: (win_w - w)/2.,
                    y: (win_h - box_h + label_h - h) / 2. + label_h * i_action as f64,
                    w: w,
                    h: h,
                });
            } else {
                let (w, h) = action.idle_sprite.size();
                phi.renderer.copy_sprite(&action.idle_sprite, Rectangle {
                    x: (win_w - w)/2.,
                    y: (win_h - box_h + label_h - h) / 2. + label_h * i_action as f64,
                    w: w,
                    h: h,
                });
            }
        }

        self.bg_set.render_fg(&mut phi.renderer, elapsed);

        ViewAction::None
    }
}

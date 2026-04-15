use macroquad::prelude::*;

pub struct Leaf {
    pub scale: f32,
    pub center: Vec2,
}

impl Leaf {
    pub fn new(scale: f32) -> Self {
        Self {
            scale,
            center: vec2(screen_width() / 2.0, screen_height() / 2.0),
        }
    }

    // Boundary check logic used for auxin generation.
    pub fn contains(&self, p: Vec2) -> bool {
        let x = (p.x - self.center.x) / self.scale;
        let y = (p.y - self.center.y) / self.scale;

        if y < -0.5 || y > 0.5 { return false; }

        let width_at_y = (0.5 - y) * (0.5 + y).abs().sqrt() * 1.2;
        x.abs() < width_at_y
    }

    // AI slop.
    pub fn draw_outline(&self) {
        let mut prev_l: Option<Vec2> = None;
        let mut prev_r: Option<Vec2> = None;
        let steps = 60;

        for i in 0..=steps {
            let y_sample = -0.5 + (i as f32 / steps as f32);
            let width = (0.5 - y_sample) * (0.5 + y_sample).abs().sqrt() * 1.2;

            let pos_y = self.center.y + (y_sample * self.scale);
            let pos_xr = self.center.x + (width * self.scale);
            let pos_xl = self.center.x - (width * self.scale);

            let curr_r = vec2(pos_xr, pos_y);
            let curr_l = vec2(pos_xl, pos_y);

            if let (Some(pr), Some(pl)) = (prev_r, prev_l) {
                draw_line(pr.x, pr.y, curr_r.x, curr_r.y, 3.0, BLACK);
                draw_line(pl.x, pl.y, curr_l.x, curr_l.y, 3.0, BLACK);
            }
            prev_r = Some(curr_r);
            prev_l = Some(curr_l);
        }
    }

    pub fn get_stem_base(&self) -> Vec2 {
        vec2(self.center.x, self.center.y + (self.scale * 0.5))
    }
}
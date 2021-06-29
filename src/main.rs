extern crate cairo;
extern crate rand;

use cairo::{Context, Format, ImageSurface};

use std::f64::consts;
use std::fs::File;

struct Rectangle {
    pub top_left: (f64, f64),
    pub bottom_right: (f64, f64),
}

impl Rectangle {
    pub fn draw(self: &Self, ctx: &Context) {
        let circle_width = 10.0;

        let draw_circle_part = |coord: (f64, f64), rotate_num: i32| {
            let offset_mult = match rotate_num {
                2 => (1.0, 1.0),
                3 => (-1.0, 1.0),
                4 => (-1.0, -1.0),
                5 => (1.0, -1.0),
                _ => (0.0, 0.0),
            };
            ctx.arc(
                coord.0 + circle_width * offset_mult.0,
                coord.1 + circle_width * offset_mult.1,
                circle_width,
                (rotate_num as f64) * 0.5 * consts::PI,
                (rotate_num as f64 + 1.0) * 0.5 * consts::PI,
            );
        };

        draw_circle_part(self.top_left, 2);

        let coord = (self.bottom_right.0, self.top_left.1);
        ctx.line_to(coord.0 - circle_width, coord.1);
        draw_circle_part(coord, 3);

        ctx.line_to(self.bottom_right.0, self.bottom_right.1 - circle_width);
        draw_circle_part(self.bottom_right, 4);

        let coord = (self.top_left.0, self.bottom_right.1);
        ctx.line_to(coord.0 + circle_width, coord.1);
        draw_circle_part(coord, 5);

        ctx.line_to(self.top_left.0, self.top_left.1 + circle_width);

        ctx.stroke();

        // ctx.set_source_rgb(1.0, 0.0, 0.0);

        // ctx.line_to(self.top_left.0, self.top_left.1);
        // ctx.line_to(self.bottom_right.0, self.top_left.1);
        // ctx.line_to(self.bottom_right.0, self.bottom_right.1);
        // ctx.line_to(self.top_left.0, self.bottom_right.1);
        // ctx.line_to(self.top_left.0, self.top_left.1);

        // ctx.stroke();
    }
}

fn draw_circle(ctx: &Context, coord: (f64, f64), radius: f64) {
    ctx.arc(coord.0, coord.1, radius, 0.0, 2.0 * consts::PI);
    ctx.stroke();
}

fn main() {
    let width = 1180;
    let height = 628;

    let surface = ImageSurface::create(Format::ARgb32, width, height).expect("create surface");
    let ctx = Context::new(&surface);

    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.paint();

    // change coordinates
    // ctx.translate(40.0, 27.0);

    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.set_line_width(4.0);

    let circle_r = 60.0;
    let circles_offset = 15.0;
    let top_margin = 13.0;
    let border_margin = 20.0;

    let person_box = Rectangle {
        top_left: (border_margin, top_margin),
        bottom_right: (490.0, 140.0),
    };

    person_box.draw(&ctx);

    let circle = |num: i32| {
        let x_offset = person_box.bottom_right.0
            + circles_offset
            + circle_r
            + num as f64 * (circles_offset + 2.0 * circle_r);
        draw_circle(
            &ctx,
            (
                x_offset,
                top_margin + (person_box.bottom_right.1 - person_box.top_left.1) / 2.0,
            ),
            circle_r,
        );
    };

    for i in 0..5 {
        circle(i);
    }

    let pvp_box = Rectangle {
        top_left: (person_box.top_left.0, person_box.bottom_right.1 + 10.0),
        bottom_right: (417.0, 597.0),
    };

    pvp_box.draw(&ctx);

    // draw_circle(&ctx, (693.0+63.5, 83.0), 63.5);

    // ctx.set_line_width(4.0);
    // ctx.arc(0.0, 0.0, 10.0, consts::PI, 1.5 * consts::PI);
    // ctx.stroke();

    let mut file = File::create("output.png").expect("create file failed");
    surface.write_to_png(&mut file).expect("write to file");
}

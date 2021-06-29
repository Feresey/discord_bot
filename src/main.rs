extern crate cairo;
extern crate rand;

use cairo::{Context, Format, ImageSurface};

use std::f64::consts;
use std::fs::File;

fn draw_box(ctx: Context, left_top: (f64, f64), right_down: (f64, f64)) {
    let circle_width = 10.0;

    let draw_circle = |coord: (f64, f64), rotate_num: i32| {
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

    draw_circle(left_top, 2);

    let coord = (right_down.0, left_top.1);
    ctx.line_to(coord.0 - circle_width, coord.1);
    draw_circle(coord, 3);

    ctx.line_to(right_down.0, right_down.1 - circle_width);
    draw_circle(right_down, 4);

    let coord = (left_top.0, right_down.1);
    ctx.line_to(coord.0 + circle_width, coord.1);
    draw_circle(coord, 5);

    ctx.line_to(left_top.0, left_top.1 + circle_width);

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
    ctx.translate(40.0, 27.0);

    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.set_line_width(4.0);

    draw_box(ctx, (0.0, 0.0), (300.0, 150.0));

    // ctx.set_line_width(4.0);
    // ctx.arc(0.0, 0.0, 10.0, consts::PI, 1.5 * consts::PI);
    // ctx.stroke();

    let mut file = File::create("output.png").expect("create file failed");
    surface.write_to_png(&mut file).expect("write to file");
}

// extern crate rand;
use askama::Template;
use wkhtmltopdf::*;
// use std::f64::consts;
use std::fs::File;
use std::io::prelude::*;

#[derive(Template)]
#[template(path = "body.html")]
struct BaseTemplate {
    user: User,
    pvp: PVP,
    text: Text,
}

struct User {
    name: String,
    clan: String,
    clan_tag: String,
    karma: i32,
}

struct Text {
    total_battles: String,
    pvp_time: String,
    winrate: String,
    kd: String,
    avg_kills: String,
}

struct PVP {
    total_battles: i32,
    total_wins: i32,
    total_kills: i32,
    total_deaths: i32,
    total_damage: f32,
    total_heal: f32,
    total_assists: f32,
    total_time: String,

    winrate: f32,
    kd: f32,
    avg_kills: f32,
    avg_deaths: f32,
    avg_assists: f32,
    avg_damage: f32,
    avg_heal: f32,
}
fn main() {
    let s = BaseTemplate {
        user: User {
            name: "name".to_string(),
            clan: "clan".to_string(),
            clan_tag: "tag".to_string(),
            karma: 32,
        },
        pvp: PVP {
            total_battles: 32,
            total_wins: 32,
            total_kills: 123,
            total_deaths: 123,
            total_damage: 123.0,
            total_assists: 123.0,
            total_time: "total_time".to_string(),
            total_heal: 32.0,

            winrate: 32.0,
            kd: 32.0,
            avg_kills: 32.0,
            avg_deaths: 32.0,
            avg_assists: 32.0,
            avg_damage: 32.0,
            avg_heal: 32.0,
        },
        text: Text {
            total_battles: "Total battles".to_string(),
            pvp_time: "PVP time".to_string(),
            winrate: "Winrate".to_string(),
            kd: "K/D".to_string(),
            avg_kills: "AVG kills".to_string(),
        },
    };

    let res = s.render().unwrap();
    println!("{}", res);

    let mut out = File::create("out.html").unwrap();
    out.write_all(res.as_bytes()).unwrap();

    let pdf_app = PdfApplication::new().expect("init pdf application");
    unsafe {
        let mut pfd_out = pdf_app
            .builder()
            .orientation(Orientation::Landscape)
            .margin(Size::Millimeters(7))
            // .object_setting(
            //     "web.userStyleSheet",
            //     "file:///home/lol/work/discord_bot/templates/css/style.css".to_string(),
            // )
            .build_from_html(res)
            // .build_from_url("file:///home/lol/work/discord_bot/out.html".parse().unwrap())
            .expect("failed to build pdf");

        pfd_out.save("out.pdf").unwrap();
    }
}

mod lib;

use crate::lib::hex_to_color;
use crate::lib::color_to_hex;
use structopt::StructOpt;
use std::{collections::HashMap, fs};
use palette::{Hue, Saturate, Shade, IntoColor, rgb::Rgb};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
//#[serde(rename_all = "PascalCase")]
struct ColorScheme{
    wallpaper: String, 
    alpha: String, 
    colors: HashMap<String, String>, 
    special: HashMap<String, String>, 
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    #[structopt(long = "hisat", help = "saturation for highlight colors", default_value = "0", group="test")]
    hisat: f32,
    #[structopt(long = "hidesat", help = "desaturation for highlight colors", default_value = "0")]
    hidesat: f32,
    #[structopt(long = "hihue", help = "hue for highlight colors", default_value = "0")]
    hihue: f32,
    #[structopt(long = "hilight", help = "lightness for highlight colors", default_value = "0")]
    hilight: f32,

    #[structopt(long = "nsat", help = "saturation for normal colors", default_value = "0")]
    nsat: f32,
    #[structopt(long = "ndesat", help = "desaturation for normal colors", default_value = "0")]
    ndesat: f32,
    #[structopt(long = "nhue", help = "hue for normal colors", default_value = "0")]
    nhue: f32,
    #[structopt(long = "nlight", help = "lightness for normal colors", default_value = "0")]
    nlight: f32,
}

fn main() {
    let opt = Cli::from_args();
    let contents = fs::read_to_string(opt.path)
        .expect("Could not read file.");

    println!("Content!\n{}", contents);
    let colorscheme:ColorScheme = serde_json::from_str(&contents)
        .expect("Something went wrong reading json");

    for mut color in &colorscheme.colors {
        let convert = hex_to_color(color.1.to_string());
        let mut rgb_color: Rgb;

        match convert {
            Some(c) => rgb_color = c, 
            None => continue
        };

        if match color.0.chars().last() {
            Some(a) => a,
            None => continue
        } as u8 > 8 {
            rgb_color = Rgb::from_linear(
                Rgb::
                into_lch(rgb_color)
                .desaturate(opt.ndesat)
                .saturate(opt.nsat)
                .lighten(opt.nlight)
                .shift_hue(opt.nhue)
                .into_rgb());
            color.1 = &color_to_hex(rgb_color);
        } else {
            rgb_color = Rgb::from_linear(
                Rgb::
                into_lch(rgb_color)
                .desaturate(opt.hidesat)
                .saturate(opt.hisat)
                .lighten(opt.hilight)
                .shift_hue(opt.hihue)
                .into_rgb());
            color.1 = &color_to_hex(rgb_color);
        }
    }
    let output_str = serde_json::to_string(&colorscheme).expect("oops");
    println!("{}", output_str);
}


#![allow(unused)]
use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;
// predlude is a collection of names that are automatically brought into the 
// scope of every module in a cracte
// https://doc.rust-lang.org/reference/names/preludes.html
// https://docs.rs/nannou/latest/nannou/prelude/index.html
// https://guide.nannou.cc/tutorials/basics/anatomy-of-a-nannou-app
use nannou::prelude::*;
use std::path::PathBuf;


fn main() {
    // start building the app and specify out `model`
    nannou::app(model)
        // specify we want to handle app events with `event`
        .update(update)
        // request a simple window to which we'll draw with `view`
        .simple_window(view)
        .run();
}

// the initial instance of the Model is created here, basically a setup function
fn model(app: &App) -> Model {
    //
    // TEXTURES
    //
    //let assets = app.assets_path().unwrap();
    //let img_path = assets.join("batteries").join("1J_Battery.png");
    //println!("Trying to load texture from {:?}", img_path);
    // damn, the docs are not good! : the asset path needs to be manually copied
    // to target/debug. no thanks.
    //let textures = wgpu::Texture::from_path(app, img_path).unwrap();
    // using relative path from execution point instead
    let batteries_path: PathBuf = ["assets", "batteries"].iter().collect();
    let states = ["1J", "2J", "3J", "4J", "5J", "6J", "7J", "8J", "9J", "off" ].into_iter();
    let mut battery_textures = HashMap::new();
    for state in states {
        let mut battery_texture_name = String::from(state);
        battery_texture_name.push_str("_Battery.png");
        let battery_path = batteries_path.join(battery_texture_name);
        println!("Trying to load texture from {:?}", battery_path);
        battery_textures.insert(state, wgpu::Texture::from_path(app, battery_path).unwrap());
    }

    //
    // BANKS
    //
    let example = "818181911112111";
    let bank = get_bank_from_str(&example, battery_textures);
    let mut banks = Vec::new();

    Model {
        banks: banks,
    }
}

// application state. events, such as mouse presses, key presses or timed updates
struct Model {
    banks: Vec<Bank>,
}

#[derive(Debug, Clone)]
struct Bank {
    batteries: Vec<Battery>,
}

#[derive(Debug, Clone, Copy)]
struct Battery {
    joltage: u32,
    on_texture: &wgpu::Texture,
    off_texture: &wgpu::Texture,
}


fn get_bank_from_str(bank_str: &str, battery_textures: HashMap<&str, wgpu::Texture>) -> Bank {
    let mut bank = Bank {
        batteries: Vec::new(),
    };
    for joltage_c in bank_str.chars() {
        match joltage_c {
            '0'..='9' => {
                let battery = Battery{
                    joltage: joltage_c as u32 - '0' as u32,
                    // thx SO https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1
                    on_texture: battery_textures["1J"],
                    off_texture: battery_textures["off"],
                };
                bank.batteries.push(battery.clone());
            },
            _ => continue,
        }
    }
    return bank;
}
// we could also pass the event function instead of the update function
// the event function allows for complex event handling such as mouse or keyboard
// input, the update fuunction is simply called every second 60 times
// possible events: https://github.com/nannou-org/nannou/blob/master/examples/nannou_basics/all_functions.rs
fn update(_app: &App, _model: &mut Model, _update: Update) {
}

// view draws to it's `frame` and returns? a frame? ?? 
fn view(app: &App, model: &Model, frame: Frame){
    frame.clear(PURPLE);
    // get a canvas to draw on
    let draw = app.draw();
    draw.background().color(rgb(0.1, 0.1, 0.1));
    let win = app.window_rect();
    //draw.texture(&model.battery_imgs[0]);
    draw.to_frame(app, &frame).unwrap();
}


//fn main_old() {
//    let file_name = env::args().nth(1).unwrap(); 
//    let content: Vec<u8> = fs::read(file_name).unwrap();
//    let content_string: String = String::try_from(content).unwrap();
//
//    let example_bank = "818181911112111";
//
//    //let max_joltage = get_max_joltage(example_bank);
//    //println!("answer 1: {}", state.total_added);
//    //println!("answer 2: {}", state.total_added);
//    let mut max_joltage: u32 = 0;
//    let banks = content_string.split('\n');
//    for bank in banks {
//        println!("{bank}");
//        println!("max: {}", get_max_joltage(&bank));
//        max_joltage += get_max_joltage(&bank);
//    }
//    println!("max joltage: {}", max_joltage);
//}
//
//fn get_max_joltage(bank_str: &str) -> u32 {
//    let mut max_joltage = 0;
//    let mut bank = get_bank(bank_str);
//
//    // reverse sorting by joltage -> highest joltage first
//    bank.batteries.sort_by(|a, b| b.joltage.cmp(&a.joltage));
//    let highest = &bank.batteries[0];
//    println!("{bank}");
//    let mut bank2 = get_bank(bank_str);
//    bank.batteries = bank2.batteries.split_off(highest.pos as usize);
//    println!("{bank}");
//
//    // sorting new one
//    bank.batteries.sort_by(|a, b| b.joltage.cmp(&a.joltage));
//    
//    max_joltage = combine_batteries_to_joltage(&bank.batteries[0], &bank.batteries[1]);
//        
//    return max_joltage;
//}
//
//
//
//fn combine_batteries_to_joltage(left_bat: &Battery, right_bat: &Battery) -> u32 {
//    let mut l_jolt = left_bat.joltage.to_string();
//    let r_jolt = right_bat.joltage.to_string();
//    l_jolt.push_str(&r_jolt); 
//    return l_jolt.parse().unwrap();
//}
//
//use std::fmt;
//
//impl fmt::Display for Bank {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        //writeln!("Joltages");
//        for b in &self.batteries {
//            write!(f, "{}", b.joltage)?;
//        }
//        Ok(())
//    }
//}
//
//
//
//fn sort_bank_by_joltage(bank: &mut Bank) {
//    bank.batteries.sort(); 
//    return;
//}


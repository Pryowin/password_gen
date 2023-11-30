#![windows_subsystem = "windows"]

use clipboard::{ClipboardContext, ClipboardProvider};
use native_windows_gui as nwg;
use rand::Rng;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

const NUMERICTHRESHOLD: u32 = 7;
const UPPERCASETHRESHOLD: u32 = 6;
const WORDLENGTH: u32 = 5;

#[derive(Deserialize)]

struct Params {
    pwd_length: u32,
    pwd_has_upper: bool,
    pwd_has_lower: bool,
    pwd_has_number: bool,
    pwd_has_special: bool,
}

fn get_params() -> Params {
    let file = format!("{}/password_params.yml", env!("CARGO_MANIFEST_DIR"));
    let mut file_content = String::new();

    File::open(&file)
        .and_then(|mut f| f.read_to_string(&mut file_content))
        .expect(&format!("Unable to find {}. Application ended.", file));

    serde_yaml::from_str(&file_content).expect("Error parsing YAML file")
}

fn pwd_gen(params: &Params) -> String {
    assert!(
        params.pwd_has_lower || params.pwd_has_upper,
        "At least one type of letter must be valid"
    );

    let mut pwd_chars = Vec::new();

    for i in 1..=params.pwd_length {
        if i % WORDLENGTH == 0 && params.pwd_has_special {
            pwd_chars.push("-".to_string());
        } else {
            if params.pwd_has_number && rand::thread_rng().gen_range(0..10) > NUMERICTHRESHOLD {
                pwd_chars.push(rand::thread_rng().gen_range(0..10).to_string());
            } else {
                pwd_chars.push(pick_letter(params.pwd_has_upper, params.pwd_has_lower));
            }
        }
    }

    pwd_chars.join("")
}

fn pick_letter(pwd_has_upper: bool, pwd_has_lower: bool) -> String {
    let letters = ('a'..='z').collect::<Vec<_>>();
    let num = rand::thread_rng().gen_range(0..letters.len());

    if !pwd_has_upper {
        letters[num].to_string()
    } else if !pwd_has_lower {
        letters[num].to_uppercase().to_string()
    } else if rand::thread_rng().gen_range(0..10) > UPPERCASETHRESHOLD {
        letters[num].to_string()
    } else {
        letters[num].to_uppercase().to_string()
    }
}

fn main() {
    nwg::init().expect("Failed to initialize Native Windows GUI");

    let params = get_params();
    let pwd = pwd_gen(&params);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(pwd.clone())
        .expect("Failed to copy password to clipboard");

    let p = nwg::MessageParams {
        title: "Password Generator",
        content: &format!("Password created and in clipboard \n {}", pwd),
        buttons: nwg::MessageButtons::Ok,
        icons: nwg::MessageIcons::Warning,
    };

    assert!(nwg::message(&p) == nwg::MessageChoice::Ok)
}

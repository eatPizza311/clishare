use std::error::Error;

use structopt::StructOpt;
use strum::EnumString;

use clishare::domain::clip::field::{Content, Expires, Password, ShortCode, Title};
use clishare::service::ask::{GetClip, NewClip, UpdateClip};
use clishare::web::api::{ApiKey, API_KEY_HEADER};
use clishare::Clip;

#[derive(StructOpt, Debug)]
enum Command {
    Get {
        shortcode: ShortCode,
        #[structopt(short, long, help = "password")]
        password: Option<String>,
    },
    New {
        #[structopt(help = "content")]
        clip: String,
        #[structopt(short, long, help = "title")]
        title: Option<Title>,
        #[structopt(short, long, help = "expiraition date")]
        expires: Option<Expires>,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
    },
    Update {
        shortcode: ShortCode,
        #[structopt(help = "content")]
        clip: String,
        #[structopt(short, long, help = "title")]
        title: Option<Title>,
        #[structopt(short, long, help = "expiraition date")]
        expires: Option<Expires>,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "clipclient", about = "CliShare API Client")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
    #[structopt(default_value = "http://127.0.0.1:8000", env = "CLISHARE_ADDR")]
    addr: String,
    #[structopt(long)]
    api_key: ApiKey,
}

fn get_clip(addr: &str, ask_service: GetClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip/{}", addr, ask_service.shortcode.into_inner());
    let mut request = client.get(addr);
    request = match ask_service.password.into_inner() {
        Some(password) => request.header(reqwest::header::COOKIE, format!("password={}", password)),
        None => request,
    };

    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.send()?.json()?)
}

fn new_clip(addr: &str, ask_service: NewClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip", addr);
    let mut request = client.post(addr);
    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.send()?.json()?)
}

fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    match opt.command {
        Command::Get {
            shortcode,
            password,
        } => {
            let req = GetClip {
                password: Password::new(password.unwrap_or_default())?,
                shortcode,
            };
            todo!()
        }

        Command::New {
            clip,
            title,
            expires,
            password,
        } => {
            let req = NewClip {
                content: Content::new(clip.as_str())?,
                title: title.unwrap_or_default(),
                expires: expires.unwrap_or_default(),
                password: password.unwrap_or_default(),
            };
            todo!()
        }

        Command::Update {
            shortcode,
            clip,
            title,
            expires,
            password,
        } => {
            todo!()
        }
    }
}
fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("An error occurred: {}", e);
    }
}

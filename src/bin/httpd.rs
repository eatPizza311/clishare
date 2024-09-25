use std::path::PathBuf;

use dotenv::dotenv;
use structopt::StructOpt;

use clishare::data::AppDatabase;
use clishare::web::{hit_counter::HitCounter, renderer::Renderer};

/// Command line options
#[derive(Debug, StructOpt)]
#[structopt(name = "httpd")]
struct Opt {
    #[structopt(default_value = "sqlite:clip.db")]
    connection_string: String,
    #[structopt(short, long, parse(from_os_str), default_value = "templates/")]
    template_directory: PathBuf,
}

fn main() {
    // pull in environment variables
    dotenv().ok();
    // Read the command line arguments and create Opt struct
    let opt = Opt::from_args();

    // Since Rocket is async, so we need an executor (tokio's Runtime is our executor)
    let rt = tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime");
    // Create a handel (access to executor) to runtime so we can pass it around
    let handle = rt.handle().clone();

    let renderer = Renderer::new(opt.template_directory.clone());

    // run a future and block a thread until the future complete
    let database = rt.block_on(async move { AppDatabase::new(&opt.connection_string).await });

    let hit_counter = HitCounter::new(database.get_pool().clone(), handle.clone());

    let config = clishare::RocketConfig {
        renderer,
        database,
        hit_counter,
    };

    rt.block_on(async move {
        clishare::rocket(config)
            .launch()
            .await
            .expect("failed to launch rocket server")
    });
}

use std::path::PathBuf;

use dotenv::dotenv;
use structopt::StructOpt;

use clishare::data::AppDatabase;
use clishare::web::renderer::Renderer;

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

    // Since Rocket is async, so we need an executor
    let rt = tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime");

    let handle = rt.handle().clone();

    // run a future and block a thread until the future complete
    rt.block_on(async move {
        let renderer = Renderer::new(opt.template_directory);
        let database = AppDatabase::new(&opt.connection_string).await;

        let config = clishare::RocketConfig { renderer, database };

        clishare::rocket(config)
            .launch()
            .await
            .expect("failed to launch rocket server")
    });
}

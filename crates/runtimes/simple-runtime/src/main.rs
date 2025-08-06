use clap::Parser as _;
use config::Config;
use log::warn;
use walkdir::WalkDir;

use crate::{settings::Settings, setup::Models};

pub mod cli;
mod execute;
pub mod settings;
pub mod setup;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    let mut input = WalkDir::new(&cli.input)
        .into_iter()
        .filter_map(|v| v.ok())
        .map(|v| v.path().to_path_buf())
        .filter(|v| v.is_file())
        .map(|v| {
            v.strip_prefix(&cli.input)
                .map(|v| v.to_path_buf())
                .unwrap_or(v)
        })
        .collect::<Vec<_>>();
    let mut settings = Config::builder();
    if let Some(config) = cli.config {
        if !config.exists() {
            panic!("Config file does not exist")
        }
        settings = settings.add_source(config::File::from(config));
    }
    let settings = settings.build().expect("Failed to build settings");
    let settings = settings.try_deserialize::<Settings>().unwrap_or_default();
    if !cli.overwrite {
        //TODO: add other extensions
        input = input
            .into_iter()
            .filter(|v| !cli.output.join(v).exists())
            .collect::<Vec<_>>();
    }
    let mut models = Models::new(2, true, false).await;
    for path in input {
        let path = cli.input.join(path);
        if !path.exists() || !path.is_file() {
            warn!("File {} cant be found", path.display());
            continue;
        }
        let img = image::open(path).unwrap();
        models.execute(img, &settings).await;
    }
}

use bea::error::BeaError;
use bea::{config, user, getdata, geofips};
use clap::Parser;
use indicatif::ProgressBar;
use tracing::{info, trace};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'c', long, help = "Command to execute.")]
    command: String,
    #[arg(short = 's', long, help = "Source of file.")]
    source: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), BeaError> {
    LogTracer::init().expect("Failed to set logger.");
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("bea".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber.");
    trace!("Subscriber initialized.");
    dotenv::dotenv().ok();
    trace!("Environmental variables loaded.");

    let url = std::env::var("BEA_URL")?;
    let key = std::env::var("API_KEY")?;
    let checklist = std::env::var("CHECKLIST")?;
    let checklist_update = std::env::var("CHECKLIST_UPDATE")?;
    let bea_data = std::env::var("BEA_CAINC5N")?;
    let user = user::User::new(&url, &key);
    let mut config = config::Config::new(&user, "Regional");
    config.set_table("CAINC5N");

    let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Downloading data.'}",
        )
        .unwrap();
    let cli = Cli::parse();
    match &cli.command as &str {
        "checklist" => {
            info!("Make checklist.");
            info!("Fetching FIPS.");
            let fips = geofips::get_geofips(&config).await?;
            let tasks = geofips::GeoFipsTasks::from(&fips.results());
            let encode: Vec<u8> = bincode::serialize(&tasks)?;
            std::fs::write(checklist, encode)?;
        }
        "checklist_report" => {
            info!("Checklist report.");
            let check = std::fs::read(checklist.clone())?;
            let check: geofips::GeoFipsTasks = bincode::deserialize(&check)?;
            check.report();
        }
        "download" => {
            info!("Download data.");
            config.set_linecode("ALL")
                .set_year("ALL");
            let check = std::fs::read(&checklist)?;
            let mut check: geofips::GeoFipsTasks = bincode::deserialize(&check)?;
            if std::path::PathBuf::from(checklist_update.clone()).exists() {
                info!("Checklist found.");
                let done = std::fs::read(&checklist_update)?;
                let done: geofips::GeoFipsTasks = bincode::deserialize(&done)?;
                for item in done.tasks() {
                    for task in check.tasks_mut() {
                        if item.key() == task.key() {
                            task.set_processed(item.processed());
                        }
                    }
                }
                info!("Tasks processed: {}.", check.tasks().iter().filter(|v| v.processed() == true).collect::<Vec<&geofips::GeoFipsTask>>().len());
                info!("Tasks remaining: {}.", check.tasks().iter().filter(|v| v.processed() == false).collect::<Vec<&geofips::GeoFipsTask>>().len());
                std::fs::remove_file(checklist_update.clone())?;
            }
            let mut data = Vec::new();
            if std::path::PathBuf::from(bea_data.clone()).exists() {
                let file = std::fs::read(bea_data.clone())?;
                let mut file: Vec<getdata::Datum> = bincode::deserialize(&file)?;
                data.append(&mut file);
            }
            let bar = ProgressBar::new(check.tasks().len() as u64);
            bar.set_style(style);
            let mut done = Vec::new();
            for task in check.tasks_mut() {
                if !task.processed() {
                    config.set_geofips(task.key());
                    match getdata::get_data(&config).await {
                        Ok(res) => {
                            let mut results = res.results();
                            data.append(&mut results);
                            task.set_processed(true);
                            done.push(task);
                            bar.inc(1);
                        }
                        Err(_) => {
                            let encode: Vec<u8> = bincode::serialize(&done)?;
                            std::fs::write(checklist_update.clone(), encode)?;
                            let encode: Vec<u8> = bincode::serialize(&data)?;
                            std::fs::write(bea_data.clone(), encode)?;
                            info!("GeoFips {} failed to download.", task.key());

                        }
                    }
                }
            }
            let encode: Vec<u8> = bincode::serialize(&data)?;
            std::fs::write(bea_data.clone(), encode)?;
            let mut data = getdata::Data::new(&data);
            data.to_csv("p:/bea_cainc5n.csv".into())?;
            info!("Data download complete.");
            std::fs::remove_file(checklist_update)?;
        }
        _ => info!("Command not recognized."),
    };
    Ok(())
}

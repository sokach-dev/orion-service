use clap::{Parser, Subcommand};
use service::start_server;
use std::env;
use time::{macros::format_description, UtcOffset};
use tracing_subscriber::{fmt::time::OffsetTime, EnvFilter};

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "sokach-dev")]
struct Cli {
    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Rpc {
        /// orion rpc server config path, or through env `ORION_RPC_CONFIG`
        #[clap(short, long)]
        config: Option<String>,

        /// aes config salt, or through env `ORION_RPC_SALT`
        #[clap(short, long)]
        salt: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
    );
    tracing_subscriber::fmt()
        .with_timer(local_time)
        .with_env_filter(EnvFilter::builder().from_env_lossy())
        .with_line_number(true)
        .with_file(true)
        .init();

    let cli = Cli::parse();

    match &cli.subcmd {
        Commands::Rpc { config, salt } => {
            // start rpc service
            let config_path = config
                .clone()
                .unwrap_or_else(|| env::var("ORION_RPC_CONFIG").unwrap());

            let salt = salt
                .clone()
                .unwrap_or_else(|| env::var("ORION_RPC_SALT").unwrap_or_default());
            let salt = if salt.is_empty() { None } else { Some(salt) };

            let c = aes_config::ConfigInfo::new(config_path, salt, aes_config::ConfigType::TOML)
                .unwrap()
                .try_get_config::<abi::Config>()
                .unwrap();

            if let Err(e) = start_server(&c).await {
                tracing::error!("start server error: {}", e);
            }
        }
    }
}

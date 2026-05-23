use env_logger::Env;
use log::info;
use paladin::config::Settings;
use paladin::config::setup::setup_and_run;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "smartcontent-aggregator")]
struct Opt {
    #[structopt(short, long, default_value = "config.yml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file in development (debug builds only)
    // Production deployments should use proper secrets management
    #[cfg(debug_assertions)]
    {
        // Silently ignore if .env doesn't exist - not all dev setups need it
        let _ = dotenv::dotenv();
    }

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let opt = Opt::from_args();
    let config = match Settings::load_from_file(&opt.config) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to load configuration: {:?}", e);
            std::process::exit(1);
        }
    };

    info!("Loaded configuration: {:?}", config);

    setup_and_run(config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opt_default_config() {
        let opt = Opt::from_iter(&["test"]);
        assert_eq!(opt.config, "config.yml");
    }

    #[test]
    fn test_opt_custom_config() {
        let opt = Opt::from_iter(&["test", "--config", "custom.yml"]);
        assert_eq!(opt.config, "custom.yml");
    }

    #[test]
    fn test_opt_short_config() {
        let opt = Opt::from_iter(&["test", "-c", "short.yml"]);
        assert_eq!(opt.config, "short.yml");
    }
}

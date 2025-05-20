use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod db;
mod migration;
mod report;
mod utils;

use crate::config::Config;
use crate::migration::MigrationContext;
use crate::report::MigrationReport;
use crate::utils::ensure_directory_exists;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long)]
    config: String,

    /// Run in dry-run mode (no actual changes)
    #[arg(long)]
    dry_run: bool,

    /// Log level (debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Output directory for reports
    #[arg(short, long, default_value = "reports")]
    output_dir: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(match args.log_level.as_str() {
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO,
        })
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .with_thread_names(false)
        .with_ansi(true)
        .pretty()
        .init();

    info!("Starting database migration tool");
    info!("Configuration file: {}", args.config);
    info!("Dry run mode: {}", args.dry_run);

    // Create output directory
    let output_dir = PathBuf::from(&args.output_dir);
    ensure_directory_exists(&output_dir)?;

    // Load configuration
    let config = Config::from_file(&args.config)?;
    config.validate()?;

    // Initialize migration context
    let mut ctx = MigrationContext::new(config).await?;

    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
    );

    // Start migration process
    pb.set_message("Collecting metadata...");
    ctx.collect_metadata().await?;

    pb.set_message("Generating migration plan...");
    ctx.generate_migration_plan().await?;

    pb.set_message("Executing migration...");
    let mut report = MigrationReport::new();
    ctx.execute_migration(args.dry_run).await?;

    // Generate reports
    pb.set_message("Generating reports...");
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let html_report_path = output_dir.join(format!("migration_report_{}.html", timestamp));
    let json_report_path = output_dir.join(format!("migration_report_{}.json", timestamp));

    report.generate_html(&html_report_path)?;
    report.generate_json(&json_report_path)?;

    pb.finish_with_message("Migration completed successfully!");
    info!("Reports generated:");
    info!("  HTML: {}", html_report_path.display());
    info!("  JSON: {}", json_report_path.display());

    Ok(())
}

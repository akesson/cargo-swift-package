use camino::Utf8PathBuf;
use clap::Parser;

/// Compile a package into a cross-platform Apple XCFramework
#[derive(Debug, Parser)]
#[clap(version)]
pub struct Cli {
    /// Do not print cargo log messages
    #[arg(short, long)]
    pub quiet: bool,

    /// Package to build (see `cargo help pkgid`)
    #[arg(short, long)]
    pub package: Option<String>,

    /// Use verbose output (-vv very verbose/build.rs output)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Build artifacts in release mode, with optimizations
    #[arg(short, long)]
    pub release: bool,

    /// Build artifacts with the specified profile
    #[arg(long, value_name = "PROFILE-NAME")]
    pub profile: Option<String>,

    /// Space or comma separated list of features to activate
    #[arg(short, long)]
    pub features: Vec<String>,

    /// Activate all available features
    #[arg(long)]
    pub all_features: bool,

    /// Do not activate the `default` feature
    #[arg(long)]
    pub no_default_features: bool,

    /// Directory for all generated artifacts
    #[arg(long, value_name = "DIRECTORY")]
    pub target_dir: Option<Utf8PathBuf>,

    /// Path to Cargo.toml.
    #[arg(long, value_name = "PATH")]
    pub manifest_path: Option<Utf8PathBuf>,
}

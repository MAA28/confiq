use std::path::PathBuf;

use clap::Parser;

/// Confiq: A configuration to tool to make life easy
/// @author Malte Aschenbach
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(flatten)]
    pub global_opts: GlobalOpts,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub struct GlobalOpts {
    /// Verbose output mode
    #[clap(short, long)]
    pub verbose: bool,

    /// Logs to a file at /tmp/confiq.log
    #[clap(long)]
    pub log_to_file: bool,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Builds to configuration to bash
    Build(BuildArgs),
    /// Tests which scopes match your current environment
    Scopes(ScopesArgs),
    /// Lists configuration properties
    List(ListArgs),
}

#[derive(Parser, Debug)]
pub struct BuildArgs{
    /// Path to config file
    #[clap(long, default_value = "~/.config/confiq.toml")]
    pub path: PathBuf,

    /// Path to build directory
    #[clap(long, default_value = "~/.config/config/build")]
    pub build_path: PathBuf,

    /// Should aliases be build?
    #[clap(short, long)]
    pub aliases: bool,

    /// Should environment variables be build?
    #[clap(short, long)]
    pub environment_variables: bool 
}

#[derive(Parser, Debug)]
pub struct ScopesArgs {
    /// Path to config file
    #[clap(long, default_value = "~/.config/confiq.toml")]
    pub path: PathBuf,
}



#[derive(Parser, Debug)]
pub struct ListArgs {
    /// Path to config file
    #[clap(long, default_value = "~/.config/confiq.toml")]
    pub path: PathBuf,

    #[clap(subcommand)]
    pub command: ListCommand 
}

#[derive(Parser, Debug)]
pub enum ListCommand {
    /// Lists all aliases
    Aliases,
    /// Lists all enviroment variables
    EnvironmentVariables,
    /// Lists all scopes
    Scopes,
}


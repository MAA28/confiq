use std::path::PathBuf;

use clap::Parser;

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
    #[clap(short, long)]
    pub verbose: bool,

    #[clap(long)]
    pub log_to_file: bool,
}

#[derive(Parser, Debug)]
pub enum Command {
    Build(BuildArgs),
    Scopes(ScopesArgs),
    List(ListArgs),
}

#[derive(Parser, Debug)]
pub struct BuildArgs{
    #[clap(long)]
    pub path: Option<PathBuf>,

    #[clap(long)]
    pub aliases: bool 
}

#[derive(Parser, Debug)]
pub struct ScopesArgs {
    #[clap(long)]
    pub path: Option<PathBuf>,
}



#[derive(Parser, Debug)]
pub struct ListArgs {
    #[clap(long)]
    pub path: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: ListCommand 
}

#[derive(Parser, Debug)]
pub enum ListCommand {
    Aliases,
    Scopes,
}


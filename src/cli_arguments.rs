use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct FafArgs {
    #[arg(short, long)]
    pub command: String,

    #[arg(short, long)]
    pub location: String,

    #[arg(short, long, default_value_t = true)]
    pub recursive: bool,
}

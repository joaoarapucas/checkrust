use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub add: Option<String>,

    #[arg(short, long)]
    pub list: bool,

    #[arg(short, long)]
    pub check: Option<u16>,

    #[arg(short, long)]
    pub remove: Option<u16>,
}

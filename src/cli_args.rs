use clap::Parser;

#[derive(Parser)]
pub(crate) enum Cli {
    Rename {
        file_name: String,
        date: Option<String>,
        #[clap(long, short = 'm')]
        date_from_metadata: bool,
    },
    Touch {
        date: Option<String>,
    },
}

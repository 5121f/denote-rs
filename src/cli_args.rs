use clap::Parser;

#[derive(Parser)]
pub(crate) enum Cli {
    Rename {
        file_names: Vec<String>,
        /// You can use "now" to use current time
        #[clap(long, short, conflicts_with("date_from_metadata"))]
        date: Option<String>,
        #[clap(long, short = 'm', conflicts_with("date"))]
        date_from_metadata: bool,
        /// Accept all questions
        #[clap(long, short)]
        accept: bool,
        #[clap(long, short = 'k')]
        no_keywords: bool,
    },
    Touch {
        /// You can use "now" to use current time
        #[clap(long, short)]
        date: Option<String>,
    },
}

use clap::Parser;

#[derive(Parser)]
pub(crate) enum Cli {
    Rename {
        #[clap(required = true)]
        file_names: Vec<String>,
        /// Date of file creating. You can use "now" to use current time
        #[clap(long, short, conflicts_with("date_from_metadata"))]
        date: Option<String>,
        /// Use date from metadata
        #[clap(long, short = 'm', conflicts_with("date"))]
        date_from_metadata: bool,
        /// Accept all questions
        #[clap(long, short)]
        accept: bool,
        /// Don't use keywords
        #[clap(long, short = 'k')]
        no_keywords: bool,
    },
    Touch {
        /// Date of file creating. You can use "now" to use current time
        #[clap(long, short)]
        date: Option<String>,
    },
}

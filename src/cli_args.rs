use clap::Parser;

#[derive(Parser)]
pub(crate) enum Cli {
    Rename {
        file_name: String,
        date: Option<String>,
    },
    Touch {
        date: Option<String>,
    },
}

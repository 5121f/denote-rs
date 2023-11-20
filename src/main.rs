mod cli_args;
mod io;
mod name_scheme;

use anyhow::{bail, Context, Result};
use clap::Parser;
use cli_args::Cli;
use io::Io;
use name_scheme::{identifier::Identifier, title::Title, NameSchemeBuilder};
use std::{fs, path::PathBuf};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Rename { file_name, date } => {
            let path = PathBuf::from(&file_name);

            if !path.exists() {
                bail!("Указаного файла не существует.");
            }
            if !path.is_file() {
                bail!("Указан не файл.");
            }

            let extension = path.extension().and_then(|s| s.to_str()).map(String::from);
            let file_title = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            let title = Title::extract_from_string(&file_title)
                .map(|f| f.desluggify())
                .unwrap_or(file_title.to_owned());
            let identifier = if let Some(date) = date {
                Identifier::from_string(&date)?
            } else {
                Identifier::extract_from_string(&file_title)
                    .unwrap_or_else(Identifier::current_time)
            };

            let mut io = Io::new();

            let mut name_scheme_builder = NameSchemeBuilder::new()
                .identifier(identifier)
                .take_title_from_user_with_old_title(&mut io, &title)?
                .take_keywords_from_user(&mut io)?;

            if let Some(extention) = extension {
                name_scheme_builder = name_scheme_builder.extention(extention);
            }

            let new_file_name = name_scheme_builder.build().to_string();

            if file_name == new_file_name {
                println!("Действие не требуется.");
            } else {
                println!("Переименовать \"{}\" в \"{}\"", &file_name, new_file_name);
                if io.question("Подтвердить переименование?", true)? {
                    fs::rename(&path, new_file_name)
                        .with_context(|| format!("Не удалсоь переименовать файл {path:?}"))?;
                }
            }
        }
        Cli::Touch { date } => {
            let mut name_scheme_builder = NameSchemeBuilder::new();
            if let Some(date) = date {
                let identifier = Identifier::from_string(&date)?;
                name_scheme_builder = name_scheme_builder.identifier(identifier);
            };
            let mut io = Io::new();
            let file_name = name_scheme_builder
                .take_title_from_user(&mut io)?
                .take_keywords_from_user(&mut io)?
                .take_extention_from_user(&mut io)?
                .build()
                .to_string();

            if io.question(&format!("Создать файл \"{file_name}\"?"), true)? {
                fs::File::create(file_name).context("Не удалсоь создать файл.")?;
            }
        }
    }

    Ok(())
}

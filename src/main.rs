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
    let mut io = Io::new();

    match cli {
        Cli::Rename {
            file_names,
            date,
            date_from_metadata,
            accept,
            title_accept,
            no_keywords,
        } => {
            for file_name in file_names {
                rename_file(
                    file_name,
                    date.as_ref().map(|d| d.as_str()),
                    date_from_metadata,
                    accept,
                    title_accept,
                    no_keywords,
                    &mut io,
                )?;
            }
        }
        Cli::Touch { date } => {
            let mut name_scheme_builder = NameSchemeBuilder::new();

            let identifier = match date {
                Some(date) => Identifier::from_string(&date)?,
                None => Identifier::current_time(),
            };

            name_scheme_builder
                .identifier(identifier)
                .take_title_from_user(&mut io)?
                .take_keywords_from_user(&mut io)?
                .take_extention_from_user(&mut io)?;

            let file_name = name_scheme_builder.build().into_string();

            let accepted = io.question(&format!("Создать файл \"{file_name}\"?"), true)?;
            if accepted {
                fs::File::create(file_name).context("Не удалсоь создать файл.")?;
            }
        }
    }

    Ok(())
}

fn rename_file(
    file_name: String,
    date: Option<&str>,
    date_from_metadata: bool,
    accept: bool,
    title_accept: bool,
    no_keywords: bool,
    io: &mut Io,
) -> Result<()> {
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

    let identifier = if date_from_metadata {
        Identifier::from_file_metadata(&path)?
    } else if let Some(date) = date {
        Identifier::from_string(&date)?
    } else {
        Identifier::extract_from_string(&file_title).unwrap_or_else(|_| Identifier::current_time())
    };

    let mut name_scheme_builder = NameSchemeBuilder::new();
    name_scheme_builder.identifier(identifier);

    if title_accept {
        let title = Title::find_in_string(&file_title).or_else(|_| Title::parse(&file_title))?;
        name_scheme_builder.title(title);
    } else {
        let title = Title::find_in_string(&file_title)
            .map(|title| title.desluggify())
            .unwrap_or(file_title);
        name_scheme_builder.take_title_from_user_with_old_title(io, &title)?;
    }

    if !no_keywords {
        name_scheme_builder.take_keywords_from_user(io)?;
    }

    if let Some(extention) = extension {
        name_scheme_builder.extention(extention);
    }

    let new_file_name = name_scheme_builder.build().into_string();

    if file_name == new_file_name {
        println!("Действий не требуется");
        return Ok(());
    }

    if !accept {
        println!("Переименовать \"{}\" в \"{}\"", &file_name, new_file_name);
        let accepted = io.question("Вы подтверждаете?", true)?;
        if !accepted {
            return Ok(());
        }
    }

    fs::rename(&path, new_file_name)
        .with_context(|| format!("Не удалсоь переименовать файл {path:?}"))?;

    Ok(())
}

use anyhow::{Context, Error, Result, anyhow};
use calcard::{Entry, Parser as CalParser, jscalendar::JSCalendar, jscontact::JSContact};
use clap::Parser;
use clap_derive::ValueEnum;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

#[derive(ValueEnum, Clone, Debug)]
pub enum FileType {
    VCard,
    ICalendar,
    JSCalendar,
    JSContact,
}

impl TryFrom<&str> for FileType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ical" | "ics" | "ifb" | "icalendar" => Ok(FileType::ICalendar),
            "vcf" | "vcard" => Ok(FileType::VCard),
            _ => Err(anyhow!("Can't determine type from file extension")),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input filepath
    ///
    /// If none is specified, it'll
    /// read the data from stdin
    #[arg(short, long)]
    pub input: Option<PathBuf>,

    /// Output filepath
    ///
    /// If none is specified, it'll
    /// send the data into stdout
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Source filetype
    ///
    /// TODO: Autodetection
    #[arg(short, long)]
    pub source_type: Option<FileType>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input = match args.input {
        Some(path) => fs::read_to_string(path).context(anyhow!("Failed to read input file"))?,
        None => io::read_to_string(io::stdin()).context(anyhow!("Failed to read from stdin"))?,
    };

    // TODO: Auto-detect source_type
    let source_type = args
        .source_type
        .context(anyhow!("Source type not specified"))?;

    let mut result = String::new();

    match source_type {
        FileType::ICalendar => {
            let mut parser = CalParser::new(&input);
            let mut calendars = Vec::new();
            loop {
                match parser.entry() {
                    Entry::ICalendar(cal) => calendars.push(cal),
                    Entry::Eof => break,
                    // TODO: Better error
                    _ => return Err(anyhow!("Tried to parse iCalendar but failed")),
                }
            }

            for cal in calendars {
                result.push_str(&cal.into_jscalendar::<String, String>().to_string_pretty());
            }
        }
        FileType::VCard => {
            let mut parser = CalParser::new(&input);
            let mut cards = Vec::new();
            loop {
                match parser.entry() {
                    Entry::VCard(vcard) => cards.push(vcard),
                    Entry::Eof => break,
                    // TODO: Better error
                    _ => return Err(anyhow!("Tried to parse vCard but failed")),
                }
            }

            for card in cards {
                result.push_str(&card.into_jscontact::<String, String>().to_string_pretty());
            }
        }
        FileType::JSCalendar => {
            let jscalendar = JSCalendar::<String, String>::parse(&input)
                .map_err(|e| anyhow!("Failed to parse JSCalendar: {}", e))?;

            result.push_str(
                &jscalendar
                    .into_icalendar()
                    .ok_or(anyhow!("Failed to convert JSCalendar into iCalendar"))?
                    .to_string(),
            );
        }
        FileType::JSContact => {
            let jscontact = JSContact::<String, String>::parse(&input)
                .map_err(|e| anyhow!("Failed to parse JSContact: {}", e))?;

            result.push_str(
                &jscontact
                    .into_vcard()
                    .ok_or(anyhow!("Failed to convert JSContact into vCard"))?
                    .to_string(),
            );
        }
    }

    match args.output {
        Some(path) => {
            // TODO: Better error messages
            let mut f = File::create(path).context(anyhow!("Failed to create file"))?;
            f.write_all(result.as_bytes())
                .context(anyhow!("Failed to write data into file"))?;
        }
        // Print to stdout if no output path is provided
        None => println!("{}", result),
    }

    Ok(())
}

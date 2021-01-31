#![feature(test)]
#![feature(proc_macro_hygiene, decl_macro)]

use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

#[macro_use]
extern crate rocket;

use futures::StreamExt;
use glob;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufWriter};

mod cli;
mod consts;
mod errors;
mod ncda;
mod ws;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = cli::cli();

    if let Some(ref matches) = matches.subcommand_matches("check") {
        let workers = set_workers(matches.value_of("workers"));

        if let Some(files) = matches.values_of("input") {
            let files = file_list(files)?;

            let cb = move |input: &str, file_path: &PathBuf, line_number: usize| -> String {
                match ncda::check(&input) {
                    Ok(()) => format!("{}:{}  {:<31} ok", file_path.display(), line_number, &input),
                    Err(e) => format!(
                        "{}:{}  {:<31} {}",
                        file_path.display(),
                        line_number,
                        &input,
                        e.to_string()
                    ),
                }
            };

            async_file_processing(files, workers, cb, matches.value_of("output")).await?;
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("checksum") {
        let workers = set_workers(matches.value_of("workers"));

        if let Some(files) = matches.values_of("input") {
            let files = file_list(files)?;

            let cb = move |input: &str, file_path: &PathBuf, line_number: usize| -> String {
                match ncda::checksum(&input) {
                    Ok(check_char) => format!(
                        "{}:{}  {}{}",
                        file_path.display(),
                        line_number,
                        &input,
                        check_char
                    ),
                    Err(e) => format!(
                        "{}:{}  {:<31} {}",
                        file_path.display(),
                        line_number,
                        &input,
                        e.to_string()
                    ),
                }
            };

            async_file_processing(files, workers, cb, matches.value_of("output")).await?;
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("ws") {
        ws::launch_ws(matches.value_of("port").map(|v| {
            v.parse::<u16>()
                .expect(&format!("{} is not a valid port number", v))
        }))?;
    }

    Ok(())
}

fn file_list<'a, S: Iterator<Item = &'a str>>(input: S) -> Result<Vec<PathBuf>, glob::GlobError> {
    let patterns = input
        .into_iter()
        .map(|file_path| {
            glob::glob(&file_path)
                .into_iter()
                .flat_map(|path| path)
                .collect::<Result<Vec<PathBuf>, glob::GlobError>>()
        })
        .collect::<Result<Vec<Vec<PathBuf>>, glob::GlobError>>()?;

    Ok(patterns
        .into_iter()
        .flat_map(|file_path| file_path)
        .filter(|file_path| file_path.is_file())
        .collect::<Vec<PathBuf>>())
}

async fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename.as_ref()).await?;
    Ok(io::BufReader::new(file).lines())
}

async fn async_file_processing(
    input: Vec<PathBuf>,
    workers: usize,
    cb: impl FnOnce(&str, &PathBuf, usize) -> String + Copy,
    output: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    futures::stream::iter({
        input.into_iter().map(|entry| async move {
            let mut writer: BufWriter<Box<dyn AsyncWrite + Unpin>> = match output {
                Some(output_file) => BufWriter::new(Box::new(
                    File::create(output_file)
                        .await
                        .expect("failed to create output file"),
                )),
                None => BufWriter::new(Box::new(io::stdout())),
            };

            let mut lines = read_lines(&entry).await.unwrap();

            let mut line_number: usize = 0;

            while let Some(line_content) = lines.next_line().await.expect("failed to get next line")
            {
                let mut r = cb(&line_content, &entry, line_number.clone());
                line_number += 1;
                r.push_str("\n");

                writer
                    .write_all(&mut r.as_bytes())
                    .await
                    .expect("failed to write data")
            }

            writer.shutdown().await.expect("failed to write data");
        })
    })
    .buffer_unordered(workers)
    .collect::<Vec<()>>()
    .await;

    Ok(())
}

fn set_workers(input: Option<&str>) -> usize {
    match input {
        Some(workers) => workers
            .parse::<usize>()
            .expect(&format!("{} is not a valid number of workers", workers)),
        None => consts::WORKERS,
    }
}

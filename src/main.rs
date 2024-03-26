use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use indicatif::ProgressIterator;
use regex::{Captures, Regex};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_parser = Regex::new)]
    regex: Regex,

    #[arg(short, long, default_value = ".")]
    path: PathBuf,
}

struct HitInfo {
    path: PathBuf,
    lines: Vec<String>,
}

fn main() -> Result<()> {
    let Args { regex, path } = Args::parse();

    let mut target_files = Vec::new();
    rec_find_target_files(&path, &mut target_files)
        .with_context(|| format!("{}@{}", line!(), file!()))?;

    println!("{} files found. Search start.", target_files.len());

    let hit_infos = target_files
        .into_iter()
        .progress()
        .map(|p| search(&p, &regex))
        .collect::<Result<Vec<_>>>()
        .with_context(|| format!("{}@{}", line!(), file!()))?;

    let hit_infos = hit_infos
        .into_iter()
        .filter(|h| !h.lines.is_empty())
        .collect::<Vec<_>>();

    println!("Hit file num: {}", hit_infos.len());

    let total_hit_num: usize = hit_infos.iter().map(|h| h.lines.len()).sum();

    println!("Total hits: {}", total_hit_num);

    for hit_info in hit_infos {
        println!("{}", hit_info.path.display());
        for line in hit_info.lines {
            println!("  {}", line);
        }
    }

    Ok(())
}

fn rec_find_target_files(p: &Path, result: &mut Vec<PathBuf>) -> Result<()> {
    if p.is_dir() {
        for entry in p.read_dir()? {
            let entry = entry.with_context(|| format!("{}@{}", line!(), file!()))?;
            let path = entry.path();
            rec_find_target_files(&path, result)
                .with_context(|| format!("{}@{}", line!(), file!()))?;
        }
    } else if p.is_file() && p.extension() == Some("pdf".as_ref()) {
        result.push(p.to_path_buf());
    }

    Ok(())
}

fn search(p: &Path, r: &Regex) -> Result<HitInfo> {
    let txt_path = p.with_extension("txt");

    if !txt_path.exists() {
        Command::new("pdftotext")
            .arg(p)
            .output()
            .with_context(|| format!("{}@{}", line!(), file!()))?;
    }

    let content = fs::read_to_string(&txt_path)?;

    let lines = content
        .lines()
        .into_iter()
        .filter_map(|line| {
            if !r.is_match(line) {
                return None;
            }

            let line = r
                .replace_all(line, |caps: &Captures| format!("{}", &caps[0].red()))
                .to_string();

            Some(line)
        })
        .collect::<Vec<_>>();

    Ok(HitInfo {
        path: p.to_path_buf(),
        lines,
    })
}

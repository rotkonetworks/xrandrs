use anyhow::{Context, Result};
use clap::{App, Arg};
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::process;

#[derive(Deserialize)]
struct Config {
    app: AppConfig,
    rules: Vec<ZoomRule>,
}

#[derive(Deserialize)]
struct AppConfig {
    name: String,
    version: String,
    author: String,
    about: String,
}

#[derive(Deserialize)]
struct ZoomRule {
    min_aspect_ratio: f64,
    max_aspect_ratio: f64,
    min_width: u32,
    zoom_level: String,
}

fn main() -> Result<()> {
    let default_config_toml = include_str!("config.toml");
    let config: Config = toml::from_str(default_config_toml)?;

    let app_name = config.app.name.clone();
    let app_version = config.app.version.clone();
    let app_author = config.app.author.clone();

    let matches = App::new(app_name)
        .version(app_version.as_str())
        .author(app_author.as_str())
        .about(config.app.about.as_str())
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config = if let Some(config_file) = matches.value_of("config") {
        toml::from_str(
            &fs::read_to_string(config_file).context("Failed to read the configuration file")?,
        )
        .context("Failed to parse the configuration file")?
    } else {
        config
    };

    // pub fn get_connected_output() -> Result<String, Box<dyn Error>> {
    //     let xrandr_output = Command::new("xrandr").output()?.stdout;
    //     let xrandr_output = String::from_utf8(xrandr_output)?;
    //
    //     let re = Regex::new(r"(?P<output_name>\w+-\w+-\d+) connected")?;
    //     let caps = re
    //         .captures(&xrandr_output)
    //         .ok_or("No connected output found")?;
    //
    //     let output_name = caps.name("output_name").unwrap().as_str().to_string();
    //     Ok(output_name)
    // }

    let xrandr_output = run_command("xrandr")?;
    let connected_re = Regex::new(r"(?P<output_name>\w+-\w+-\d+) connected")?;
    let resolution_re = Regex::new(r"(\d+)x(\d+)")?;

    let (display, width, height) =
        parse_display_info(&xrandr_output, &connected_re, &resolution_re)?;
    let zoom_level = calculate_zoom_level(width, height, &config.rules);

    run_command_with_args(
        "xrandr",
        &["--output", display, "--scale", &zoom_level, "--auto"],
    )?;
    Ok(())
}

fn parse_display_info<'a>(
    output: &'a str,
    connected_re: &Regex,
    resolution_re: &Regex,
) -> Result<(&'a str, u32, u32)> {
    output
        .lines()
        .filter_map(|line| {
            let output_name = connected_re.captures(line)?.name("output_name")?.as_str();
            let captures = resolution_re.captures(line)?;
            let width = captures[1].parse().ok()?;
            let height = captures[2].parse().ok()?;
            Some((output_name, width, height))
        })
        .next()
        .context("Failed to find connected display")
}

fn run_command(cmd: &str) -> Result<String> {
    let output = process::Command::new(cmd)
        .output()
        .context(format!("Failed to execute {}", cmd))?;

    String::from_utf8(output.stdout).context(format!("Failed to decode {} output", cmd))
}

fn run_command_with_args(cmd: &str, args: &[&str]) -> Result<()> {
    process::Command::new(cmd)
        .args(args)
        .status()
        .context(format!("Failed to execute {} with arguments", cmd))?;
    Ok(())
}

fn calculate_zoom_level(width: u32, height: u32, rules: &[ZoomRule]) -> String {
    let aspect_ratio = width as f64 / height as f64;

    for rule in rules {
        if aspect_ratio >= rule.min_aspect_ratio
            && aspect_ratio <= rule.max_aspect_ratio
            && width >= rule.min_width
        {
            return rule.zoom_level.clone();
        }
    }

    "1x1".to_string()
}

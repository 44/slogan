#![allow(unused)]

use clap::Parser;
use log::*;
use std::fs;

extern crate ini;
use ini::Ini;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cfg {
    #[arg(short, long)]
    manifest: Option<String>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    follow: bool,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    index: bool,
    #[arg(short, long)]
    config: Option<String>,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(short, long)]
    last: Option<String>,
    #[arg(short, long)]
    include: Vec<String>,
    #[arg(short, long)]
    exclude: Vec<String>,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    passthru: bool,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    color: bool,
    #[arg(long="no-manifest", action = clap::ArgAction::SetTrue)]
    no_manifest: bool,
    #[arg(short, long)]
    timerange: Option<String>,
    #[arg(short, long)]
    key: Option<String>,

    logsources: Vec<String>,

}

pub fn main() {
    let args = Cfg::parse();
    println!("Hello world: {} -> manifest '{:?}', verbosity={}, {:?}", args.logsources[0], args.manifest, args.verbose, args);
    stderrlog::new()
        .module(module_path!())
        .verbosity(usize::from(args.verbose) + 1)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();
    info!("Parsing: {:?}", args.logsources);
    debug!("Config: {:?}", args);

    match args.config {
        Some(cfg_file) => {
            let from_file = Ini::load_from_file(cfg_file).unwrap();
            for (sec, prop) in &from_file {
                debug!("Config file section: {:?}", sec);
                for (k, v) in prop.iter() {
                    debug!("  {:?} : {:?}", k, v);
                }
            }
        },
        None => {}
    }


}

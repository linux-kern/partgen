// https://transform.tools/json-to-rust-serde
//#![allow(unused)]
use serde::{Deserialize, Serialize};
use std::io::prelude::*;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use clap::Parser;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub logicalname: String,
    pub partitions: Vec<Pation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pation {
    pub id: String,
    #[serde(rename = "first_sector")]
    pub first_sector: String,
    #[serde(rename = "last_sector")]
    pub last_sector: String,
    pub filesystem: String,
}

fn load_disks_from_file<P: AsRef<Path>>(path: P) -> Result<Root, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let r: Root = serde_json::from_reader(reader)?;

    Ok(r)
}

/// Toolkit To generate a partition script
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the input file
    #[clap(short, long, default_value = "partitions.yaml")]
    input: String,

    /// Name of the output file
    #[clap(short, long, default_value = "setup_partitions.sh")]
    output: String,
}

fn main() {
    let args = Args::parse();

    let disks = load_disks_from_file(args.input).unwrap();
    let mut file = File::create(args.output).unwrap();

    // OP$
    // Partion ID$
    // Last Sector$
    // Filesystem type$
    // Write OP$
    // Yes to save$
    for disk in disks {
        let mut script = format!("{}\n\ngdisk {} << EOF", "#!/bin/bash -e", disk.logicalname);
        for p in disk.partitions {
            script = format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n",
                script, "n", p.id, p.first_sector, p.last_sector, p.filesystem
            );
        }
        script = format!("{}w\nY\nEOF\n", script);
        file.write_all(script.as_bytes()).unwrap();
    }
}

use std::{fs::{self, File}, io::{BufReader, BufRead}, process};
use urlencoding;
use crate::utils::terminal::*;

pub fn parse_osrelease() -> Option<Vec<Vec<String>>> {
	if fs::metadata("/etc/os-release").is_err() {
		return None;
	}

	let mut osrelease = Vec::new();
	let osreleasefilebuf = BufReader::new(File::open("/etc/os-release").expect("Failed to open /etc/os-release"));

	for line in osreleasefilebuf.lines().flatten() {
		let line = line.split("=").map(|s| s.to_string()).collect::<Vec<String>>();
		osrelease.push(line);
	}
	
	Some(osrelease)
}

pub fn is_running_on_steamos() -> bool { // Check if we are running SteamOS
	let osrelease = parse_osrelease();
	if osrelease.is_none() {
		warning!("Unable to find /etc/os-release, assuming not SteamOS");
		false;
	}
	let mut is_steamos = false;

	for line in osrelease.unwrap() {
		if line[0] == "ID" {
			if line[1] == "steamos" {
				is_steamos = true;
			}
		}
	}

	is_steamos
}

pub fn add_item_to_steam_library(path: String) {
	let name: BufReader<File> = BufReader::new(File::open(path.clone()).expect("Failed to open"));
	let application_name = name.lines().flatten().collect::<Vec<String>>()[1].replace("Name=", "");
	let url_encoded_path = urlencoding::encode(&path);

	// Took me, over 6 hours to figure this out, apparently this is meant to prevent browser abuse
	fs::write("/tmp/addnonsteamgamefile", "").expect("Failed to write to /tmp/addnonsteamgamefile");

	match process::Command::new("steam")
		.arg(format!("steam://addnonsteamgame/{}", url_encoded_path))
		.stderr(process::Stdio::null())
		.stdout(process::Stdio::null())
		.spawn() {
		Ok(_) => {
			success!("Added {application_name} to Steam library");
		},
		Err(_) => {
			warning!("Failed to add {application_name} to Steam library!");
		}
	}
}

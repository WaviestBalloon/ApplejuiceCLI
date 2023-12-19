use std::{fs::{self, File}, io::{BufReader, BufRead}, process, env};
use urlencoding;
use crate::utils::{terminal::*, setup};

pub fn parse_osrelease() -> Option<Vec<Vec<String>>> {
	if fs::metadata("/etc/os-release").is_err() {
		return None;
	}

	let osreleasefilebuf = BufReader::new(File::open("/etc/os-release").expect("Failed to open /etc/os-release"));
	let mut osrelease = Vec::new();

	for line in osreleasefilebuf.lines().flatten() {
		let line = line.split("=").map(|s| s.to_string()).collect::<Vec<String>>();
		osrelease.push(line);
	}
	
	Some(osrelease)
}

pub fn get_fullscreen_value_from_rbxxml() -> Option<String> {
	let xml_location = format!("{}/prefixdata/pfx/drive_c/users/steamuser/AppData/Local/Roblox/GlobalBasicSettings_13.xml", setup::get_applejuice_dir());
	if fs::metadata(xml_location.clone()).is_err() {
		return None;
	}
	let rbxxml = fs::read_to_string(xml_location).expect("Failed to read Roblox GlobalBasicSettings_13.xml");
	let rbxxml = rbxxml.split("\n").collect::<Vec<&str>>();
	let mut fullscreen_value = None;

	for line in rbxxml {
		if line.contains("Fullscreen") {
			let line = line.split(">").collect::<Vec<&str>>()[1];
			let line = line.split("<").collect::<Vec<&str>>()[0];
			fullscreen_value = Some(line.to_string());
		}
	}

	fullscreen_value
}

pub fn set_rbx_fullscreen_value(toggle: bool) {
	let rbxxml = fs::read_to_string(format!("{}/prefixdata/pfx/drive_c/users/steamuser/AppData/Local/Roblox/GlobalBasicSettings_13.xml", setup::get_applejuice_dir())).expect("Failed to read Roblox GlobalSettings.xml");
	let rbxxml = rbxxml.split("\n").collect::<Vec<&str>>();
	let mut new_rbxxml = String::new();

	for line in rbxxml {
		if line.contains("Fullscreen") {
			let line = line.split(">").collect::<Vec<&str>>()[0];
			let line = format!("{line}>{toggle}</bool>");
			new_rbxxml.push_str(&line);
		} else {
			new_rbxxml.push_str(&line);
		}
		new_rbxxml.push_str("\n");
	}

	fs::write(format!("{}/prefixdata/pfx/drive_c/users/steamuser/AppData/Local/Roblox/GlobalBasicSettings_13.xml", setup::get_applejuice_dir()), new_rbxxml).expect("Failed to write to Roblox GlobalSettings.xml");
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

pub fn is_running_deck_big_picture_mode() -> bool {
	if env::var("SteamOS").unwrap_or_default() == "1" && env::var("SteamGamepadUI").unwrap_or_default() == "1" {
		return true;
	}
	
	false
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

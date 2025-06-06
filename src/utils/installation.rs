use std::borrow::Cow;
use std::process::exit;
use std::{process, fs, path, io, thread, thread::available_parallelism};
use crate::utils::terminal::*;
use crate::setup;
use rbxdd::rbxcdn::Binary;
use rbxdd::{appsettings, bindings, rbxcdn};

const LIVE_DEPLOYMENT_CDN: &str = "https://setup.rbxcdn.com/";
const CHANNEL_DEPLOYMENT_CDN: &str = "https://roblox-setup.cachefly.net/channel/";

pub struct ExactVersion<'a> {
	pub channel: Cow<'a, str>,
	pub hash: Cow<'a, str>
}

impl<'a> ExactVersion<'a> {
	pub fn new(channel: impl Into<Cow<'a, str>>, hash: impl Into<Cow<'a, str>>) -> Self {
		Self {channel: channel.into(), hash: hash.into()}
	}
}

pub struct LatestVersion<'a> {
	pub channel: Cow<'a, str>,
	pub binary: Cow<'a, str>
}

impl<'a> LatestVersion<'a> {
	pub fn new(channel: impl Into<Cow<'a, str>>, binary: impl Into<Cow<'a, str>>) -> Self {
		Self {channel: channel.into(), binary: binary.into()}
	}
}

pub enum Version<'a> {
	Exact(ExactVersion<'a>),
	Latest(LatestVersion<'a>)
}

impl<'a> Version<'a> {
	pub fn exact(channel: impl Into<Cow<'a, str>>, hash: impl Into<Cow<'a, str>>) -> Self {
		Self::Exact(ExactVersion::new(channel, hash))
	}

	pub fn latest(channel: impl Into<Cow<'a, str>>, binary: impl Into<Cow<'a, str>>) -> Self {
		Self::Latest(LatestVersion::new(channel, binary))
	}

	pub fn fetch_latest(self) -> ExactVersion<'a> {
		match self {
			Self::Latest(latest) => fetch_latest_version(latest),
			Self::Exact(exact) => exact
		}
	}
}

pub fn get_latest_version_hash(binary: &str, channel: &str) -> String {
	fetch_latest_version(LatestVersion {
		channel: Cow::Borrowed(channel),
		binary: Cow::Borrowed(binary)
	}).hash.into_owned()
}

pub fn fetch_latest_version(version: LatestVersion) -> ExactVersion {
	let LatestVersion {channel, binary} = version;

	let required_binary: Binary = match &*binary.to_lowercase() {
		"player" => Binary::Player,
		"studio" => Binary::Studio,
		_ => {
			error!("Unknown binary type {:?}", binary);
			exit(1);
		}
	};

	let hash = match rbxcdn::get_latest_version(required_binary, None) {
		Ok(hash) => hash,
		Err(err) => {
			error!("Failed to get latest version: {:?}", err);
			exit(1);
		}
	};

	success!("Resolved hash to {}", hash);
	ExactVersion {channel, hash: Cow::Owned(hash)}
}

pub fn get_binary_type(package_manifest: Vec<&str>) -> &str {
	let mut binary: &str = "";
	for package in package_manifest {
		let package_str = package.to_string();
		if package_str.contains("RobloxApp.zip") {
			binary = "Player";
			break;
		} else if package_str.contains("RobloxStudio.zip") {
			binary = "Studio";
			break;
		}
	}
	if binary.is_empty() {
		error!("Could not determine binary type for provided package manifest!");
		exit(1);
	}

	binary
}

pub fn write_appsettings_xml(path: String) { // spaghetti
	fs::write(format!("{}/AppSettings.xml", path), appsettings::XML_DATA).expect("Failed to write AppSettings.xml");
}

pub fn download_deployment(binary: &str, version_hash: String, channel: &str) -> String {
	let root_path = setup::get_applejuice_dir();
	let temp_path = format!("{}/cache/{}-download", root_path, version_hash);

	if setup::confirm_existence(&temp_path) {
		warning!("{} is already downloaded. Skipping download. Use --purge cache to delete previously downloaded files.", version_hash);
		return temp_path;
	}
	setup::create_dir(&format!("cache/{}-download", version_hash));
	success!("Constructed cache directory");
	status!("Downloading deployment...");
	status!("Using cache directory: {temp_path}");
	
	let bindings: &[_] = if binary == "Player" { &bindings::PLAYER_EXTRACT_BINDINGS } else { &bindings::STUDIO_EXTRACT_BINDINGS };
	let deployment_channel = if channel == "LIVE" { LIVE_DEPLOYMENT_CDN.to_string() } else { format!("{CHANNEL_DEPLOYMENT_CDN}{channel}/") };

	status!("Using deployment CDN URL: {}", deployment_channel);
	status!("{} files will be downloaded", bindings.len());

	let client = reqwest::blocking::Client::new();
	progress_bar::init_progress_bar_with_eta(bindings.len());
	for (package, _path) in bindings.iter() {
		progress_bar::print_progress_bar_info("•", format!("Downloading {package}... ({version_hash}-{package})").as_str(), progress_bar::Color::Blue, progress_bar::Style::Bold);

		let mut response = client.get(format!("{}{}-{}", deployment_channel, version_hash, package)).send().unwrap();
		if !response.status().is_success() {
			warning!("Failed to download {} from CDN! Status code: {}", package, response.status());
			continue;
		}
		let path: path::PathBuf = format!("{}/{}", temp_path, package).into();
		fs::create_dir_all(path.parent().unwrap()).unwrap();
		let mut file = fs::File::create(path).unwrap();
		io::copy(&mut response, &mut file).unwrap();

		progress_bar::inc_progress_bar();
	}

	progress_bar::finalize_progress_bar();
	success!("All compressed files downloaded, expanding files...");
	temp_path // Return the cache path to continue with extraction
}

pub fn extract_deployment_zips(binary: &str, temp_path: String, extraction_path: String, disallow_multithreading: bool) {
	let bindings: &[_] = if binary == "Player" { &bindings::PLAYER_EXTRACT_BINDINGS } else { &bindings::STUDIO_EXTRACT_BINDINGS };
	help!("{} files will be extracted", bindings.len());

	progress_bar::init_progress_bar_with_eta(bindings.len());

	if disallow_multithreading {
		for (package, path) in bindings.iter() {
			progress_bar::print_progress_bar_info("•", format!("Extracting {package}...").as_str(), progress_bar::Color::Blue, progress_bar::Style::Bold);
	
			if setup::confirm_existence(&format!("{}/{}", extraction_path, path)) && !path.is_empty() {
				progress_bar::print_progress_bar_info("!", format!("{} is already extracted. Skipping extraction.", package).as_str(), progress_bar::Color::LightYellow, progress_bar::Style::Bold);
				continue;
			}
			if !path.is_empty() { // Create directory if it doesn't exist during extraction
				progress_bar::print_progress_bar_info("•", format!("Creating path for {}/{}", extraction_path, path).as_str(), progress_bar::Color::Blue, progress_bar::Style::Bold);
				setup::create_dir(&format!("{}/{}", extraction_path, path));
			}
			process::Command::new("unzip")
				.arg(format!("{}/{}", temp_path, package))
				.arg("-d")
				.arg(format!("{}/{}", extraction_path, path))
				.output()
				.expect("Failed to execute unzip command");
	
			progress_bar::inc_progress_bar();
		}
	} else {
		let threads_available = available_parallelism().unwrap();
		let chunked_files = bindings.chunks((bindings.len() + threads_available.get() - 1) / threads_available);
		let mut threads = vec![];

		let indentation = status!("Multi-threading is enabled");
		help!("You can disable this with --nothreads");
		help!("{} threads available, {} chunks created from bindings", threads_available, chunked_files.size_hint().0);
		drop(indentation);

		for chunk in chunked_files {
			let extract_bind = extraction_path.clone();
			let temp_path_bind = temp_path.clone();
			let indentation = LogContext::get_indentation();
			threads.push(thread::spawn(move || {
				LogContext::set_indentation(indentation);
				for (package, path) in chunk.iter() {
					if setup::confirm_existence(&format!("{}/{}", extract_bind, path)) && !path.is_empty() {
						warning!("Skipping extracting {}", package);
						continue;
					}
					if !path.is_empty() { // Create directory if it doesn't exist during extraction
						setup::create_dir(&format!("{}/{}", extract_bind, path));
					}
					process::Command::new("unzip")
						.arg(format!("{}/{}", temp_path_bind, package))
						.arg("-d")
						.arg(format!("{}/{}", extract_bind, path))
						.output()
						.expect("Failed to execute unzip command");
					success!("Extracted {}", package);
				}
			}));
		}

		for thread in threads { // Wait for all threads to finish
			let _ = thread.join();
		}
	}

	progress_bar::finalize_progress_bar();
}

pub fn get_package_manifest(version_hash: String, channel: String) -> String {
	let channel = if channel == "LIVE" { "".to_string() } else { format!("channel/{}/", channel) };
	let url = format!("{LIVE_DEPLOYMENT_CDN}{channel}{version_hash}-rbxPkgManifest.txt");
	let client = reqwest::blocking::Client::new();
	let output = client.get(url.clone())
		.send()
		.expect("Failed to get the latest available version hash.")
		.text()
		.unwrap();

	if output.contains("AccessDenied") {
		error!("Recieved AccessDenied response from server when getting rbxPkgManifest, the version hash is probably invalid.\nResponse: {}\nVersion hash: {}\nFull URL: {}", output, version_hash, url);
		exit(1);
	} else if output.contains("Error") {
		error!("Unexpected server response when getting the rbxPkgManifest information.\nResponse: {}", output);
		exit(1);
	}

	output
}

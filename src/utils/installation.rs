use std::borrow::Cow;
use std::process::exit;
use std::{process, fs, path, io, thread, thread::available_parallelism};
use crate::utils::terminal::*;
use crate::setup;
use serde::Deserialize;
use serde_json::from_str;
use reqwest::blocking::get;

const LATEST_VERSION_PLAYER_CHANNEL: &str = "https://clientsettings.roblox.com/v2/client-version/WindowsPlayer/channel/";
const LATEST_VERSION_STUDIO_CHANNEL: &str = "https://clientsettings.roblox.com/v2/client-version/WindowsStudio64/channel/";
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
	client_version_upload: String
}

/* Response error handling for fetch_latest_version fn */
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseErrorMeat {
	code: i32,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseError {
	errors: Vec<ResponseErrorMeat>
}

const PLAYER_EXTRACT_BINDINGS: [(&str, &str); 20] = [
	("RobloxApp.zip", ""),
	("redist.zip", ""),
	("shaders.zip", "shaders/"),
	("ssl.zip", "ssl/"),
	("WebView2.zip", ""),
	// ("WebView2RuntimeInstaller.zip", ""), (Unnecessary)
	("content-avatar.zip", "content/avatar/"),
	("content-configs.zip", "content/configs/"),
	("content-fonts.zip", "content/fonts/"),
	("content-sky.zip", "content/sky/"),
	("content-sounds.zip", "content/sounds/"),
	("content-textures2.zip", "content/textures/"),
	("content-models.zip", "content/models/"),
	("content-textures3.zip", "PlatformContent/pc/textures/"),
	("content-terrain.zip", "PlatformContent/pc/terrain/"),
	("content-platform-fonts.zip", "PlatformContent/pc/fonts/"),
	("extracontent-luapackages.zip", "ExtraContent/LuaPackages/"),
	("extracontent-translations.zip", "ExtraContent/translations/"),
	("extracontent-models.zip", "ExtraContent/models/"),
	("extracontent-textures.zip", "ExtraContent/textures/"),
	("extracontent-places.zip", "ExtraContent/places/")
];
const STUDIO_EXTRACT_BINDINGS: [(&str, &str); 31] = [
	("RobloxStudio.zip", ""),
	("redist.zip", ""),
	("Libraries.zip", ""),
	("LibrariesQt5.zip", ""),
	("WebView2.zip", ""),
	// ("WebView2RuntimeInstaller.zip", ""), (Unnecessary)
	("shaders.zip", "shaders/"),
	("ssl.zip", "ssl/"),
	("Qml.zip", "Qml/"),
	("Plugins.zip", "Plugins/"),
	("StudioFonts.zip", "StudioFonts/"),
	("BuiltInPlugins.zip", "BuiltInPlugins/"),
	("ApplicationConfig.zip", "ApplicationConfig/"),
	("BuiltInStandalonePlugins.zip", "BuiltInStandalonePlugins/"),
	("content-qt_translations.zip", "content/qt_translations/"),
	("content-sky.zip", "content/sky/"),
	("content-fonts.zip", "content/fonts/"),
	("content-avatar.zip", "content/avatar/"),
	("content-models.zip", "content/models/"),
	("content-sounds.zip", "content/sounds/"),
	("content-configs.zip", "content/configs/"),
	("content-api-docs.zip", "content/api_docs/"),
	("content-textures2.zip", "content/textures/"),
	("content-studio_svg_textures.zip", "content/studio_svg_textures/"),
	("content-platform-fonts.zip", "PlatformContent/pc/fonts/"),
	("content-terrain.zip", "PlatformContent/pc/terrain/"),
	("content-textures3.zip", "PlatformContent/pc/textures/"),
	("extracontent-translations.zip", "ExtraContent/translations/"),
	("extracontent-luapackages.zip", "ExtraContent/LuaPackages/"),
	("extracontent-textures.zip", "ExtraContent/textures/"),
	("extracontent-scripts.zip", "ExtraContent/scripts/"),
	("extracontent-models.zip", "ExtraContent/models/")
];

pub fn get_latest_version_hash(binary: &str, channel: &str) -> String {
	fetch_latest_version(LatestVersion {
		channel: Cow::Borrowed(channel),
		binary: Cow::Borrowed(binary)
	}).hash.into_owned()
}

pub fn fetch_latest_version(version: LatestVersion) -> ExactVersion {
	let LatestVersion {channel, binary} = version;

	let version = match &*binary.to_lowercase() {
		"player" => format!("{}{}", LATEST_VERSION_PLAYER_CHANNEL, channel),
		"studio" => format!("{}{}", LATEST_VERSION_STUDIO_CHANNEL, channel),
		_ => {
			error!("Unknown binary type {:?}", binary);
			exit(1);
		}
	};

	let output = get(version)
		.expect("Failed to get the latest available version hash.")
		.text()
		.unwrap();

	let Response {client_version_upload: hash} = match from_str(&output) {
		Ok(json_parsed) => json_parsed,
		Err(error) => {
			let ResponseError {errors} = from_str(&output).expect(&format!("Failed to parse error response from server.\nResponse: {}\nError: {}", output, error));
			match errors[0].code {
				1 => { error!("Could not find version details for channel {}, make sure you have spelt the deployment channel name correctly.", channel); },
				5 => { error!("The deployment channel {} is restricted by Roblox!", channel); },
				_ => { error!("Unknown error response when attempting to resolve channel {}!\nResponse: {}\nError: {}", channel, output, error); }
			}

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
	fs::write(format!("{}/AppSettings.xml", path), "\
<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<Settings>
	<ContentFolder>content</ContentFolder>
	<BaseUrl>http://www.roblox.com</BaseUrl>
</Settings>").expect("Failed to write AppSettings.xml");
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
	
	let bindings: &[_] = if binary == "Player" { &PLAYER_EXTRACT_BINDINGS } else { &STUDIO_EXTRACT_BINDINGS };
	let deployment_channel = if channel == "LIVE" { LIVE_DEPLOYMENT_CDN.to_string() } else { format!("{CHANNEL_DEPLOYMENT_CDN}{channel}/") };

	status!("Using deployment CDN URL: {}", deployment_channel);
	status!("{} files will be downloaded", bindings.len());

	let client = reqwest::blocking::Client::new();
	progress_bar::init_progress_bar_with_eta(bindings.len());
	for (_index, (package, _path)) in bindings.iter().enumerate() {
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
	let bindings: &[_] = if binary == "Player" { &PLAYER_EXTRACT_BINDINGS } else { &STUDIO_EXTRACT_BINDINGS };
	help!("{} files will be extracted", bindings.len());

	progress_bar::init_progress_bar_with_eta(bindings.len());

	if disallow_multithreading {
		for (_index, (package, path)) in bindings.iter().enumerate() {
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

		for (_index, chunk) in chunked_files.enumerate() {
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

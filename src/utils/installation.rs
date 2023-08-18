use std::{process, fs, path, io};
use crate::utils::terminal::*;
use crate::setup;
use reqwest;
use serde_json;
use progress_bar;

const LATEST_VERSION_PLAYER: &str = "https://setup.rbxcdn.com/version";
const LATEST_VERSION_STUDIO: &str = "https://setup.rbxcdn.com/versionQTStudio";
const LATEST_VERSION_PLAYER_CHANNEL: &str = "https://clientsettings.roblox.com/v2/client-version/WindowsPlayer/channel/";
const LATEST_VERSION_STUDIO_CHANNEL: &str = "https://clientsettings.roblox.com/v2/client-version/WindowsStudio/channel/";
const LIVE_DEPLOYMENT_CDN: &str = "https://setup.rbxcdn.com/";
const CHANNEL_DEPLOYMENT_CDN: &str = "https://roblox-setup.cachefly.net/channel/";

const PLAYER_EXTRACT_BINDINGS: [(&'static str, &'static str); 20] = [
	("RobloxApp.zip", ""),
	("shaders.zip", "shaders/"),
	("ssl.zip", "ssl/"),
	("WebView2.zip", ""),
	("WebView2RuntimeInstaller.zip", "WebView2RuntimeInstaller/"),
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
const STUDIO_EXTRACT_BINDINGS: [(&'static str, &'static str); 32] = [
	("RobloxStudio.zip", ""),
	("redist.zip", ""),
	("Libraries.zip", ""),
	("LibrariesQt5.zip", ""),
	("WebView2.zip", ""),
	("WebView2RuntimeInstaller.zip", ""),
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

pub fn get_latest_version_hash(version_type: &str, channel: &str) -> String {
	let version_url: String;
	if version_type == "Player" {
		version_url = if channel == "LIVE" { LATEST_VERSION_PLAYER.to_string() } else { format!("{}{}", LATEST_VERSION_PLAYER_CHANNEL, channel) };
	} else if version_type == "Studio" {
		version_url = if channel == "LIVE" { LATEST_VERSION_STUDIO.to_string() } else { format!("{}{}", LATEST_VERSION_STUDIO_CHANNEL, channel) };
	} else {
		error(format!("Invalid version type: {}", version_type));
		return "".to_string();
	}

	let client = reqwest::blocking::Client::new();
	let mut output = client.get(version_url)
		.send()
		.expect("Failed to get the latest available version hash.")
		.text()
		.unwrap();

	if channel != "LIVE" {
		let json: serde_json::Value = serde_json::from_str(&output).unwrap();
		let version_hash = json["clientVersionUpload"].as_str().unwrap();
		output = version_hash.to_string();
	}

	success(format!("Received latest version hash: {}", output));

	return output;
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
	if binary == "" {
		error("Could not determine binary type for provided package menifest!");
	}

	return binary;
}

pub fn write_appsettings_xml(path: String) {
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
		warning(format!("{} is already downloaded. Skipping download.", version_hash));
		return temp_path;
	}
	setup::create_dir(&format!("cache/{}-download", version_hash));
	success("Constructed cache directory");
	status("Downloading deployment...");
	status(format!("Using cache directory: {temp_path}"));
	
	let bindings: &[_] = if binary == "Player" { &PLAYER_EXTRACT_BINDINGS } else { &STUDIO_EXTRACT_BINDINGS };
	let deployment_channel = if channel == "LIVE" { LIVE_DEPLOYMENT_CDN.to_string() } else { format!("{CHANNEL_DEPLOYMENT_CDN}{channel}/") };

	println!("{} {} {}", CHANNEL_DEPLOYMENT_CDN, channel, version_hash);
	status(format!("Using deployment CDN URL: {}", deployment_channel));
	status(format!("{} files will be downloaded from {}!", bindings.len(), deployment_channel));

	let client = reqwest::blocking::Client::new();
	progress_bar::init_progress_bar_with_eta(bindings.len());
	for (index, (package, _path)) in bindings.iter().enumerate() {
		progress_bar::print_progress_bar_info("•", format!("Downloading {package}... ({version_hash}-{package})").as_str(), progress_bar::Color::Blue, progress_bar::Style::Bold);

		let mut response = client.get(format!("{}{}-{}", deployment_channel, version_hash, package)).send().unwrap();
		if response.status().is_success() == false {
			warning(format!("Failed to download {} from CDN! Status code: {}", package, response.status()));
			continue;
		}
		let path: path::PathBuf = format!("{}/{}", temp_path, package).into();
		fs::create_dir_all(path.parent().unwrap()).unwrap();
		let mut file = fs::File::create(path).unwrap();
		io::copy(&mut response, &mut file).unwrap();

		progress_bar::inc_progress_bar();
	}

	progress_bar::finalize_progress_bar();
	success("All compressed files downloaded, expanding files...".to_string());
	return temp_path; // Return the cache path to continue with extraction
}

pub fn extract_deployment_zips(binary: &str, temp_path: String, extraction_path: String) {
	let bindings: &[_] = if binary == "Player" { &PLAYER_EXTRACT_BINDINGS } else { &STUDIO_EXTRACT_BINDINGS };
	status(format!("{} files will be extracted!", bindings.len()));

	progress_bar::init_progress_bar_with_eta(bindings.len());
	for (index, (package, path)) in bindings.iter().enumerate() {
		progress_bar::print_progress_bar_info("•", format!("Extracting {package}...").as_str(), progress_bar::Color::Blue, progress_bar::Style::Bold);

		if setup::confirm_existence(&format!("{}/{}", extraction_path, path)) && path.is_empty() == false {
			progress_bar::print_progress_bar_info("Warning", format!("{} is already extracted. Skipping extraction.", package).as_str(), progress_bar::Color::Red, progress_bar::Style::Bold);
			continue;
		}
		if path.to_string() != "" { // Create directory if it doesn't exist during extraction
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
	progress_bar::finalize_progress_bar();
}

pub fn get_package_manifest(version_hash: String, channel: String) -> String {
	let channel = if channel == "LIVE" { "".to_string() } else { format!("channel/{}/", channel) };
	let url = format!("https://setup.rbxcdn.com/{channel}{version_hash}-rbxPkgManifest.txt");
	let client = reqwest::blocking::Client::new();
	let output = client.get(url.clone())
		.send()
		.expect("Failed to get the latest available version hash.")
		.text()
		.unwrap();

	if output.contains("AccessDenied") {
		error(format!("Recieved AccessDenied response from server when getting rbxPkgManifest, the version hash is probably invalid.\nResponse: {}\nVersion hash: {}\nFull URL: {}", output, version_hash, url));
	} else if output.contains("Error") {
		error(format!("Unexpected server response when getting the rbxPkgManifest information.\nResponse: {}", output));
	}

	return output;
}

use std::{process, fs};
use crate::utils::terminal::*;

use super::setup;
static LATEST_VERSION_PLAYER: &str = "https://setup.rbxcdn.com/version";
static LATEST_VERSION_STUDIO: &str = "https://setup.rbxcdn.com/versionQTStudio";
static DEPLOYMENT_CDN: &str = "https://setup.rbxcdn.com/";

static PLAYER_EXTRACT_BINDINGS: [(&'static str, &'static str); 20] = [
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
static STUDIO_EXTRACT_BINDINGS: [(&'static str, &'static str); 32] = [
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

pub fn get_latest_version_hash(version_type: &str) -> String {
	let mut version_url: &str = "";
	if version_type == "Player" {
		version_url = LATEST_VERSION_PLAYER;
	} else if version_type == "Studio" {
		version_url = LATEST_VERSION_STUDIO;
	} else {
		error(format!("Invalid version type: {}", version_type));
	}

	let output = process::Command::new("curl")
		.arg(version_url)
		.output()
		.expect("Failed to execute curl command");

	if output.status.success() == false {
		error(format!("Failed to get the latest available version hash.\ncurl quitted with: {}", output.status));
	}

	let output_string = String::from_utf8_lossy(&output.stdout);
	success(format!("Received latest version hash: {}", output_string));

	return output_string.to_string();
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
		error("Could not determine binary type for provided package menifest!")
	}

	return binary;
}

pub fn write_appsettings_xml(path: String) {
	fs::write(format!("{}/AppSettings.xml", path), "\
<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<Settings>
	<ContentFolder>content</ContentFolder>
	<BaseUrl>http://www.roblox.com</BaseUrl>
</Settings>\
").expect("Failed to write AppSettings.xml");
}

pub fn download_deployment(binary: &str, version_hash: String) -> String {
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
	
	let client = reqwest::blocking::Client::new();
	let bindings: &[_] = if binary == "Player" { &PLAYER_EXTRACT_BINDINGS } else { &STUDIO_EXTRACT_BINDINGS };
	status(format!("{} files will be downloaded from {}!", bindings.len(), DEPLOYMENT_CDN));

	for (index, (package, _path)) in bindings.iter().enumerate() {
		let start_epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
		status(format!("Downloading {package}..."));

		let mut response = client.get(format!("{}{}-{}", DEPLOYMENT_CDN, version_hash, package)).send().unwrap();

		let path: std::path::PathBuf = format!("{}/{}", temp_path, package).into();
		std::fs::create_dir_all(path.parent().unwrap()).unwrap();
		let mut file = std::fs::File::create(path).unwrap();
		std::io::copy(&mut response, &mut file).unwrap();

		let end_epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
		let elapsed = end_epoch - start_epoch;
		let percentage = ((index as f32 + 1.0) / bindings.len() as f32 * 100.0) as u64;
		statusprogress(format!("Took {} seconds - {}% completed", elapsed, percentage));
	}

	success("All compressed files downloaded, expanding files...".to_string());
	return temp_path; // Return the cache path to continue with extraction
}

pub fn extract_deployment_zips(binary: &str, temp_path: String, extraction_path: String) {
	let bindings: &[_] = if binary == "Player" { &PLAYER_EXTRACT_BINDINGS } else { &STUDIO_EXTRACT_BINDINGS };
	status(format!("{} files will be extracted!", bindings.len()));

	for (index, (package, path)) in bindings.iter().enumerate() {
		let start_epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
		status(format!("Extracting {package}..."));

		if setup::confirm_existence(&format!("{}/{}", extraction_path, path)) && path.is_empty() == false {
			warning(format!("{} is already extracted. Skipping extraction.", package));
			continue;
		}
		
		if path.to_string() != "" {
			status(format!("Creating directory {}/{}", extraction_path, path));
			setup::create_dir(&format!("{}/{}", extraction_path, path));
		}
		let _output = process::Command::new("unzip")
			.arg(format!("{}/{}", temp_path, package))
			.arg("-d")
			.arg(format!("{}/{}", extraction_path, path))
			.output()
			.expect("Failed to execute unzip command");

		/*if output.status.success() == false { // TODO FIX DAMNIT
			warning("status.success() returned false, extraction may have failed");
		}*/
		let end_epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
		let elapsed = end_epoch - start_epoch;
		let percentage = ((index as f32 + 1.0) / bindings.len() as f32 * 100.0) as u64;
		statusprogress(format!("Took {} seconds - {}% completed", elapsed, percentage));
	}
}

pub fn get_package_manifest(version_hash: String) -> String {
	let output = process::Command::new("curl")
		.arg(format!("https://setup.rbxcdn.com/{}-rbxPkgManifest.txt", version_hash))
		.output()
		.expect("Failed to execute curl command");

	let output_string = String::from_utf8_lossy(&output.stdout);
	if output.status.success() == false {
		error(format!("Failed to get the rbxPkgManifest.txt.\ncurl quitted with: {}", output.status));
	} else if output_string.contains("AccessDenied") {
		error(format!("Recieved AccessDenied response from server when getting the rbxPkgManifest information, the version hash is probably invalid.\nResponse: {}", output_string));
	} else if output_string.contains("Error") {
		error(format!("Unexpected server response when getting the rbxPkgManifest information.\nResponse: {}", output_string));
	}

	return output_string.to_string();
}

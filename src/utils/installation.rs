use std::process;
use crate::utils::terminal::*;
static LATEST_VERSION: &str = "https://setup.rbxcdn.com/version";
static DEPLOYMENT_URL_CLIENT: &str = "https://setup.rbxcdn.com/version-{}-Roblox.exe";
static DEPLOYMENT_URL_STUDIO: &str = "https://setup.rbxcdn.com/RobloxStudioLauncherBeta.exe";

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

pub fn get_latest_version_hash() -> String {
	let output = process::Command::new("curl")
		.arg(LATEST_VERSION)
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
			binary = "WindowsPlayer";
		} else if package_str.contains("RobloxStudio.zip") {
			binary = "WindowsStudio";
		}
	}

	return binary;
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

use crate::utils::{terminal::*, notification::create_notification, setup};
use crate::args::launch;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, sync::mpsc, thread, time, io::{BufRead, BufReader, Seek}};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RichPresenceImage {
	asset_id: Option<i64>,
	hover_text: Option<String>,
	#[serde(default)]
	clear: bool,
	#[serde(default)]
	reset: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RichPresenceData {
	state: Option<String>,
	details: Option<String>,
	time_start: Option<i64>,
	time_end: Option<i64>,
	small_image: Option<RichPresenceImage>,
	large_image: Option<RichPresenceImage>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RichPresence {
	command: Option<String>,
	data: Option<RichPresenceData>,
}

#[derive(Serialize, Deserialize, Debug)]
enum BloxstrapCommand {
	SetRichPresence(RichPresence),
}

fn convert_into_assetdelivery_url(asset_id: i64) -> String {
	format!("https://assetdelivery.roblox.com/v1/asset/?id={}", asset_id)
}

macro_rules! construct_default_rpc {
	($activity:ident, $application_name:expr) => {
		let state = format!("Using Roblox {} on Linux!", $application_name);
		let $activity = activity::Activity::new()
			.state(&state)
			.details("With Applejuice")
			.assets(
				activity::Assets::new()
					.large_image("crudejuice")
					.large_text("Bitdancer Approved"),
			)
			.timestamps(
				activity::Timestamps::new()
				.start(
					time::SystemTime::now()
						.duration_since(time::SystemTime::UNIX_EPOCH)
						.unwrap()
						.as_millis() as i64,
				)
			);
	};
}

macro_rules! construct_rpc_assets {
	($rpc_assets:ident, $small_image:expr, $large_image:expr) => {
		// `clear` and `reset` have not been implemented yet, and may never be fully implemented due to how unstable RPC is
		// DO NOT TOUCH OR SCREW WITH THIS CODE IT WILL BREAK EVERYTHING AND CAUSE YOU HOURS UPON MONT HSOF DEBUGGING FOR THE LOVE OF GOD DO NOT CHANGE

		let large_image_url;
		let large_image_hover_text;
		let small_image_url;
		let small_image_hover_text;

		let mut $rpc_assets = activity::Assets::new();
		if let Some(RichPresenceImage {
			clear: _,
			asset_id,
			hover_text,
			reset: _,
		}) = $large_image {
			if let Some(asset_id) = asset_id {
				large_image_url = convert_into_assetdelivery_url(asset_id);
				$rpc_assets = $rpc_assets.large_image(&large_image_url);
			}
			if let Some(hover_text) = hover_text {
				large_image_hover_text = hover_text;
				$rpc_assets = $rpc_assets.large_text(&large_image_hover_text);
			}
		}
		if let Some(RichPresenceImage {
			clear: _,
			asset_id,
			hover_text,
			reset: _,
		}) = $small_image
		{
			if let Some(asset_id) = asset_id {
				small_image_url = convert_into_assetdelivery_url(asset_id);
				$rpc_assets = $rpc_assets.small_image(&small_image_url);
			}
			if let Some(hover_text) = hover_text {
				small_image_hover_text = hover_text;
				$rpc_assets = $rpc_assets.small_text(&small_image_hover_text);
			}
		}
	}
}

pub fn init_rpc(binary_type: String, already_known_log_file: Option<String>, output_log_to_stdout: Option<bool>) {
	let client = DiscordIpcClient::new("1160530617117712384").and_then(|mut client| {
		client.connect()?;

		let state = format!("Using Roblox {} on Linux!", binary_type.clone());
		let payload = activity::Activity::new()
			.state(&state)
			.details("With Applejuice")
			.assets(
				activity::Assets::new()
					.large_image("crudejuice")
					.large_text("Meower Approved"),
				
			)
			.timestamps(
				activity::Timestamps::new()
				.start(
					time::SystemTime::now()
						.duration_since(time::SystemTime::UNIX_EPOCH)
						.unwrap()
						.as_millis() as i64,
				)
			);

		client.set_activity(payload)?;
		success!("RPC instance started");

		Ok(client)
	}).inspect_err(|err| {
		warning!("Failed to start RPC instance: {}", err);
	});
	
	if let Ok(mut rpc_handler) = client { // If the RPC Client had successfully initialised
		thread::spawn(move || {
			let log_path= if already_known_log_file.is_some() {
				already_known_log_file
			} else {
				launch::resolve_active_logfile(format!("{}/prefixdata/pfx/drive_c/users/steamuser/AppData/Local/Roblox/logs/", setup::get_applejuice_dir()))
			};
			
			if let Some(log_path) = log_path {
				let mut file = fs::File::open(log_path.clone()).unwrap();
				let mut position = fs::metadata(log_path.clone()).unwrap().len();

				let (tx, rx) = mpsc::channel();
				let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
				let _ = watcher.watch(log_path.as_ref(), RecursiveMode::NonRecursive);

				let mut detected_bloxstrap = false;
				let mut last_successful_rec_unwrap: (u32, Value) = (0, Value::Null);
				//let mut old_presence = activity::Activity::new(); TODO
				for result in rx {
					match result {
						Ok(_event) => {
							if file.metadata().unwrap().len() == position {
								continue;
							}

							file.seek(std::io::SeekFrom::Start(position + 1)).unwrap();
							position = file.metadata().unwrap().len();

							let reader = BufReader::new(&file);
							for line in reader.lines() {
								let line_usable = line.unwrap_or_default(); // Sometimes Roblox likes to just throw vomit into the log file causing a panic
								let mut was_rpc_updated = false;

								if line_usable.contains("[BloxstrapRPC] ") {
									if !detected_bloxstrap {
										create_notification(&format!("{}/assets/crudejuice.png", setup::get_applejuice_dir()), 5000, "BloxstrapRPC enabled", "This game has support for the BloxstrapRPC protocol! We have switched to using it for your rich presence.");
										detected_bloxstrap = true;
									}
									status!("Parsing Log line for RPC: {}", line_usable);
									let line_split = line_usable.split("[BloxstrapRPC] ").collect::<Vec<&str>>()[1];

									let parsed_data: RichPresence = match serde_json::from_str(line_split) {
										Ok(parsed_data) => parsed_data,
										Err(error) => {
											warning!("Error occurred when attempting to parse RPC data: {error}");
											continue;
										}
									};
									
									let command = parsed_data.command;
									let data: RichPresenceData = parsed_data.data.unwrap();
									let state = data.state;
									let details = data.details;
									let time_start = data.time_start;
									let time_end = data.time_end;
									let small_image = data.small_image;
									let large_image = data.large_image;

									if command.is_none() {
										warning!("RPC command is none");
										continue;
									} else if command.unwrap() != "SetRichPresence" {
										warning!("RPC command is not SetRichPresence; ignoring");
										continue;
									}

									construct_rpc_assets!(rpc_assets, small_image, large_image);
									let state = &state.unwrap_or_default();
									let details = &details.unwrap_or_default();
									let mut activity = activity::Activity::new()
										.timestamps(
											activity::Timestamps::new().start(
												if time_start.unwrap_or_default() == 0 {
													time::SystemTime::now()
														.duration_since(time::SystemTime::UNIX_EPOCH)
														.unwrap()
														.as_millis() as i64
												} else {
													time_start.unwrap()
												},
											)
										)
										.assets(rpc_assets);

									if !state.is_empty() {
										activity = activity.state(state);
									}
									if !details.is_empty() {
										activity = activity.details(details);
									}
									if time_end.unwrap_or_default() != 0 && time_end.is_some() {
										activity = activity.timestamps(activity::Timestamps::new().end(time_end.unwrap()));
									}

									let _ = rpc_handler.set_activity(activity);
									was_rpc_updated = true;
								} else if line_usable.contains("leaveUGCGameInternal") { // When the user leaves a game and enters the LuaApp
									status!("Detected game leave; resetting RPC...");
									construct_default_rpc!(activity, binary_type);
									
									let _ = rpc_handler.set_activity(activity);
									was_rpc_updated = true;
								} else if output_log_to_stdout.unwrap_or_default() {
									status!("Client log: {}", line_usable);
								}

								if was_rpc_updated { // Debug related
									match rpc_handler.recv() {
										Ok(output) => {
											let output_string = format!("{:?}", output);
											last_successful_rec_unwrap = output;
											
											status!("RPC output: {}", output_string);
											if output_string.contains("ERROR") {
												warning!("Error occurred when attempting to send new RPC");
											}
										}
										Err(error) => {
											warning!("Error occurred when attempting to display RPC request receive: {error}\nLast successful receive unwrap: {:?}", last_successful_rec_unwrap);
											
											status!("Attempting to reset RPC...");
											construct_default_rpc!(activity, binary_type);

											let _ = rpc_handler.set_activity(activity);
										}
									}
								}
							}
						}
						Err(error) => {
							warning!("Log error: {error}");
						}
					}
				}
			} else {
				warning!("A file was created, but it is not a log file");
			}
		});
	}
}

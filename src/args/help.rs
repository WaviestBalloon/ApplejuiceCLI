pub fn main() {
	println!("ApplejuiceCLI - A manager to get Roblox to run on Linux using Valve's Proton
Usage: applejuicecli [command]
Optional parameters: [?removeolder] (The question mark shows it's optional)

Commands: 
\t[Normal commands]
\t--help\t\tDisplays this help message
\t--init\t\tInitialises Applejuice
\t--update\tUpdates Applejuice from GitHub Releases (not implemented yet)
\t--play\tLaunch Roblox Player with parameters like PlaceID, etc (not implemented yet)

\t[Installation-related commands]
\t--install\tInstalls Roblox Client or Roblox Studio
\t--init\t\tInitialises Applejuice
\t--purge\t\tPurges Roblox installations or cache
\t--opendata\tOpens the data folder for Applejuice, where installations, cache, configuration files and more are located
");
}

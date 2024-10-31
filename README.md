
# YouTube Client

This application is a simple YouTube video downloader built with Tauri and Rust. It allows users to download videos from YouTube and tracks their download status and watch progress in a JSON file.

## Features

- Download videos from YouTube using `yt-dlp`.
- Tracks downloaded videos and their watch progress in a JSON file. (WIP)
- Easy to use graphical interface with Dioxus.

## Prerequisites

Before you start, ensure you have met the following requirements:

- **Rust**: Make sure you have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/).
- **Node.js**: Install Node.js, which is required for running the frontend.
- **yt-dlp**: Ensure you have `yt-dlp` installed. You can find the installation instructions [here](https://github.com/yt-dlp/yt-dlp#installation).

## Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/MW3r3/yt-client.git
   cd yt-client
   ```

2. **Build the Tauri app**:

   ```bash
   cargo tauri build
   ```

3. **Run the application**:

   ```bash
   cargo tauri dev
   ```

## Usage

1. **Open the Application**: Run the application after building.

2. **Enter the Video ID**: In the input field, enter the YouTube video ID (the part after `v=` in the video URL).

3. **Download the Video**: Click the "Download" button to start downloading the video.

4. **Track Progress**: The application tracks the download progress and saves the information in a `video_progress.json` file located in your home directory.

5. **View Download Progress**: Check the application to see the download progress.

## JSON File

The application creates a `video_progress.json` file in your home directory. This file contains the following fields for each downloaded video:

- `id`: The YouTube video ID.
- `title`: The title of the video (to be extracted from the `yt-dlp` output).
- `downloaded`: A boolean indicating whether the video has been downloaded.
- `watch_progress`: The percentage of the video watched, which can be updated in the future.

## Contributing

Contributions are welcome! If you have suggestions or improvements, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Tauri](https://tauri.studio/) for the framework.
- [Dioxus](https://dioxuslabs.com/) for the frontend framework.
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) for the video downloading tool.

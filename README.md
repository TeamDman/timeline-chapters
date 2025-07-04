# Timeline Chapters

A command-line tool that converts timeline CSV files from video editing software into YouTube chapter format.

## 📋 Overview

This tool parses timeline CSV exports (such as those from DaVinci Resolve) and converts the marker timecodes into YouTube-compatible chapter timestamps. Perfect for content creators who want to automatically generate chapter markers for their YouTube videos from their video editing timeline.

## ✨ Features

- **CSV to YouTube Chapters**: Converts timeline CSV files to YouTube chapter format
- **Timecode Conversion**: Automatically converts `HH:MM:SS:FF` format to YouTube's `MM:SS` format
- **Smart Filtering**: Skips empty notes/markers automatically
- **Error Handling**: Robust error handling with helpful error messages
- **Cross-platform**: Works on Windows, macOS, and Linux

## 🚀 Installation

### Download Pre-built Binary (Recommended)

Download the latest release for your platform from the [GitHub Releases page](https://github.com/TeamDman/timeline-chapters/releases/tag/v0.1.0):

- **Windows**: Download `timeline_chapters.exe`

After downloading, you can run the tool directly or add it to your PATH for easier access.

### From Source
```powershell
git clone https://github.com/TeamDman/timeline-chapters
cd timeline-chapters
cargo build --release
```

The compiled binary will be available at `target/release/timeline_chapters.exe` (Windows) or `target/release/timeline_chapters` (Unix).

## 📖 Usage

```powershell
timeline_chapters <csv_path>
```

### Example
```powershell
timeline_chapters "Timeline 1.csv"
```

### Output
```
YouTube Chapters:

0:00 Intro
0:08 Water strainers into compacting drawers and composters
2:02 Crushing stuff to be sifted
3:44 Clay from water,sand,gravel in jar
4:16 Sifting the crushed materials
7:28 Furnace automation
9:42 Lava production from cobblestone in jars
10:18 Lava into power via magmators
11:00 Moving power from the cube into things that need power
```

## 📁 CSV Format

The tool expects a CSV file with the following columns:
- `Record In`: Timecode in `HH:MM:SS:FF` format
- `Notes`: Chapter title/description

Other columns are ignored. The tool automatically skips entries with empty notes.

### Supported Video Editing Software

- **DaVinci Resolve**: Export markers as CSV from the timeline
- **Other NLEs**: Any software that can export timeline markers in CSV format with the above columns

## 🎬 Exporting from DaVinci Resolve

For detailed instructions on how to export markers from DaVinci Resolve, check out this helpful video:

[Jason Yadlovski - 🚨 UPDATED 2024 🚨 How to Export MARKERS in DaVinci Resolve 18.6.5 (& Make YouTube Chapters)](https://www.youtube.com/watch?v=Qjh2WxA7W8E)

## 📝 License

This project is licensed under the Mozilla Public License 2.0 - see the [LICENSE](LICENSE) file for details.

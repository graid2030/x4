# X4: Foundations Trading System

A modular, web-based trading system for X4: Foundations written in Rust.

## Features

- Extract game data from X4 installation (CAT/DAT files)
- Parse save files to identify trade opportunities
- Filter by sectors, wares, and cargo capacity
- Calculate arbitrage opportunities
- Web-based UI with real-time filtering
- Automatic caching of game data

## Architecture

Built following SOLID principles and modular design:

```
src/
├── models/         # Data structures
├── parsers/        # CAT/DAT and XML parsers
├── extractors/     # Game data extraction logic
├── services/       # Business logic (cache, arbitrage)
└── api/            # Web server and API endpoints
```

## Quick Start (Windows)

1. **Build the project:**
   - Double-click `build.bat` (requires Visual Studio Build Tools)
   - Or run: `cargo build --release` in Developer Command Prompt

2. **Run the server:**
   - Double-click `run.bat`
   - Or run: `target\release\x4-trading-system.exe`

3. **Open your browser:**
   Navigate to `http://localhost:3000`

4. **Configure paths:**
   - **Game folder:** Path to your X4 installation (e.g., `C:\SteamLibrary\steamapps\common\X4 Foundations`)
   - **Saves directory:** Path to your saves folder (e.g., `C:\Users\YourName\Documents\Egosoft\X4\profile_name`)
   - **Select save:** Choose a save file from the dropdown list

5. **Filter and search:**
   - Select sectors and wares
   - Optionally set cargo volume
   - Click "Find Trade Opportunities"

## Requirements

- Rust 1.70+
- X4: Foundations installation
- Save file (from the game)

## How It Works

1. **Game Data Extraction:**
   - Parses CAT/DAT files from X4 installation
   - Extracts ware metadata (volume, transport type)
   - Extracts localization and sector names
   - Caches results for faster subsequent runs

2. **Save File Parsing:**
   - Loads save file (supports .gz compression)
   - Extracts sectors and stations
   - Parses trade offers (buy/sell prices, amounts)

3. **Arbitrage Calculation:**
   - Matches buy and sell offers for same ware
   - Calculates profit per unit and total profit
   - Applies filters (sectors, wares, cargo limits)
   - Groups by ware if requested

## License

This project replicates functionality from x4-cat-miner.py and x4-arbitrage.py

Original copyright (c) 2025 Mark Boddington

## Project Overview

This project is a **trading system for the game X4: Foundations**, designed to be modular, scalable, and easy to maintain.

---

## Architecture Standards

### File Size & Structure

* **Max file size: 200–250 lines**
* Break any file exceeding the limit into smaller modules
* Organize modules into directories by feature/domain
* Favor **composition** and modular design

### Design Philosophy

> Build simple, maintainable systems that solve current needs

#### Required Principles

* **KISS** — Keep It Simple
* **YAGNI** — Only build what is currently required
* **SOLID** principles:

    * Single Responsibility
    * Open/Closed
    * Liskov Substitution
    * Interface Segregation
    * Dependency Inversion

---

## Expected Output Style

* Clean, readable, documented code
* Clear separation of **data**, **logic**, and **configuration**
* Self-explanatory names
* Single-responsibility functions
* Minimalistic, focused architecture

---

## Avoid

* Over-engineering or futuristic abstractions
* Monolithic / god scripts
* Added features without request
* Deep inheritance — prefer composition

---

## Goal

Create a **simple, modular, scalable trading system** for X4 that follows modern software engineering practices.

---

## Implementation Status

### Completed Features

1. **Game Data Extraction**
   - CAT/DAT file parser (src/parsers/cat_dat.rs)
   - XML parsing utilities (src/parsers/game_xml.rs)
   - Ware metadata extractor (src/extractors/wares.rs)
   - Localization system (src/extractors/localization.rs)
   - Sector name extraction (src/extractors/sectors.rs)

2. **Save File Processing**
   - Save file loader with gzip support (src/parsers/save_xml.rs)
   - Sector extraction
   - Trade offer parsing

3. **Business Logic**
   - Game data caching system (src/services/cache.rs)
   - Arbitrage calculation (src/services/arbitrage.rs)
   - Filter support (sectors, wares, cargo, min profit)
   - Group by ware functionality

4. **Web Server**
   - Axum-based REST API (src/api/)
   - JSON endpoints for all operations
   - Static file serving for frontend

5. **Frontend UI**
   - Path configuration interface
   - Multi-select filters for sectors and wares
   - Cargo capacity calculator
   - Responsive results table
   - Real-time filtering

### Module Breakdown

All modules adhere to the 200-250 line limit:

---

# Sectors API Specification

## GET /api/sectors/list

### Purpose
Retrieve all sectors with aggregated statistics for display in sectors list.

### Request
```
GET /api/sectors/list
```

**Query Parameters:**
- None (future: filters like `?owner=argon`)

### Response

**Success (200):**
```json
{
  "sectors": [
    {
      "code": "AAM-257",
      "name": "The Reach",
      "macro_name": "cluster_18_sector001_macro",
      "owner": "argon",
      "owner_name": "Argon Federation",
      "contested": false,
      "station_count": 16,
      "player_stations": 1,
      "buy_offers": 45,
      "sell_offers": 38,
      "total_trades": 83,
      "resource_types": ["ice", "ore"],
      "discovered": true
    }
  ],
  "stats": {
    "total_sectors": 50,
    "player_owned": 3,
    "discovered": 45,
    "total_stations": 200
  }
}
```

**Error (500):**
```json
{
  "error": "Failed to load sectors",
  "details": "..."
}
```

### Data Model

```rust
#[derive(Serialize)]
pub struct SectorListItem {
    pub code: String,
    pub name: String,
    pub macro_name: String,
    pub owner: Option<String>,
    pub owner_name: Option<String>,
    pub contested: bool,
    pub station_count: usize,
    pub player_stations: usize,
    pub buy_offers: usize,
    pub sell_offers: usize,
    pub total_trades: usize,
    pub resource_types: Vec<String>,
    pub discovered: bool,
}

#[derive(Serialize)]
pub struct SectorsListResponse {
    pub sectors: Vec<SectorListItem>,
    pub stats: SectorStats,
}

#[derive(Serialize)]
pub struct SectorStats {
    pub total_sectors: usize,
    pub player_owned: usize,
    pub discovered: usize,
    pub total_stations: usize,
}
```

### Implementation Notes

**Data Sources:**
1. Sectors: From `state.save_data.sectors`
2. Station counts: Parse save XML, count by sector
3. Trade offers: From existing `extract_all_trades`
4. Resource types: Future enhancement (Phase 4)
5. Owner names: Map faction IDs to names

**Calculations:**
- `station_count`: Count stations where `sector_code == code`
- `player_stations`: Count stations where `owner == "player"`
- `buy_offers`: Count trade offers where `trade_type == Buy`
- `sell_offers`: Count trade offers where `trade_type == Sell`
- `discovered`: `knownto` contains "player"

### Backend Handler

```rust
// src/api/handlers.rs
pub async fn get_sectors_list(
    State(state): State<AppState>,
) -> Result<Json<SectorsListResponse>, StatusCode> {
    // 1. Get sectors from state
    // 2. Load save file
    // 3. Extract all trades
    // 4. Count stations per sector
    // 5. Aggregate stats
    // 6. Return response
}
```

---

## GET /api/sectors/:code/details (Future)

### Purpose
Get detailed information about a specific sector.

### Request
```
GET /api/sectors/AAM-257/details
```

### Response
```json
{
  "sector": {
    "code": "AAM-257",
    "name": "The Reach",
    "owner": "argon",
    "position": { "x": -8174.36, "y": 4329.99, "z": -38616.41 }
  },
  "stations": [...],
  "resources": {...},
  "top_routes": [...]
}
```

**Status**: Phase 2.2

---

## Performance Considerations

**Caching Strategy:**
- Cache station counts per sector (expensive to recalculate)
- Cache trade offer counts
- Invalidate on save reload

**Optimization:**
- Use HashMap for O(1) lookups
- Pre-aggregate during init if possible
- Lazy load sector details (not needed for list)

---

## Testing

**Test Cases:**
1. Empty save (no sectors) → Returns empty array
2. Multiple sectors → Returns sorted list
3. Sector with no stations → station_count = 0
4. Player-owned sector → owner = "player"
5. Undiscovered sector → discovered = false

**Manual Testing:**
```bash
curl http://localhost:3000/api/sectors/list | jq
```

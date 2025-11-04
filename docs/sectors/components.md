# Sectors Components Documentation

## React Components

### SectorsPage
**File:** `static/react-app/pages/sectors.jsx`
**Lines:** 224

#### Description
Main sectors list page with filtering, sorting, and statistics.

#### Props
None (uses router context)

#### State
```javascript
{
  loading: boolean,
  error: string | null,
  data: SectorsListResponse | null,
  filteredSectors: SectorListItem[],
  searchQuery: string,
  ownerFilter: string,
  sortBy: { key: string, direction: 'ASC' | 'DESC' }
}
```

#### Features
- **Stats Cards**: 4 summary cards showing totals
- **Search**: Real-time filtering by name/code/owner
- **Owner Filter**: Dropdown to filter by faction
- **Sortable Table**: Click any column header to sort
- **Row Navigation**: Click row → navigate to detail page
- **Loading/Error States**: Graceful UX

#### API Dependencies
- `window.API.getSectorsList()` - Fetch sectors data

#### Router Dependencies
- `useRouter()` - For navigation to detail pages

---

## Backend Components

### SectorListItem Model
**File:** `src/models/sector_list.rs`

```rust
pub struct SectorListItem {
    pub code: String,              // Sector code (e.g., "AAM-257")
    pub name: String,              // Display name (e.g., "The Reach")
    pub macro_name: String,        // Internal macro ID
    pub owner: Option<String>,     // Faction ID (e.g., "argon")
    pub owner_name: Option<String>,// Faction name (e.g., "Argon Federation")
    pub contested: bool,           // Contested status
    pub station_count: usize,      // Total stations in sector
    pub player_stations: usize,    // Player-owned stations
    pub buy_offers: usize,         // Number of buy trade offers
    pub sell_offers: usize,        // Number of sell trade offers
    pub total_trades: usize,       // Buy + sell offers
    pub resource_types: Vec<String>, // Resource types (future)
    pub discovered: bool,          // Discovery status
}
```

### SectorsListResponse Model
```rust
pub struct SectorsListResponse {
    pub sectors: Vec<SectorListItem>,
    pub stats: SectorStats,
}

pub struct SectorStats {
    pub total_sectors: usize,
    pub player_owned: usize,
    pub discovered: usize,
    pub total_stations: usize,
}
```

---

### get_sectors_list Handler
**File:** `src/api/handlers.rs`
**Lines:** 640-764

#### Purpose
Fetch all sectors with aggregated statistics.

#### Logic Flow
```
1. Load save data from state
2. Load save file XML
3. Extract all trades → count buy/sell offers
4. Count stations per sector → parse XML
5. Map faction IDs to display names
6. Build SectorListItem for each sector
7. Aggregate stats
8. Sort by name
9. Return JSON response
```

#### Key Functions
- `get_sectors_list()` - Main handler
- `count_stations_per_sector()` - Helper to parse stations from XML

#### Performance Notes
- Parses save XML on every request (slow for large saves)
- TODO: Cache results, invalidate on save reload

---

### count_stations_per_sector Helper
**File:** `src/api/handlers.rs`
**Lines:** 767-840

#### Purpose
Parse save XML to count stations per sector.

#### Algorithm
```rust
1. Initialize XML reader
2. Track current sector (state machine)
3. For each component:
   - If sector → set current_sector
   - If station → increment count for current_sector
   - If player-owned → increment player count
4. Return HashMap<sector_code, (total, player)>
```

#### Returns
```rust
HashMap<String, (usize, usize)>
// Key: sector code
// Value: (total stations, player stations)
```

---

## Data Flow Diagram

```
User clicks "Sectors" tab
    ↓
SectorsPage.loadSectors()
    ↓
API.getSectorsList()
    ↓
GET /api/sectors/list
    ↓
get_sectors_list() handler
    ├─ Load save XML
    ├─ extract_all_trades() → count offers
    ├─ count_stations_per_sector() → count stations
    ├─ Map factions
    └─ Aggregate & return JSON
    ↓
React state updates
    ↓
applyFilters() → filter/sort data
    ↓
Render table with react-virtualized
    ↓
User clicks row
    ↓
navigate(`/sectors/${code}`)
```

---

## Styling

### CSS Classes Used
- `.card` - Stat cards and filter panels
- `.search-input` - Search box and dropdowns
- `.btn-secondary` - Refresh button
- `.rv-row` - Table rows (react-virtualized)
- `.col-num` - Right-aligned number columns

### Inline Styles
- Stats grid: `gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))'`
- Filter grid: `gridTemplateColumns: '1fr 200px'`

---

## Future Enhancements

### Planned Features
1. **Discovery Filter** - Hide/show undiscovered sectors
2. **Contested Badge** - Visual indicator for contested sectors
3. **Resource Icons** - Show resource types with icons
4. **Export to CSV** - Download sector data
5. **Favorites** - Mark favorite sectors
6. **Comparison Mode** - Select multiple sectors to compare

### Performance Optimizations
1. **Caching** - Cache station/trade counts
2. **Incremental Updates** - Only reparse changed data
3. **Pagination** - For very large sector lists (100+)
4. **Virtual Scrolling** - Already implemented via react-virtualized

---

## Testing Checklist

### Unit Tests (Future)
- [ ] SectorListItem serialization/deserialization
- [ ] count_stations_per_sector() with various XML structures
- [ ] Faction mapping edge cases

### Integration Tests
- [ ] GET /api/sectors/list returns valid JSON
- [ ] Correct station counts
- [ ] Correct trade counts
- [ ] Sorting works
- [ ] Filtering works

### UI Tests
- [ ] Loading state appears
- [ ] Error state with retry button
- [ ] Stats cards display correct numbers
- [ ] Search filter works
- [ ] Owner filter works
- [ ] Column sorting toggles ASC/DESC
- [ ] Row click navigates

---

## Dependencies

### Backend
- `quick-xml` - XML parsing
- `serde` - Serialization
- `axum` - Web framework

### Frontend
- `React` - UI framework
- `react-virtualized` - Table virtualization
- Custom router - Navigation

---

**END OF COMPONENTS DOCUMENTATION**

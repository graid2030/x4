# Sectors Module Documentation

## Overview
The Sectors module provides functionality to view, analyze, and navigate game sectors.

## Components

### 1. Sectors List (`/sectors`)
- **Purpose**: Display all sectors with statistics
- **Status**: 🚧 In Progress
- **File**: `static/react-app/pages/sectors.jsx`

### 2. Sector Detail (`/sectors/:code`)
- **Purpose**: Detailed view of a single sector
- **Status**: 📋 Planned
- **File**: TBD

## API Endpoints

### GET /api/sectors/list
Returns all sectors with enhanced statistics.

**Response:**
```json
{
  "sectors": [
    {
      "code": "AAM-257",
      "name": "The Reach",
      "macro_name": "cluster_18_sector001_macro",
      "owner": "argon",
      "station_count": 16,
      "trade_offers_count": 83,
      "resource_types": ["ice", "ore"]
    }
  ]
}
```

### GET /api/sectors/:code/details
Returns detailed information about a specific sector.

**Status**: 📋 Planned

## Data Flow

```
User clicks "Sectors" tab
    ↓
SectorsPage loads
    ↓
Fetches /api/sectors/list
    ↓
Rust handler aggregates:
    - Sectors from save
    - Station counts
    - Trade offer counts
    ↓
React renders table
    ↓
User clicks row
    ↓
Navigate to /sectors/:code
```

## Files Structure

```
docs/sectors/
├── README.md              (this file)
├── api-specification.md   (API details)
├── components.md          (React components)
└── implementation-log.md  (development log)

src/
├── api/handlers.rs        (get_sectors_list handler)
└── services/              (business logic)

static/react-app/pages/
└── sectors.jsx            (SectorsPage component)
```

## Dependencies

**Backend:**
- Existing sector extraction (✅ Done)
- Trade offers extraction (✅ Done)
- Need: Aggregation logic

**Frontend:**
- react-virtualized (✅ Available)
- Existing table components
- Router (✅ Done)

## Implementation Progress

- [x] Phase 1: Infrastructure (routing, navigation)
- [ ] Phase 2.1: Sectors List
  - [ ] API design
  - [ ] Backend implementation
  - [ ] Frontend table
  - [ ] Filters & sorting
  - [ ] Navigation to detail
- [ ] Phase 2.2: Sector Detail
- [ ] Phase 2.3: Sector Comparison

## Related Documentation

- [V2.MD](../../V2.MD) - Overall V2 plan
- [BasicInfo.md](../../x4research/BasicInfo.md) - Game data structure
- [SectorInfo.md](../../x4research/SectorInfo.md) - Sector details

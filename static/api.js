// API calls (<= 200 lines per CLAUDE.md). Expose on window for JSX scripts.
window.API = {
  async initSystem(gamePath, savesDir, selectedSave) {
    const res = await fetch('/api/init', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ game_path: gamePath, saves_dir: savesDir, selected_save: selectedSave, lang_id: '44' })
    });
    if (!res.ok) throw new Error(`Init failed (${res.status})`);
    return res.json();
  },
  async listSaves(savesDir) {
    const res = await fetch('/api/list-saves', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ saves_dir: savesDir })
    });
    if (!res.ok) throw new Error('Failed to list saves');
    return res.json();
  },
  async loadPlayerProperty() {
    const res = await fetch('/api/player-property');
    if (!res.ok) throw new Error('Failed to load player property');
    return res.json();
  },
  async lastPaths() {
    const res = await fetch('/api/last-paths');
    if (!res.ok) return { game_path: null, saves_dir: null };
    return res.json();
  },
  async status() {
    const res = await fetch('/api/status');
    if (!res.ok) return { initialized: false };
    return res.json();
  },
  async loadSectors() {
    const res = await fetch('/api/sectors');
    if (!res.ok) throw new Error('Failed to load sectors');
    return res.json();
  },
  async loadWares() {
    const res = await fetch('/api/wares');
    if (!res.ok) throw new Error('Failed to load wares');
    return res.json();
  },
  async loadPilot() {
    const res = await fetch('/api/pilot');
    if (!res.ok) throw new Error('Failed to load pilot');
    return res.json();
  },
  async searchTradeOffers(filters) {
    const res = await fetch('/api/trade-offers', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(filters)
    });
    if (!res.ok) throw new Error(`Search failed (${res.status})`);
    return res.json();
  },
  async getWareTrades(wareId, sectors, cargoVolume, sameSectorOnly) {
    const res = await fetch('/api/ware-trades', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        ware_id: wareId,
        sectors: sectors,
        cargo_volume: cargoVolume,
        same_sector_only: sameSectorOnly
      })
    });
    if (!res.ok) throw new Error(`Failed to load ware trades (${res.status})`);
    return res.json();
  },
  async getSectorMap(sectorCode) {
    const res = await fetch('/api/sector-map', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ sector_code: sectorCode })
    });
    if (!res.ok) throw new Error(`Failed to load sector map (${res.status})`);
    return res.json();
  },
  async getSectorsList() {
    const res = await fetch('/api/sectors/list');
    if (!res.ok) throw new Error(`Failed to load sectors list (${res.status})`);
    return res.json();
  },
  async getSectorDetail(sectorCode) {
    const res = await fetch(`/api/sectors/${sectorCode}`);
    if (!res.ok) throw new Error(`Failed to load sector detail (${res.status})`);
    return res.json();
  }
};

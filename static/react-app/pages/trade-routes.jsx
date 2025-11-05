// Trade Routes Page - Enhanced with filter presets and CSV export (<= 200 lines)

const { Layout, PageContainer, Filters, Results } = window.Components;
const { useState, useMemo } = React;

function TradeRoutesPage() {
  const [offers, setOffers] = useState([]);
  const [lastFilters, setLastFilters] = useState(null);
  const [showPresets, setShowPresets] = useState(false);
  const [showOnlyFavorites, setShowOnlyFavorites] = useState(false);
  const [favorites, setFavorites] = useState(() => {
    const stored = localStorage.getItem('trade_route_favorites');
    return stored ? new Set(JSON.parse(stored)) : new Set();
  });

  async function handleSearch(filters) {
    setLastFilters(filters);
    const data = await window.API.searchTradeOffers(filters);
    setOffers(data);
    return data;
  }

  // Favorites: unique key per route
  function getFavoriteKey(offer) {
    return `${offer.ware}|${offer.buy_station_code}|${offer.sell_station_code}`;
  }

  function toggleFavorite(offer) {
    const key = getFavoriteKey(offer);
    const newFavorites = new Set(favorites);
    newFavorites.has(key) ? newFavorites.delete(key) : newFavorites.add(key);
    setFavorites(newFavorites);
    localStorage.setItem('trade_route_favorites', JSON.stringify(Array.from(newFavorites)));
  }

  // Filter by favorites
  const displayOffers = useMemo(() => {
    if (!showOnlyFavorites) return offers;
    return offers.filter(offer => favorites.has(getFavoriteKey(offer)));
  }, [offers, showOnlyFavorites, favorites]);

  // Filter presets - saved in localStorage
  function getPresets() {
    const stored = localStorage.getItem('trade_route_presets');
    return stored ? JSON.parse(stored) : [];
  }

  function savePreset() {
    if (!lastFilters) {
      alert('Please run a search first before saving a preset');
      return;
    }

    const presetName = prompt('Enter a name for this preset:');
    if (!presetName) return;

    const presets = getPresets();
    presets.push({
      id: Date.now(),
      name: presetName,
      filters: lastFilters,
    });

    localStorage.setItem('trade_route_presets', JSON.stringify(presets));
    alert('Preset saved!');
    setShowPresets(true);
  }

  function loadPreset(preset) {
    // The Filters component loads from localStorage, so we need to set the values there
    if (preset.filters.sectors) {
      localStorage.setItem('flt_sectors', JSON.stringify(preset.filters.sectors));
    }
    if (preset.filters.wares) {
      localStorage.setItem('flt_wares', JSON.stringify(preset.filters.wares));
    }
    if (preset.filters.cargo_volume) {
      localStorage.setItem('flt_cargo', String(preset.filters.cargo_volume));
    }
    localStorage.setItem('flt_group', preset.filters.group_by_ware ? '1' : '0');
    localStorage.setItem('flt_same_sector', preset.filters.same_sector_only ? '1' : '0');

    // Reload page to apply filters
    window.location.reload();
  }

  function deletePreset(presetId) {
    if (!confirm('Delete this preset?')) return;

    const presets = getPresets().filter(p => p.id !== presetId);
    localStorage.setItem('trade_route_presets', JSON.stringify(presets));
    setShowPresets(true); // Force re-render
  }

  // CSV Export
  function exportToCSV() {
    if (offers.length === 0) {
      alert('No results to export');
      return;
    }

    const headers = [
      'Ware',
      'Ware Name',
      'Buy Station',
      'Buy Sector',
      'Buy Price',
      'Sell Station',
      'Sell Sector',
      'Sell Price',
      'Amount',
      'Profit',
      'Profit Per Unit'
    ];

    const rows = offers.map(offer => [
      offer.ware,
      offer.ware_name || offer.ware,
      offer.buy_station,
      offer.buy_sector,
      offer.buy_price,
      offer.sell_station,
      offer.sell_sector,
      offer.sell_price,
      offer.amount,
      offer.profit,
      offer.profit_per_unit || 0
    ]);

    const csvContent = [
      headers.join(','),
      ...rows.map(row => row.map(cell => {
        // Escape cells with commas or quotes
        const str = String(cell);
        if (str.includes(',') || str.includes('"') || str.includes('\n')) {
          return `"${str.replace(/"/g, '""')}"`;
        }
        return str;
      }).join(','))
    ].join('\n');

    // Download file
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
    const link = document.createElement('a');
    const url = URL.createObjectURL(blob);
    link.setAttribute('href', url);
    link.setAttribute('download', `trade-routes-${new Date().toISOString().slice(0, 10)}.csv`);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }

  const presets = getPresets();

  return (
    <Layout>
      <PageContainer
        title="Trade Routes"
        subtitle={`${displayOffers.length} route${displayOffers.length !== 1 ? 's' : ''} found${showOnlyFavorites ? ' (favorites only)' : ''}`}
        actions={
          <div style={{ display: 'flex', gap: '8px' }}>
            <button
              className={showOnlyFavorites ? "btn-primary" : "btn-secondary"}
              onClick={() => setShowOnlyFavorites(!showOnlyFavorites)}
              disabled={offers.length === 0}
            >
              ⭐ Favorites ({favorites.size})
            </button>
            <button className="btn-secondary" onClick={exportToCSV} disabled={offers.length === 0}>
              📥 Export CSV
            </button>
            <button className="btn-secondary" onClick={savePreset} disabled={!lastFilters}>
              💾 Save Preset
            </button>
            <button className="btn-secondary" onClick={() => setShowPresets(!showPresets)}>
              📋 Presets ({presets.length})
            </button>
          </div>
        }
      >
        {/* Presets Panel */}
        {showPresets && presets.length > 0 && (
          <div className="card" style={{ marginBottom: '16px' }}>
            <h3 style={{ margin: '0 0 12px 0', fontSize: '14px', fontWeight: '600' }}>
              Saved Presets
            </h3>
            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
              {presets.map(preset => (
                <div
                  key={preset.id}
                  style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    padding: '8px',
                    background: 'var(--card-hover)',
                    borderRadius: '4px',
                  }}
                >
                  <div style={{ flex: 1 }}>
                    <div style={{ fontWeight: '600', fontSize: '13px' }}>{preset.name}</div>
                    <div style={{ fontSize: '11px', color: 'var(--muted)' }}>
                      {preset.filters.sectors?.length || 'All'} sectors,{' '}
                      {preset.filters.wares?.length || 'All'} wares
                      {preset.filters.cargo_volume && `, ${preset.filters.cargo_volume}m³`}
                    </div>
                  </div>
                  <div style={{ display: 'flex', gap: '8px' }}>
                    <button
                      className="btn-secondary"
                      onClick={() => loadPreset(preset)}
                      style={{ fontSize: '12px', padding: '4px 8px' }}
                    >
                      Load
                    </button>
                    <button
                      className="btn-secondary"
                      onClick={() => deletePreset(preset.id)}
                      style={{ fontSize: '12px', padding: '4px 8px', color: '#ef4444' }}
                    >
                      Delete
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        <Filters onSearch={handleSearch} />
        <Results
          data={displayOffers}
          filters={lastFilters}
          favorites={favorites}
          onToggleFavorite={toggleFavorite}
          getFavoriteKey={getFavoriteKey}
        />
      </PageContainer>
    </Layout>
  );
}

window.Pages = { ...window.Pages, TradeRoutesPage };

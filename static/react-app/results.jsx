// Results (React Virtualized) (<= 200 lines)

const { useMemo, useState, useEffect, useRef } = React;
const { Table, Column, SortDirection } = window.ReactVirtualized;

function number(v) { const n = Number(v); return isFinite(n) ? n : 0; }

function Results({ data, filters, fontScale = 1, favorites, onToggleFavorite, getFavoriteKey }) {
  const [sortBy, setSortBy] = useState('total_profit');
  const [sortDirection, setSortDirection] = useState(SortDirection.DESC);
  const [selectedIndex, setSelectedIndex] = useState(-1);
  const [expandedWare, setExpandedWare] = useState(null);
  const [wareDetails, setWareDetails] = useState([]);
  const [loadingDetails, setLoadingDetails] = useState(false);
  const [mapModalSector, setMapModalSector] = useState(null);

  const numericKeys = new Set(['buy_price','sell_price','unit_profit','percent_profit','total_profit','qty','fits','limited_total_profit','total_volume','limited_total_volume','volume']);

  const sorted = useMemo(() => {
    if (!Array.isArray(data)) return [];
    const m = sortDirection === SortDirection.ASC ? 1 : -1;
    const arr = data.slice();
    arr.sort((a, b) => {
      if (numericKeys.has(sortBy)) {
        const av = number(a?.[sortBy]);
        const bv = number(b?.[sortBy]);
        return m * (av - bv);
      }
      const as = String(a?.[sortBy] ?? '').toLowerCase();
      const bs = String(b?.[sortBy] ?? '').toLowerCase();
      return m * (as < bs ? -1 : as > bs ? 1 : 0);
    });
    return arr;
  }, [data, sortBy, sortDirection]);

  function onSort({ sortBy: sb, sortDirection: sd }) {
    setSortBy(sb); setSortDirection(sd);
  }

  async function handleRowClick({ index }) {
    setSelectedIndex(prev => prev === index ? -1 : index);
    const row = sorted[index];
    if (!row || !filters) return;

    // Toggle expansion
    if (expandedWare === row.ware) {
      setExpandedWare(null);
      setWareDetails([]);
      return;
    }

    // Load details for this ware
    setExpandedWare(row.ware);
    setLoadingDetails(true);
    try {
      const details = await window.API.getWareTrades(
        row.ware,
        filters.sectors,
        filters.cargo_volume,
        filters.same_sector_only || false
      );
      setWareDetails(details);
    } catch (e) {
      console.error('Failed to load ware details:', e);
      setWareDetails([]);
    } finally {
      setLoadingDetails(false);
    }
  }

  const rowGetter = ({ index }) => sorted[index];
  const cell = key => ({ cellData }) => key === 'percent_profit' ? `${number(cellData).toFixed(2)}%` :
    (typeof cellData === 'number' ? number(cellData).toLocaleString(undefined, { maximumFractionDigits: 2 }) : (cellData ?? '-'));

  // Custom renderer for station columns: "Sector [OWN], Station (Code)" with clickable sector
  const stationCell = (type) => ({ rowData }) => {
    const sector = rowData[`${type}_sector`] || '-';
    const station = rowData[`${type}_station`] || '-';
    const code = rowData[`${type}_station_code`] || '';
    const owner = rowData[`${type}_sector_owner`] || '';
    const sectorCode = rowData[`${type}_sector`]?.match(/\(([^)]+)\)/)?.[1] || sector;
    const ownerTag = owner ? ` [${owner.substring(0, 3).toUpperCase()}]` : '';

    return (
      <span>
        <span
          className="sector-link"
          onClick={(e) => { e.stopPropagation(); setMapModalSector({ code: sectorCode, name: sector }); }}
        >
          {sector}{ownerTag}
        </span>
        , {station}{code && code !== station ? ` (${code})` : ''}
      </span>
    );
  };

  const header = dataKey => ({ label, sortBy: sb, sortDirection: sd }) => (
    <span>{label}{sb === dataKey ? (sd === SortDirection.ASC ? ' ▲' : ' ▼') : ''}</span>
  );

  // Manual sizing to avoid AutoSizer edge-cases
  const wrapRef = useRef(null);
  const [dims, setDims] = useState({ width: 800, height: Math.max(320, Math.round(window.innerHeight * 0.6)) });
  useEffect(() => {
    const measure = () => {
      const w = wrapRef.current ? wrapRef.current.clientWidth : window.innerWidth - 40;
      const h = Math.max(320, Math.round(window.innerHeight * 0.6));
      setDims({ width: w, height: h });
    };
    measure();
    window.addEventListener('resize', measure);
    return () => window.removeEventListener('resize', measure);
  }, []);

  const baseRow = 30, baseHeader = 34;
  const rowH = Math.round(baseRow * fontScale);
  const headH = Math.round(baseHeader * fontScale);

  const totalWidth = 2020; // sum of column widths below (buy_station:400 + sell_station:400)

  return (
    <section className="card">
      <h2>Trade Opportunities</h2>
      <div id="results-count" style={{marginBottom:8}}>{sorted.length.toLocaleString()} results</div>
      <div ref={wrapRef} className="table-container" style={{height: dims.height, overflowX:'auto'}}>
        <div style={{minWidth: totalWidth}}>
        <Table
          width={Math.max(dims.width, totalWidth)}
          height={dims.height}
          headerHeight={headH}
          rowHeight={rowH}
          rowCount={sorted.length}
          rowGetter={rowGetter}
          sort={onSort}
          sortBy={sortBy}
          sortDirection={sortDirection}
          rowClassName={({ index }) => {
            if (index < 0) return 'rv-header';
            const base = index % 2 ? 'rv-row odd' : 'rv-row';
            return index === selectedIndex ? base + ' selected' : base;
          }}
          onRowClick={handleRowClick}
        >
          <Column label="Ware" dataKey="ware_name" width={220} headerRenderer={header('ware_name')} cellRenderer={({ rowData }) => {
            const isFavorite = favorites && getFavoriteKey && favorites.has(getFavoriteKey(rowData));
            return (
              <span style={{display: 'flex', alignItems: 'center', gap: '4px'}}>
                {onToggleFavorite && (
                  <span
                    style={{cursor: 'pointer', fontSize: '16px', userSelect: 'none'}}
                    onClick={(e) => { e.stopPropagation(); onToggleFavorite(rowData); }}
                    title={isFavorite ? 'Remove from favorites' : 'Add to favorites'}
                  >
                    {isFavorite ? '⭐' : '☆'}
                  </span>
                )}
                <span>{rowData.ware_name || rowData.ware || '-'}</span>
              </span>
            );
          }} />
          <Column label="Buy From" dataKey="buy_station" width={400} headerRenderer={header('buy_station')} cellRenderer={stationCell('buy')} />
          <Column label="Buy Price" dataKey="buy_price" width={120} className="col-num" headerRenderer={header('buy_price')} cellRenderer={cell('buy_price')} />
          <Column label="Sell To" dataKey="sell_station" width={400} headerRenderer={header('sell_station')} cellRenderer={stationCell('sell')} />
          <Column label="Sell Price" dataKey="sell_price" width={120} className="col-num" headerRenderer={header('sell_price')} cellRenderer={cell('sell_price')} />
          <Column label="Profit/Unit" dataKey="unit_profit" width={140} className="col-num" headerRenderer={header('unit_profit')} cellRenderer={cell('unit_profit')} />
          <Column label="Profit %" dataKey="percent_profit" width={110} className="col-num" headerRenderer={header('percent_profit')} cellRenderer={cell('percent_profit')} />
          <Column label="Qty" dataKey="qty" width={90} className="col-num" headerRenderer={header('qty')} cellRenderer={cell('qty')} />
          <Column label="Total Profit" dataKey="total_profit" width={160} className="col-num" headerRenderer={header('total_profit')} cellRenderer={cell('total_profit')} />
          <Column label="Fits" dataKey="fits" width={90} className="col-num" headerRenderer={header('fits')} cellRenderer={cell('fits')} />
          <Column label="Limited Profit" dataKey="limited_total_profit" width={170} className="col-num" headerRenderer={header('limited_total_profit')} cellRenderer={cell('limited_total_profit')} />
        </Table>
        </div>
      </div>

      {expandedWare && (
        <div style={{marginTop: '1rem', borderTop: '2px solid var(--border)', paddingTop: '1rem'}}>
          <h3 style={{marginBottom: '0.5rem'}}>
            All trades for: {sorted.find(r => r.ware === expandedWare)?.ware_name || expandedWare}
            <button
              onClick={() => { setExpandedWare(null); setWareDetails([]); }}
              style={{marginLeft: '1rem', padding: '4px 12px', fontSize: '0.9em'}}
              className="btn-secondary"
            >
              Close ✕
            </button>
          </h3>
          {loadingDetails ? (
            <div style={{padding: '2rem', textAlign: 'center'}}>Loading details...</div>
          ) : wareDetails.length === 0 ? (
            <div style={{padding: '2rem', textAlign: 'center', color: 'var(--subtle)'}}>No trades found</div>
          ) : (
            <WareDetailsTable data={wareDetails} fontScale={fontScale} onSectorClick={setMapModalSector} />
          )}
        </div>
      )}

      {mapModalSector && (
        <window.Components.SectorMapModal
          sectorCode={mapModalSector.code}
          sectorName={mapModalSector.name}
          onClose={() => setMapModalSector(null)}
        />
      )}
    </section>
  );
}

// Nested table component for ware details
function WareDetailsTable({ data, fontScale = 1, onSectorClick }) {
  const [sortBy, setSortBy] = useState('percent_profit');
  const [sortDirection, setSortDirection] = useState(SortDirection.DESC);

  const numericKeys = new Set(['buy_price','sell_price','unit_profit','percent_profit','total_profit','qty','fits','limited_total_profit']);

  const sorted = useMemo(() => {
    const m = sortDirection === SortDirection.ASC ? 1 : -1;
    const arr = data.slice();
    arr.sort((a, b) => {
      if (numericKeys.has(sortBy)) {
        const av = number(a?.[sortBy]);
        const bv = number(b?.[sortBy]);
        return m * (av - bv);
      }
      const as = String(a?.[sortBy] ?? '').toLowerCase();
      const bs = String(b?.[sortBy] ?? '').toLowerCase();
      return m * (as < bs ? -1 : as > bs ? 1 : 0);
    });
    return arr;
  }, [data, sortBy, sortDirection]);

  function onSort({ sortBy: sb, sortDirection: sd }) {
    setSortBy(sb); setSortDirection(sd);
  }

  const rowGetter = ({ index }) => sorted[index];
  const cell = key => ({ cellData }) => key === 'percent_profit' ? `${number(cellData).toFixed(2)}%` :
    (typeof cellData === 'number' ? number(cellData).toLocaleString(undefined, { maximumFractionDigits: 2 }) : (cellData ?? '-'));

  const stationCell = (type) => ({ rowData }) => {
    const sector = rowData[`${type}_sector`] || '-';
    const station = rowData[`${type}_station`] || '-';
    const code = rowData[`${type}_station_code`] || '';
    const owner = rowData[`${type}_sector_owner`] || '';
    const sectorCode = rowData[`${type}_sector`]?.match(/\(([^)]+)\)/)?.[1] || sector;
    const ownerTag = owner ? ` [${owner.substring(0, 3).toUpperCase()}]` : '';

    return (
      <span>
        <span
          className="sector-link"
          onClick={(e) => { e.stopPropagation(); if (onSectorClick) onSectorClick({ code: sectorCode, name: sector }); }}
        >
          {sector}{ownerTag}
        </span>
        , {station}{code && code !== station ? ` (${code})` : ''}
      </span>
    );
  };

  const header = dataKey => ({ label, sortBy: sb, sortDirection: sd }) => (
    <span>{label}{sb === dataKey ? (sd === SortDirection.ASC ? ' ▲' : ' ▼') : ''}</span>
  );

  const baseRow = 30, baseHeader = 34;
  const rowH = Math.round(baseRow * fontScale);
  const headH = Math.round(baseHeader * fontScale);
  const totalWidth = 2020;
  const tableHeight = Math.min(400, sorted.length * rowH + headH + 20);

  return (
    <div className="table-container" style={{height: tableHeight, overflowX:'auto', marginTop: '0.5rem'}}>
      <div style={{minWidth: totalWidth}}>
        <Table
          width={totalWidth}
          height={tableHeight}
          headerHeight={headH}
          rowHeight={rowH}
          rowCount={sorted.length}
          rowGetter={rowGetter}
          sort={onSort}
          sortBy={sortBy}
          sortDirection={sortDirection}
          rowClassName={({ index }) => index < 0 ? 'rv-header' : (index % 2 ? 'rv-row odd' : 'rv-row')}
        >
          <Column label="Buy From" dataKey="buy_station" width={400} headerRenderer={header('buy_station')} cellRenderer={stationCell('buy')} />
          <Column label="Buy Price" dataKey="buy_price" width={120} className="col-num" headerRenderer={header('buy_price')} cellRenderer={cell('buy_price')} />
          <Column label="Sell To" dataKey="sell_station" width={400} headerRenderer={header('sell_station')} cellRenderer={stationCell('sell')} />
          <Column label="Sell Price" dataKey="sell_price" width={120} className="col-num" headerRenderer={header('sell_price')} cellRenderer={cell('sell_price')} />
          <Column label="Profit/Unit" dataKey="unit_profit" width={140} className="col-num" headerRenderer={header('unit_profit')} cellRenderer={cell('unit_profit')} />
          <Column label="Profit %" dataKey="percent_profit" width={110} className="col-num" headerRenderer={header('percent_profit')} cellRenderer={cell('percent_profit')} />
          <Column label="Qty" dataKey="qty" width={90} className="col-num" headerRenderer={header('qty')} cellRenderer={cell('qty')} />
          <Column label="Total Profit" dataKey="total_profit" width={160} className="col-num" headerRenderer={header('total_profit')} cellRenderer={cell('total_profit')} />
          <Column label="Fits" dataKey="fits" width={90} className="col-num" headerRenderer={header('fits')} cellRenderer={cell('fits')} />
          <Column label="Limited Profit" dataKey="limited_total_profit" width={170} className="col-num" headerRenderer={header('limited_total_profit')} cellRenderer={cell('limited_total_profit')} />
        </Table>
      </div>
    </div>
  );
}

window.Components = window.Components || {};
window.Components.Results = Results;

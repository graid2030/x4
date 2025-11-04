// Filters component (<= 200 lines)

const { useEffect, useMemo, useState } = React;

function Filters({ onSearch, preload }) {
  const [sectors, setSectors] = useState([]);
  const [wares, setWares] = useState([]);
  const [sectorQuery, setSectorQuery] = useState(localStorage.getItem('flt_sector_query') || '');
  const [wareQuery, setWareQuery] = useState(localStorage.getItem('flt_ware_query') || '');
  const [cargoVolume, setCargoVolume] = useState(localStorage.getItem('flt_cargo') || '');
  const [groupByWare, setGroupByWare] = useState(localStorage.getItem('flt_group') === '1');
  const [sameSectorOnly, setSameSectorOnly] = useState(localStorage.getItem('flt_same_sector') === '1');
  const [selSectors, setSelSectors] = useState(() => new Set(JSON.parse(localStorage.getItem('flt_sectors') || '[]')));
  const [selWares, setSelWares] = useState(() => new Set(JSON.parse(localStorage.getItem('flt_wares') || '[]')));
  const [busy, setBusy] = useState(false);
  const [mapModalSector, setMapModalSector] = useState(null);

  useEffect(() => {
    (async () => {
      const s = await window.API.loadSectors();
      const w = await window.API.loadWares();
      // if nothing saved, default to all
      if (selSectors.size === 0) setSelSectors(new Set(s.map(x => x.code)));
      if (selWares.size === 0) setSelWares(new Set(w.map(x => x.id)));
      setSectors(s); setWares(w);
      if (preload) preload({ sectors: s, wares: w });
    })();
  }, []);

  // persist filters
  useEffect(() => { localStorage.setItem('flt_sector_query', sectorQuery); }, [sectorQuery]);
  useEffect(() => { localStorage.setItem('flt_ware_query', wareQuery); }, [wareQuery]);
  useEffect(() => { localStorage.setItem('flt_cargo', cargoVolume); }, [cargoVolume]);
  useEffect(() => { localStorage.setItem('flt_group', groupByWare ? '1' : '0'); }, [groupByWare]);
  useEffect(() => { localStorage.setItem('flt_same_sector', sameSectorOnly ? '1' : '0'); }, [sameSectorOnly]);
  useEffect(() => { localStorage.setItem('flt_sectors', JSON.stringify(Array.from(selSectors))); }, [selSectors]);
  useEffect(() => { localStorage.setItem('flt_wares', JSON.stringify(Array.from(selWares))); }, [selWares]);

  function bulkSelect(setter, list, on) { setter(new Set(on ? list : [])); }

  const filteredSectors = useMemo(() =>
    sectors.filter(s => (s.name + ' ' + s.code).toLowerCase().includes(sectorQuery.toLowerCase())),
    [sectors, sectorQuery]
  );
  const filteredWares = useMemo(() =>
    wares.filter(w => (w.name).toLowerCase().includes(wareQuery.toLowerCase())),
    [wares, wareQuery]
  );

  const sectorsCount = selSectors.size;
  const waresCount = selWares.size;

  async function handleSearch() {
    setBusy(true);
    const filters = {
      sectors: sectorsCount ? Array.from(selSectors) : null,
      wares: waresCount ? Array.from(selWares) : null,
      cargo_volume: cargoVolume ? Number(cargoVolume) : null,
      group_by_ware: !!groupByWare,
      same_sector_only: !!sameSectorOnly,
    };
    try { await onSearch(filters); } finally { setBusy(false); }
  }

  return (
    <section className="card">
      <h2>Filters</h2>

      <div className="toolbar">
        <div className="badge">Sectors: {sectorsCount}/{sectors.length}</div>
        <div className="badge">Wares: {waresCount}/{wares.length}</div>
      </div>

      <div className="form-group">
        <label>Sectors</label>
        <div className="input-group">
          <input className="search-input" placeholder="Search sectors..." value={sectorQuery} onChange={e=>setSectorQuery(e.target.value)} />
          <div className="filter-buttons">
            <button className="btn-secondary" onClick={()=>bulkSelect(setSelSectors, sectors.map(x=>x.code), true)}>Select All</button>
            <button className="btn-secondary" onClick={()=>bulkSelect(setSelSectors, sectors.map(x=>x.code), false)}>Clear</button>
          </div>
        </div>
        <div id="sectors-list" className="checkbox-list">
          {filteredSectors.map(s => (
            <label key={s.code} title={s.name}>
              <input type="checkbox" className="sector-checkbox" checked={selSectors.has(s.code)} onChange={e=>{
                const next = new Set(selSectors); e.target.checked ? next.add(s.code) : next.delete(s.code); setSelSectors(next);
              }} />
              <span
                className="sector-link"
                onClick={(e) => { e.preventDefault(); e.stopPropagation(); setMapModalSector({ code: s.code, name: s.name }); }}
              >
                {s.name}
              </span>
              {' '}({s.code}){s.owner ? ` [${s.owner.substring(0, 3).toUpperCase()}]` : ''}
            </label>
          ))}
        </div>
      </div>

      <div className="form-group">
        <label>Wares</label>
        <div className="input-group">
          <input className="search-input" placeholder="Search wares..." value={wareQuery} onChange={e=>setWareQuery(e.target.value)} />
          <div className="filter-buttons">
            <button className="btn-secondary" onClick={()=>bulkSelect(setSelWares, wares.map(x=>x.id), true)}>Select All</button>
            <button className="btn-secondary" onClick={()=>bulkSelect(setSelWares, wares.map(x=>x.id), false)}>Clear</button>
          </div>
        </div>
        <div id="wares-list" className="checkbox-list">
          {filteredWares.map(w => (
            <label key={w.id} title={w.name}>
              <input type="checkbox" className="ware-checkbox" checked={selWares.has(w.id)} onChange={e=>{
                const next = new Set(selWares); e.target.checked ? next.add(w.id) : next.delete(w.id); setSelWares(next);
              }} />
              {w.name}
            </label>
          ))}
        </div>
      </div>

      <div className="form-group">
        <label>Cargo Volume (optional)</label>
        <input type="number" value={cargoVolume} onChange={e=>setCargoVolume(e.target.value)} placeholder="e.g., 15000" min="0" step="100" />
      </div>

      <div className="form-group">
        <label>
          <input type="checkbox" checked={groupByWare} onChange={e=>setGroupByWare(e.target.checked)} />
          {' '}Group by ware (show most profitable only)
        </label>
      </div>

      <div className="form-group">
        <label>
          <input type="checkbox" checked={sameSectorOnly} onChange={e=>setSameSectorOnly(e.target.checked)} />
          {' '}Same sector only (buy and sell in same sector)
        </label>
      </div>

      <button className="btn-primary" onClick={handleSearch} disabled={busy}>{busy ? 'Searching…' : 'Find Trade Opportunities'}</button>

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

window.Components = window.Components || {};
window.Components.Filters = Filters;

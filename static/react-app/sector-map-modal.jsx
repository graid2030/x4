// Sector Map Modal (<= 200 lines)

const { useState, useEffect, useMemo } = React;

// Faction colors matching Python script
const FACTION_COLORS = {
  argon: '#4a9eff',
  paranid: '#ff8c42',
  teladi: '#4caf50',
  split: '#e74c3c',
  xenon: '#95a5a6',
  khaak: '#9b59b6',
  player: '#ffd700',
  antigone: '#00bcd4',
  holyorder: '#ffeb3b',
  ministry: '#ff5722',
  ownerless: '#bdc3c7',
  unknown: '#7f8c8d',
};

function SectorMapModal({ sectorCode, sectorName, onClose }) {
  const [mapData, setMapData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [zoom, setZoom] = useState(1);

  useEffect(() => {
    async function loadMap() {
      if (!sectorCode) return;
      setLoading(true);
      setError(null);
      try {
        const data = await window.API.getSectorMap(sectorCode);
        setMapData(data);
      } catch (e) {
        console.error('Failed to load sector map:', e);
        setError(e.message || 'Failed to load sector map');
      } finally {
        setLoading(false);
      }
    }
    loadMap();
  }, [sectorCode]);

  // Build SVG map
  const svgContent = useMemo(() => {
    if (!mapData) return null;

    const W = 1200, H = 800, margin = 48;

    // Collect all positions
    const allX = [], allZ = [];
    mapData.stations.forEach(s => { allX.push(s.position.x); allZ.push(s.position.z); });
    mapData.ships.forEach(s => { allX.push(s.position.x); allZ.push(s.position.z); });
    mapData.gates.forEach(g => { allX.push(g.position.x); allZ.push(g.position.z); });
    mapData.asteroids.forEach(a => { allX.push(a.position.x); allZ.push(a.position.z); });
    mapData.resources.forEach(r => { allX.push(r.position.x); allZ.push(r.position.z); });

    if (allX.length === 0) return <text x="50%" y="50%" textAnchor="middle" fill="var(--muted)">No objects found in sector</text>;

    const minX = Math.min(...allX), maxX = Math.max(...allX);
    const minZ = Math.min(...allZ), maxZ = Math.max(...allZ);
    const rangeX = maxX - minX || 1, rangeZ = maxZ - minZ || 1;
    const scale = Math.min((W - 2 * margin) / rangeX, (H - 2 * margin) / rangeZ) * zoom;

    const mapPoint = (x, z) => ({
      x: W / 2 + (x - (minX + maxX) / 2) * scale,
      y: H / 2 - (z - (minZ + maxZ) / 2) * scale
    });

    return (
      <svg width={W} height={H} viewBox={`0 0 ${W} ${H}`} style={{background: '#0a0f1a', borderRadius: '8px'}}>
        {/* Grid */}
        {[...Array(11)].map((_, i) => {
          const gx = minX + i * rangeX / 10;
          const gz = minZ + i * rangeZ / 10;
          const p1 = mapPoint(gx, minZ), p2 = mapPoint(gx, maxZ);
          const p3 = mapPoint(minX, gz), p4 = mapPoint(maxX, gz);
          return (
            <g key={i}>
              <line x1={p1.x} y1={p1.y} x2={p2.x} y2={p2.y} stroke="#1c2a38" strokeWidth="1" />
              <line x1={p3.x} y1={p3.y} x2={p4.x} y2={p4.y} stroke="#1c2a38" strokeWidth="1" />
            </g>
          );
        })}

        {/* Title */}
        <text x={W/2} y={24} textAnchor="middle" fill="#e8eef5" fontSize="18" fontWeight="600">
          {mapData.sector_name} ({mapData.sector_code})
        </text>

        {/* Asteroids (grey dots - smaller) */}
        {mapData.asteroids.map((a, i) => {
          const p = mapPoint(a.position.x, a.position.z);
          return <circle key={`ast-${i}`} cx={p.x} cy={p.y} r="1" fill="#555" stroke="none">
            <title>Asteroid {a.code}</title>
          </circle>;
        })}

        {/* Resources (cyan circles) */}
        {mapData.resources.map((r, i) => {
          const p = mapPoint(r.position.x, r.position.z);
          return (
            <g key={`res-${i}`}>
              <circle cx={p.x} cy={p.y} r="3" fill="#4dabf7" stroke="#fff" strokeWidth="1">
                <title>{r.name} ({r.code})</title>
              </circle>
            </g>
          );
        })}

        {/* Gates (yellow diamonds) */}
        {mapData.gates.map((g, i) => {
          const p = mapPoint(g.position.x, g.position.z);
          const size = 6;
          const points = `${p.x},${p.y-size} ${p.x+size},${p.y} ${p.x},${p.y+size} ${p.x-size},${p.y}`;
          return (
            <g key={`gate-${i}`}>
              <polygon points={points} fill="#ffeb3b" stroke="#fff" strokeWidth="1.5">
                <title>{g.name} ({g.code}){g.destination ? ` → ${g.destination}` : ''}</title>
              </polygon>
              <text x={p.x+10} y={p.y-10} fill="#e8eef5" fontSize="9">{g.code}</text>
            </g>
          );
        })}

        {/* Ships (colored triangles) */}
        {mapData.ships.map((sh, i) => {
          const p = mapPoint(sh.position.x, sh.position.z);
          const owner = (sh.owner || 'unknown').toLowerCase();
          const color = FACTION_COLORS[owner] || FACTION_COLORS.unknown;
          const size = 3;
          const points = `${p.x},${p.y-size} ${p.x-size*0.866},${p.y+size*0.5} ${p.x+size*0.866},${p.y+size*0.5}`;
          return (
            <g key={`ship-${i}`}>
              <polygon points={points} fill={color} stroke="#fff" strokeWidth="0.8">
                <title>{sh.name} ({owner}) - {sh.code}</title>
              </polygon>
            </g>
          );
        })}

        {/* Stations (blue circles) */}
        {mapData.stations.map((s, i) => {
          const p = mapPoint(s.position.x, s.position.z);
          return (
            <g key={`sta-${i}`}>
              <circle cx={p.x} cy={p.y} r="5" fill="#1f77b4" stroke="#fff" strokeWidth="1.5">
                <title>{s.name} ({s.code})</title>
              </circle>
              <text x={p.x+8} y={p.y-8} fill="#e8eef5" fontSize="10">{s.code}</text>
            </g>
          );
        })}
      </svg>
    );
  }, [mapData, zoom]);

  if (!sectorCode) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={e => e.stopPropagation()} style={{maxWidth: '1300px', maxHeight: '90vh', overflow: 'auto'}}>
        <div className="modal-header">
          <h2>Sector Map: {sectorName || sectorCode}</h2>
          <div style={{display: 'flex', gap: '8px', alignItems: 'center'}}>
            <button onClick={() => setZoom(z => Math.max(0.5, z - 0.25))} className="btn-secondary" style={{padding: '4px 12px'}}>−</button>
            <span style={{minWidth: '60px', textAlign: 'center', fontSize: '14px'}}>{(zoom * 100).toFixed(0)}%</span>
            <button onClick={() => setZoom(z => Math.min(5, z + 0.25))} className="btn-secondary" style={{padding: '4px 12px'}}>+</button>
            <button className="modal-close" onClick={onClose}>✕</button>
          </div>
        </div>

        {loading && <div style={{padding: '3rem', textAlign: 'center'}}>Loading map...</div>}
        {error && <div style={{padding: '3rem', textAlign: 'center', color: 'var(--down)'}}>Error: {error}</div>}

        {mapData && (
          <div style={{padding: '1rem'}}>
            {svgContent}

            <div style={{marginTop: '1rem', display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '1rem', fontSize: '13px'}}>
              <div>
                <strong>Stations:</strong> {mapData.stations.length} &nbsp;
                <strong>Ships:</strong> {mapData.ships.length} &nbsp;
                <strong>Gates:</strong> {mapData.gates.length}
              </div>
              <div>
                <strong>Asteroids:</strong> {mapData.asteroids.length} &nbsp;
                <strong>Resources:</strong> {mapData.resources.length}
              </div>
            </div>

            <div style={{marginTop: '1rem', fontSize: '12px', color: 'var(--muted)'}}>
              Legend: <span style={{color: '#1f77b4'}}>●</span> Stations (blue) &nbsp;
              <span style={{color: '#4dabf7'}}>●</span> Resources (cyan) &nbsp;
              <span style={{color: '#ffeb3b'}}>◆</span> Gates (yellow) &nbsp;
              <span style={{color: '#555'}}>●</span> Asteroids (grey) &nbsp;
              ▲ Ships (by faction color)
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

window.Components = window.Components || {};
window.Components.SectorMapModal = SectorMapModal;

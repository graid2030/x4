// Player Assets Card (<= 200 lines)

const { useEffect, useMemo, useState } = React;

function Row({ children }) {
  return <div style={{display:'grid',gridTemplateColumns:'minmax(240px,1fr) 1fr 140px 280px',gap:8,padding:'2px 0'}}>{children}</div>;
}

function AssetsCard() {
  const [data, setData] = useState(null);
  const [err, setErr] = useState(null);

  useEffect(() => {
    (async () => {
      try {
        const d = await window.API.loadPlayerProperty();
        setData(d);
      } catch (e) {
        setErr('Assets unavailable');
      }
    })();
  }, []);

  const assets = data?.assets || [];
  const ships = useMemo(() => assets.filter(a => (a.class || '').includes('ship')), [assets]);
  const stations = useMemo(() => assets.filter(a => (a.class || '') === 'station'), [assets]);
  const npcs = data?.npcs || [];
  const idToName = useMemo(() => Object.fromEntries(assets.map(a => [a.id, a.name])), [assets]);

  return (
    <section className="card">
      <h2 style={{marginBottom:6}}>Player Property</h2>
      {!data && !err && <div className="subtle">Loading…</div>}
      {err && <div className="subtle" style={{color:'#c33'}}>{err}</div>}
      {data && (
        <>
          <div className="toolbar" style={{gap:8}}>
            <div className="badge">Stations: {stations.length}</div>
            <div className="badge">Ships: {ships.length}</div>
            <div className="badge">NPCs: {npcs.length}</div>
          </div>

          {stations.length > 0 && (
            <div style={{marginTop:8}}>
              <div className="subtle" style={{fontWeight:600, marginBottom:4}}>Stations</div>
              <Row>
                <strong>Name</strong>
                <strong>Sector</strong>
                <strong>Code</strong>
                <strong>Macro</strong>
              </Row>
              {stations.map(s => (
                <Row key={s.id}>
                  <div title={s.id}>{s.name}</div>
                  <div>{s.sector_name || s.sector_code || '-'}</div>
                  <div>{s.code || '-'}</div>
                  <div className="subtle" title={s.macro_name || ''}>{s.macro_name || '-'}</div>
                </Row>
              ))}
            </div>
          )}

          {ships.length > 0 && (
            <div style={{marginTop:12}}>
              <div className="subtle" style={{fontWeight:600, marginBottom:4}}>Ships</div>
              <Row>
                <strong>Name</strong>
                <strong>Sector</strong>
                <strong>Code</strong>
                <strong>Type</strong>
              </Row>
              {ships.map(s => (
                <Row key={s.id}>
                  <div title={s.id}>{s.name}</div>
                  <div>{s.sector_name || s.sector_code || '-'}</div>
                  <div>{s.code || '-'}</div>
                  <div className="subtle">{s.ship_type || s.cargo_type || '-'}</div>
                </Row>
              ))}
            </div>
          )}

          {npcs.length > 0 && (
            <div style={{marginTop:12}}>
              <div className="subtle" style={{fontWeight:600, marginBottom:4}}>NPCs</div>
              <div style={{display:'grid',gridTemplateColumns:'minmax(240px,1fr) 100px 100px 100px 120px 100px',gap:8}}>
                <strong>Name</strong>
                <strong>Piloting</strong>
                <strong>Eng</strong>
                <strong>Board</strong>
                <strong>Management</strong>
                <strong>Morale</strong>
                {npcs.map(n => (
                  <React.Fragment key={n.id}>
                    <div title={n.code || ''}>{n.name}</div>
                    <div>{n.piloting}</div>
                    <div>{n.engineering}</div>
                    <div>{n.boarding}</div>
                    <div>{n.management}</div>
                    <div>{n.morale}</div>
                  </React.Fragment>
                ))}
              </div>
            </div>
          )}

          {/* Wings removed as requested */}
        </>
      )}
    </section>
  );
}

window.Components = window.Components || {};
window.Components.AssetsCard = AssetsCard;

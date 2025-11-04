// Pilot Card (<= 200 lines)

const { useEffect, useState } = React;

function PilotCard() {
  const [pilot, setPilot] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    (async () => {
      try {
        const p = await window.API.loadPilot();
        setPilot(p);
      } catch (e) {
        setError('Pilot info unavailable');
      }
    })();
  }, []);

  const creditsText = pilot ? `${(pilot.credits ?? 0).toLocaleString()} Cr` : '—';
  const locationText = pilot?.location ? pilot.location : null;

  return (
    <section className="card">
      <h2 style={{marginBottom:6}}>Pilot</h2>
      {pilot && (
        <>
          <div className="subtle">{pilot.name} — {creditsText}</div>
          {locationText && <div className="subtle">{locationText}</div>}
        </>
      )}
      {!pilot && !error && <div className="subtle">Loading…</div>}
      {error && <div className="subtle" style={{color:'#c33'}}>{error}</div>}
    </section>
  );
}

window.Components = window.Components || {};
window.Components.PilotCard = PilotCard;

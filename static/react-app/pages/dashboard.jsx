// Dashboard Page - Full implementation (<= 250 lines per CLAUDE.md)

const { useState, useEffect } = React;
const { Layout, PageContainer } = window.Components;
const { useRouter } = window.Router;

function DashboardPage() {
  const { navigate } = useRouter();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [data, setData] = useState(null);

  useEffect(() => {
    loadDashboard();
  }, []);

  async function loadDashboard() {
    setLoading(true);
    setError(null);
    try {
      const response = await window.API.getDashboard();
      setData(response);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }

  function formatCredits(amount) {
    if (amount >= 1000000) {
      return `${(amount / 1000000).toFixed(2)}M Cr`;
    } else if (amount >= 1000) {
      return `${(amount / 1000).toFixed(1)}K Cr`;
    }
    return `${amount.toLocaleString()} Cr`;
  }

  if (loading) {
    return (
      <Layout>
        <PageContainer title="Dashboard" subtitle="Loading...">
          <div className="card">
            <p>Loading dashboard data...</p>
          </div>
        </PageContainer>
      </Layout>
    );
  }

  if (error) {
    return (
      <Layout>
        <PageContainer title="Dashboard" subtitle="Error">
          <div className="card">
            <p style={{ color: '#ef4444' }}>Error: {error}</p>
            <button className="btn-primary" onClick={loadDashboard} style={{ marginTop: '12px' }}>
              Retry
            </button>
          </div>
        </PageContainer>
      </Layout>
    );
  }

  return (
    <Layout>
      <PageContainer
        title="Dashboard"
        subtitle="Overview of your trading empire"
        actions={
          <button className="btn-secondary" onClick={loadDashboard}>
            🔄 Refresh
          </button>
        }
      >
        {/* Player Profile & Quick Stats */}
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '12px', marginBottom: '20px' }}>
          {/* Player Card */}
          <div className="card" style={{ padding: '16px', gridColumn: 'span 2' }}>
            <div style={{ fontSize: '12px', color: 'var(--muted)', marginBottom: '8px' }}>Player</div>
            <div style={{ fontSize: '20px', fontWeight: '700', marginBottom: '4px' }}>{data.player.name}</div>
            <div style={{ fontSize: '24px', fontWeight: '700', color: '#10b981', marginBottom: '4px' }}>
              {formatCredits(data.player.money)}
            </div>
            {data.player.location && (
              <div style={{ fontSize: '12px', color: 'var(--muted)' }}>
                Location: {data.player.location}
              </div>
            )}
          </div>

          {/* Stats Cards */}
          <div className="card" style={{ padding: '16px' }}>
            <div style={{ fontSize: '12px', color: 'var(--muted)', marginBottom: '4px' }}>Sectors</div>
            <div style={{ fontSize: '24px', fontWeight: '700' }}>{data.stats.sectors}</div>
          </div>
          <div className="card" style={{ padding: '16px' }}>
            <div style={{ fontSize: '12px', color: 'var(--muted)', marginBottom: '4px' }}>Stations</div>
            <div style={{ fontSize: '24px', fontWeight: '700' }}>{data.stats.stations}</div>
          </div>
          <div className="card" style={{ padding: '16px' }}>
            <div style={{ fontSize: '12px', color: 'var(--muted)', marginBottom: '4px' }}>Ships</div>
            <div style={{ fontSize: '24px', fontWeight: '700' }}>{data.stats.ships}</div>
          </div>
          <div className="card" style={{ padding: '16px' }}>
            <div style={{ fontSize: '12px', color: 'var(--muted)', marginBottom: '4px' }}>NPCs</div>
            <div style={{ fontSize: '24px', fontWeight: '700' }}>{data.stats.npcs}</div>
          </div>
        </div>

        {/* Top 5 Trade Routes */}
        <div className="card" style={{ marginBottom: '16px' }}>
          <h3 style={{ margin: '0 0 16px 0', fontSize: '16px', fontWeight: '600' }}>
            Top 5 Profitable Routes
          </h3>
          {data.top_routes.length === 0 ? (
            <p style={{ color: 'var(--muted)' }}>No profitable routes found</p>
          ) : (
            <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
              {data.top_routes.map((route, index) => (
                <div
                  key={index}
                  className="trade-route-card"
                  style={{
                    padding: '12px',
                    background: 'var(--card-hover)',
                    borderRadius: '6px',
                    display: 'grid',
                    gridTemplateColumns: '40px 1fr auto',
                    gap: '12px',
                    alignItems: 'center',
                  }}
                >
                  {/* Rank */}
                  <div
                    style={{
                      width: '32px',
                      height: '32px',
                      borderRadius: '50%',
                      background: index === 0 ? '#fbbf24' : index === 1 ? '#94a3b8' : index === 2 ? '#c2410c' : 'var(--border)',
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'center',
                      fontWeight: '700',
                      fontSize: '14px',
                    }}
                  >
                    {index + 1}
                  </div>

                  {/* Route Info */}
                  <div style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
                    <div style={{ fontWeight: '600', fontSize: '14px' }}>{route.ware_name}</div>
                    <div style={{ fontSize: '12px', color: 'var(--muted)' }}>
                      <span style={{ color: '#10b981' }}>Buy:</span> {route.buy_station} ({route.buy_sector})
                      {' → '}
                      <span style={{ color: '#3b82f6' }}>Sell:</span> {route.sell_station} ({route.sell_sector})
                    </div>
                    <div style={{ fontSize: '11px', color: 'var(--muted)' }}>
                      {route.amount.toLocaleString()} units @ {route.buy_price} Cr → {route.sell_price} Cr
                    </div>
                  </div>

                  {/* Profit */}
                  <div style={{ textAlign: 'right' }}>
                    <div style={{ fontSize: '18px', fontWeight: '700', color: '#10b981' }}>
                      {formatCredits(route.profit)}
                    </div>
                    <div style={{ fontSize: '11px', color: 'var(--muted)' }}>profit</div>
                  </div>
                </div>
              ))}
            </div>
          )}
          <div style={{ marginTop: '16px', textAlign: 'center' }}>
            <button
              className="btn-primary"
              onClick={() => navigate('/trade-routes')}
            >
              View All Routes →
            </button>
          </div>
        </div>

        {/* Sector Summary */}
        <div className="card">
          <h3 style={{ margin: '0 0 16px 0', fontSize: '16px', fontWeight: '600' }}>
            Top Sectors by Activity
          </h3>
          {data.sector_summary.length === 0 ? (
            <p style={{ color: 'var(--muted)' }}>No sectors found</p>
          ) : (
            <div
              style={{
                display: 'grid',
                gridTemplateColumns: 'repeat(auto-fill, minmax(280px, 1fr))',
                gap: '12px',
              }}
            >
              {data.sector_summary.map((sector) => (
                <div
                  key={sector.code}
                  className="sector-card"
                  style={{
                    padding: '12px',
                    background: 'var(--card-hover)',
                    borderRadius: '6px',
                    cursor: 'pointer',
                    transition: 'all 0.2s',
                  }}
                  onClick={() => navigate(`/sectors/${sector.code}`)}
                  onMouseEnter={(e) => {
                    e.currentTarget.style.transform = 'translateY(-2px)';
                    e.currentTarget.style.boxShadow = '0 4px 12px rgba(0, 0, 0, 0.15)';
                  }}
                  onMouseLeave={(e) => {
                    e.currentTarget.style.transform = 'translateY(0)';
                    e.currentTarget.style.boxShadow = 'none';
                  }}
                >
                  <div style={{ fontWeight: '600', fontSize: '13px', marginBottom: '4px' }}>
                    {sector.name}
                  </div>
                  <div style={{ fontSize: '11px', color: 'var(--muted)', marginBottom: '8px' }}>
                    {sector.code} • {sector.owner_name || 'Unowned'}
                  </div>
                  <div style={{ display: 'flex', gap: '12px', fontSize: '11px' }}>
                    <div>
                      <span style={{ color: 'var(--muted)' }}>Stations:</span>{' '}
                      <span style={{ fontWeight: '600' }}>{sector.station_count}</span>
                    </div>
                    {sector.player_stations > 0 && (
                      <div>
                        <span style={{ color: 'var(--muted)' }}>Player:</span>{' '}
                        <span style={{ fontWeight: '600', color: '#10b981' }}>
                          {sector.player_stations}
                        </span>
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>
          )}
          <div style={{ marginTop: '16px', textAlign: 'center' }}>
            <button
              className="btn-primary"
              onClick={() => navigate('/sectors')}
            >
              View All Sectors →
            </button>
          </div>
        </div>
      </PageContainer>
    </Layout>
  );
}

window.Pages = { ...window.Pages, DashboardPage };

// Sector Detail Page (<= 200 lines)

const { useState, useEffect } = React;
const { Layout, PageContainer } = window.Components;
const { useRouter } = window.Router;
const { Table, Column } = ReactVirtualized;

function SectorDetailPage() {
  const { currentPath, navigate } = useRouter();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [data, setData] = useState(null);

  // Filters for trade offers
  const [tradeFilter, setTradeFilter] = useState('all'); // all, buy, sell
  const [wareFilter, setWareFilter] = useState('');

  // Extract sector code from URL
  const sectorCode = currentPath.split('/').pop();

  useEffect(() => {
    loadSectorDetail();
  }, [sectorCode]);

  async function loadSectorDetail() {
    setLoading(true);
    setError(null);
    try {
      const response = await window.API.getSectorDetail(sectorCode);
      setData(response);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }

  // Filter trade offers
  const filteredTrades = data ? data.trade_offers.filter(t => {
    const matchesType = tradeFilter === 'all' || t.trade_type === tradeFilter;
    const matchesWare = !wareFilter || t.ware_name.toLowerCase().includes(wareFilter.toLowerCase());
    return matchesType && matchesWare;
  }) : [];

  if (loading) {
    return (
      <Layout>
        <PageContainer title="Sector Detail" subtitle="Loading...">
          <div className="card">
            <p>Loading sector details...</p>
          </div>
        </PageContainer>
      </Layout>
    );
  }

  if (error) {
    return (
      <Layout>
        <PageContainer title="Sector Detail" subtitle="Error">
          <div className="card">
            <p style={{color: '#ef4444'}}>Error: {error}</p>
            <button className="btn-primary" onClick={() => navigate('/sectors')} style={{marginTop: '12px'}}>
              Back to Sectors
            </button>
          </div>
        </PageContainer>
      </Layout>
    );
  }

  return (
    <Layout>
      <PageContainer
        title={`${data.code} - ${data.name}`}
        subtitle={data.owner_name || 'No owner'}
        actions={
          <button className="btn-secondary" onClick={() => navigate('/sectors')}>
            ← Back to List
          </button>
        }
      >
        {/* Stats Cards */}
        <div style={{display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(150px, 1fr))', gap: '12px', marginBottom: '20px'}}>
          <div className="card" style={{padding: '12px'}}>
            <div style={{fontSize: '11px', color: 'var(--muted)', marginBottom: '4px'}}>Stations</div>
            <div style={{fontSize: '20px', fontWeight: '700'}}>{data.stats.total_stations}</div>
          </div>
          <div className="card" style={{padding: '12px'}}>
            <div style={{fontSize: '11px', color: 'var(--muted)', marginBottom: '4px'}}>Player Stations</div>
            <div style={{fontSize: '20px', fontWeight: '700'}}>{data.stats.player_stations}</div>
          </div>
          <div className="card" style={{padding: '12px'}}>
            <div style={{fontSize: '11px', color: 'var(--muted)', marginBottom: '4px'}}>Buy Offers</div>
            <div style={{fontSize: '20px', fontWeight: '700'}}>{data.stats.buy_offers}</div>
          </div>
          <div className="card" style={{padding: '12px'}}>
            <div style={{fontSize: '11px', color: 'var(--muted)', marginBottom: '4px'}}>Sell Offers</div>
            <div style={{fontSize: '20px', fontWeight: '700'}}>{data.stats.sell_offers}</div>
          </div>
          <div className="card" style={{padding: '12px'}}>
            <div style={{fontSize: '11px', color: 'var(--muted)', marginBottom: '4px'}}>Unique Wares</div>
            <div style={{fontSize: '20px', fontWeight: '700'}}>{data.stats.unique_wares}</div>
          </div>
        </div>

        {/* Stations Section */}
        <div className="card" style={{marginBottom: '16px'}}>
          <h3 style={{margin: '0 0 12px 0', fontSize: '16px', fontWeight: '600'}}>Stations ({data.stations.length})</h3>
          {data.stations.length === 0 ? (
            <p style={{color: 'var(--muted)'}}>No stations in this sector</p>
          ) : (
            <Table
              width={1400}
              height={Math.min(300, data.stations.length * 40 + 40)}
              headerHeight={40}
              rowHeight={40}
              rowCount={data.stations.length}
              rowGetter={({ index }) => data.stations[index]}
              rowClassName={({ index }) => index % 2 === 0 ? 'rv-row even' : 'rv-row odd'}
            >
              <Column label="Code" dataKey="code" width={120} />
              <Column label="Name" dataKey="name" width={300} flexGrow={1} />
              <Column label="Owner" dataKey="owner_name" width={180} />
              <Column
                label="Position (X)"
                dataKey="position"
                width={120}
                className="col-num"
                cellRenderer={({ cellData }) => cellData[0].toFixed(0)}
              />
              <Column
                label="Position (Y)"
                dataKey="position"
                width={120}
                className="col-num"
                cellRenderer={({ cellData }) => cellData[1].toFixed(0)}
              />
              <Column
                label="Position (Z)"
                dataKey="position"
                width={120}
                className="col-num"
                cellRenderer={({ cellData }) => cellData[2].toFixed(0)}
              />
            </Table>
          )}
        </div>

        {/* Trade Offers Section */}
        <div className="card">
          <div style={{display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '12px'}}>
            <h3 style={{margin: 0, fontSize: '16px', fontWeight: '600'}}>
              Trade Offers ({filteredTrades.length})
            </h3>
            <div style={{display: 'flex', gap: '12px'}}>
              <select
                value={tradeFilter}
                onChange={(e) => setTradeFilter(e.target.value)}
                className="search-input"
                style={{width: '120px'}}
              >
                <option value="all">All Types</option>
                <option value="buy">Buy Only</option>
                <option value="sell">Sell Only</option>
              </select>
              <input
                type="text"
                className="search-input"
                placeholder="Filter by ware..."
                value={wareFilter}
                onChange={(e) => setWareFilter(e.target.value)}
                style={{width: '200px'}}
              />
            </div>
          </div>

          {filteredTrades.length === 0 ? (
            <p style={{color: 'var(--muted)'}}>No trade offers match the filters</p>
          ) : (
            <Table
              width={1400}
              height={400}
              headerHeight={40}
              rowHeight={40}
              rowCount={filteredTrades.length}
              rowGetter={({ index }) => filteredTrades[index]}
              rowClassName={({ index }) => index % 2 === 0 ? 'rv-row even' : 'rv-row odd'}
            >
              <Column label="Ware" dataKey="ware_name" width={200} flexGrow={1} />
              <Column
                label="Type"
                dataKey="trade_type"
                width={80}
                cellRenderer={({ cellData }) => (
                  <span style={{
                    color: cellData === 'buy' ? '#10b981' : '#3b82f6',
                    fontWeight: '600',
                    textTransform: 'uppercase'
                  }}>
                    {cellData}
                  </span>
                )}
              />
              <Column label="Price" dataKey="price" width={100} className="col-num" />
              <Column label="Amount" dataKey="amount" width={100} className="col-num" />
              <Column label="Station" dataKey="station_name" width={300} />
            </Table>
          )}
        </div>
      </PageContainer>
    </Layout>
  );
}

window.Pages = { ...window.Pages, SectorDetailPage };

// Sectors List Page - Full implementation (<= 200 lines)

const { useState, useEffect } = React;
const { Layout, PageContainer } = window.Components;
const { useRouter } = window.Router;
const { Table, Column } = ReactVirtualized;

function SectorsPage() {
  const { navigate } = useRouter();
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [data, setData] = useState(null);
  const [filteredSectors, setFilteredSectors] = useState([]);

  // Filters
  const [searchQuery, setSearchQuery] = useState('');
  const [ownerFilter, setOwnerFilter] = useState('all');
  const [sortBy, setSortBy] = useState({ key: 'name', direction: 'ASC' });

  // Load sectors on mount
  useEffect(() => {
    loadSectors();
  }, []);

  // Apply filters when data or filters change
  useEffect(() => {
    if (data) {
      applyFilters();
    }
  }, [data, searchQuery, ownerFilter, sortBy]);

  async function loadSectors() {
    setLoading(true);
    setError(null);
    try {
      const response = await window.API.getSectorsList();
      setData(response);
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  }

  function applyFilters() {
    let sectors = [...data.sectors];

    // Search filter
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      sectors = sectors.filter(s =>
        s.name.toLowerCase().includes(query) ||
        s.code.toLowerCase().includes(query) ||
        (s.owner_name && s.owner_name.toLowerCase().includes(query))
      );
    }

    // Owner filter
    if (ownerFilter !== 'all') {
      sectors = sectors.filter(s => s.owner === ownerFilter);
    }

    // Sort
    sectors.sort((a, b) => {
      let aVal = a[sortBy.key];
      let bVal = b[sortBy.key];

      // Handle null/undefined
      if (aVal == null) aVal = '';
      if (bVal == null) bVal = '';

      // String comparison (case-insensitive for text)
      if (typeof aVal === 'string') {
        aVal = aVal.toLowerCase();
        bVal = bVal.toLowerCase();
      }

      if (sortBy.direction === 'ASC') {
        return aVal > bVal ? 1 : aVal < bVal ? -1 : 0;
      } else {
        return aVal < bVal ? 1 : aVal > bVal ? -1 : 0;
      }
    });

    setFilteredSectors(sectors);
  }

  function handleSort({ sortBy: key }) {
    setSortBy(prev => ({
      key,
      direction: prev.key === key && prev.direction === 'ASC' ? 'DESC' : 'ASC',
    }));
  }

  function handleRowClick({ rowData }) {
    navigate(`/sectors/${rowData.code}`);
  }

  // Get unique owners for filter
  const owners = data
    ? [...new Set(data.sectors.map(s => s.owner).filter(Boolean))]
    : [];

  if (loading) {
    return (
      <Layout>
        <PageContainer title="Sectors" subtitle="Loading...">
          <div className="card">
            <p>Loading sectors data...</p>
          </div>
        </PageContainer>
      </Layout>
    );
  }

  if (error) {
    return (
      <Layout>
        <PageContainer title="Sectors" subtitle="Error">
          <div className="card">
            <p style={{color: '#ef4444'}}>Error: {error}</p>
            <button className="btn-primary" onClick={loadSectors} style={{marginTop: '12px'}}>
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
        title="Sectors"
        subtitle={`${filteredSectors.length} of ${data.stats.total_sectors} sectors`}
        actions={
          <button className="btn-secondary" onClick={loadSectors}>
            🔄 Refresh
          </button>
        }
      >
        {/* Stats Cards */}
        <div style={{display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '12px', marginBottom: '20px'}}>
          <div className="card" style={{padding: '16px'}}>
            <div style={{fontSize: '12px', color: 'var(--muted)', marginBottom: '4px'}}>Total Sectors</div>
            <div style={{fontSize: '24px', fontWeight: '700'}}>{data.stats.total_sectors}</div>
          </div>
          <div className="card" style={{padding: '16px'}}>
            <div style={{fontSize: '12px', color: 'var(--muted)', marginBottom: '4px'}}>Player Owned</div>
            <div style={{fontSize: '24px', fontWeight: '700'}}>{data.stats.player_owned}</div>
          </div>
          <div className="card" style={{padding: '16px'}}>
            <div style={{fontSize: '12px', color: 'var(--muted)', marginBottom: '4px'}}>Total Stations</div>
            <div style={{fontSize: '24px', fontWeight: '700'}}>{data.stats.total_stations}</div>
          </div>
          <div className="card" style={{padding: '16px'}}>
            <div style={{fontSize: '12px', color: 'var(--muted)', marginBottom: '4px'}}>Discovered</div>
            <div style={{fontSize: '24px', fontWeight: '700'}}>{data.stats.discovered}</div>
          </div>
        </div>

        {/* Filters */}
        <div className="card" style={{marginBottom: '16px'}}>
          <div style={{display: 'grid', gridTemplateColumns: '1fr 200px', gap: '12px'}}>
            <div>
              <label>Search</label>
              <input
                type="text"
                className="search-input"
                placeholder="Search by name, code, or owner..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
              />
            </div>
            <div>
              <label>Owner</label>
              <select
                className="search-input"
                value={ownerFilter}
                onChange={(e) => setOwnerFilter(e.target.value)}
                style={{width: '100%'}}
              >
                <option value="all">All Owners</option>
                {owners.map(owner => (
                  <option key={owner} value={owner}>
                    {data.sectors.find(s => s.owner === owner)?.owner_name || owner}
                  </option>
                ))}
              </select>
            </div>
          </div>
        </div>

        {/* Table */}
        <div className="card">
          <Table
            width={1500}
            height={600}
            headerHeight={40}
            rowHeight={40}
            rowCount={filteredSectors.length}
            rowGetter={({ index }) => filteredSectors[index]}
            onRowClick={handleRowClick}
            rowClassName={({ index }) => index % 2 === 0 ? 'rv-row even' : 'rv-row odd'}
            sort={handleSort}
            sortBy={sortBy.key}
            sortDirection={sortBy.direction}
          >
            <Column label="Code" dataKey="code" width={120} />
            <Column label="Name" dataKey="name" width={250} flexGrow={1} />
            <Column label="Owner" dataKey="owner_name" width={180} />
            <Column label="Stations" dataKey="station_count" width={100} className="col-num" />
            <Column label="Player" dataKey="player_stations" width={100} className="col-num" />
            <Column label="Buy Offers" dataKey="buy_offers" width={120} className="col-num" />
            <Column label="Sell Offers" dataKey="sell_offers" width={120} className="col-num" />
            <Column label="Total Trades" dataKey="total_trades" width={120} className="col-num" />
          </Table>
        </div>
      </PageContainer>
    </Layout>
  );
}

window.Pages = { ...window.Pages, SectorsPage };

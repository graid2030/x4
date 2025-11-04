// Assets Page - Player property management

const { Layout, PageContainer, AssetsCard } = window.Components;

function AssetsPage() {
  return (
    <Layout>
      <PageContainer
        title="Assets"
        subtitle="Manage your ships, stations, and fleets"
      >
        <AssetsCard />
        <div className="card" style={{marginTop: '16px'}}>
          <h2>Enhanced Features Coming Soon</h2>
          <p>Future enhancements will include:</p>
          <ul>
            <li>Tabs: Ships / Stations / Fleet Overview</li>
            <li>Filter by sector, type, size</li>
            <li>Fleet hierarchies (commander/subordinates)</li>
            <li>Crew assignments</li>
            <li>Cargo tracking</li>
            <li>Ship detail modals with equipment</li>
          </ul>
        </div>
      </PageContainer>
    </Layout>
  );
}

window.Pages = { ...window.Pages, AssetsPage };

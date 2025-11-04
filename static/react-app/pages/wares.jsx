// Wares Page - Ware analysis and price tracking

const { Layout, PageContainer } = window.Components;

function WaresPage() {
  return (
    <Layout>
      <PageContainer
        title="Wares"
        subtitle="Analyze trade goods and prices"
      >
        <div className="card">
          <h2>Wares List</h2>
          <p>This page will show:</p>
          <ul>
            <li>Table of all tradeable wares</li>
            <li>Average buy/sell prices</li>
            <li>Best profit margins</li>
            <li>Number of stations selling</li>
            <li>Click ware to see detailed price analysis</li>
          </ul>
        </div>
      </PageContainer>
    </Layout>
  );
}

window.Pages = { ...window.Pages, WaresPage };

// Dashboard Page - Overview of game state

const { Layout, PageContainer } = window.Components;

function DashboardPage() {
  return (
    <Layout>
      <PageContainer
        title="Dashboard"
        subtitle="Quick overview of your trading empire"
      >
        <div className="card">
          <h2>Welcome to X4 Trading System V2!</h2>
          <p>Dashboard page coming soon...</p>
          <p>This page will show:</p>
          <ul>
            <li>Player profile and credits</li>
            <li>Quick stats (sectors, stations, ships)</li>
            <li>Top 5 profitable routes</li>
            <li>Sector quick view cards</li>
            <li>Recent activity log</li>
          </ul>
        </div>
      </PageContainer>
    </Layout>
  );
}

window.Pages = { ...window.Pages, DashboardPage };

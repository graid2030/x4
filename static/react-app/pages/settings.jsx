// Settings Page - Configuration and preferences

const { Layout, PageContainer } = window.Components;

function SettingsPage() {
  return (
    <Layout>
      <PageContainer
        title="Settings"
        subtitle="Configure your trading system"
      >
        <div className="card">
          <h2>Settings</h2>
          <p>This page will include:</p>
          <ul>
            <li>Game paths configuration</li>
            <li>Display options (hide undiscovered, faction colors)</li>
            <li>UI preferences (font size, theme)</li>
            <li>Cache management</li>
            <li>Export functionality</li>
          </ul>
        </div>
      </PageContainer>
    </Layout>
  );
}

window.Pages = { ...window.Pages, SettingsPage };

// Root App V2 - Multi-Page Router (<= 200 lines)

const { useState, useEffect } = React;
const { Router } = window.Router;
const { Setup, SaveSwitcher } = window.Components;
const {
  DashboardPage,
  TradeRoutesPage,
  SectorsPage,
  SectorDetailPage,
  WaresPage,
  AssetsPage,
  AnalyticsPage,
  NPCsPage,
  SettingsPage,
  NotFoundPage,
} = window.Pages;

function App() {
  const [initialized, setInitialized] = useState(false);
  const [fontScale, setFontScale] = useState(Number(localStorage.getItem('ui_font_scale') || '1'));

  // Check if system is already initialized
  useEffect(() => {
    (async () => {
      try {
        const s = await window.API.status();
        if (s.initialized) setInitialized(true);
      } catch {}
    })();
  }, []);

  // Apply font scale
  useEffect(() => {
    document.documentElement.style.setProperty('--font-scale', String(fontScale));
    localStorage.setItem('ui_font_scale', String(fontScale));
  }, [fontScale]);

  // If not initialized, show setup screen
  if (!initialized) {
    return (
      <div className="container">
        <header className="page-header">
          <h1>X4: Foundations Trading System</h1>
          <p className="subtle">Configure paths to get started</p>
        </header>
        <Setup onInitialized={() => setInitialized(true)} />
      </div>
    );
  }

  // Define routes
  const routes = [
    { path: '/', component: DashboardPage },
    { path: '/trade-routes', component: TradeRoutesPage },
    { path: '/sectors/:code', component: SectorDetailPage },
    { path: '/sectors', component: SectorsPage },
    { path: '/wares', component: WaresPage },
    { path: '/assets', component: AssetsPage },
    { path: '/analytics', component: AnalyticsPage },
    { path: '/npcs', component: NPCsPage },
    { path: '/settings', component: SettingsPage },
  ];

  // Main app with router
  return <Router routes={routes} fallback={NotFoundPage} />;
}

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<App />);

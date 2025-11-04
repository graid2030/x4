// Navigation Bar Component (<= 200 lines per CLAUDE.md)

const { useState, useEffect } = React;
const { useRouter, Link } = window.Router;

function Navigation() {
  const { currentPath } = useRouter();
  const [pilot, setPilot] = useState(null);
  const [saveFile, setSaveFile] = useState(null);

  // Load pilot info
  useEffect(() => {
    loadPilotInfo();
  }, []);

  async function loadPilotInfo() {
    try {
      const pilotData = await window.API.loadPilot();
      setPilot(pilotData);
    } catch (err) {
      console.error('Failed to load pilot:', err);
    }
  }

  // Format credits with commas
  function formatCredits(amount) {
    if (!amount) return '0';
    return amount.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',');
  }

  // Navigation tabs
  const tabs = [
    { path: '/', label: 'Dashboard', icon: '🏠' },
    { path: '/trade-routes', label: 'Trade Routes', icon: '🚀' },
    { path: '/sectors', label: 'Sectors', icon: '🗺️' },
    { path: '/wares', label: 'Wares', icon: '📦' },
    { path: '/assets', label: 'Assets', icon: '🏗️' },
    { path: '/analytics', label: 'Analytics', icon: '📊' },
    { path: '/npcs', label: 'NPCs', icon: '👥' },
    { path: '/settings', label: 'Settings', icon: '⚙️' },
  ];

  // Check if path is active (exact or starts with for sub-routes)
  function isActive(tabPath) {
    if (tabPath === '/') {
      return currentPath === '/';
    }
    return currentPath.startsWith(tabPath);
  }

  return (
    <nav className="navigation">
      <div className="nav-header">
        <div className="nav-brand">
          <h1 className="nav-title">X4 Trading System</h1>
          <p className="nav-subtitle">v2.0</p>
        </div>
        {pilot && (
          <div className="nav-player-info">
            <div className="player-details">
              <span className="player-name">{pilot.name}</span>
              <span className="player-credits">
                {formatCredits(pilot.money)} Cr
              </span>
            </div>
            {pilot.location && (
              <div className="player-location">
                📍 {pilot.location}
              </div>
            )}
          </div>
        )}
      </div>

      <div className="nav-tabs">
        {tabs.map(tab => (
          <Link
            key={tab.path}
            to={tab.path}
            className={`nav-tab ${isActive(tab.path) ? 'active' : ''}`}
          >
            <span className="nav-tab-icon">{tab.icon}</span>
            <span className="nav-tab-label">{tab.label}</span>
          </Link>
        ))}
      </div>
    </nav>
  );
}

// Export to window
window.Components = { ...window.Components, Navigation };

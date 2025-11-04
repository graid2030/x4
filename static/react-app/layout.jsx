// Layout Component with Breadcrumbs (<= 200 lines per CLAUDE.md)

const { useRouter, Link } = window.Router;
const { Navigation } = window.Components;

// Breadcrumbs component
function Breadcrumbs() {
  const { currentPath } = useRouter();

  // Don't show breadcrumbs on home
  if (currentPath === '/') return null;

  // Parse path into segments
  const segments = currentPath.split('/').filter(Boolean);

  // Build breadcrumb trail
  const breadcrumbs = [{ label: 'Home', path: '/' }];

  let accumulatedPath = '';
  segments.forEach((segment, index) => {
    accumulatedPath += `/${segment}`;

    // Capitalize and format segment
    let label = segment
      .split('-')
      .map(word => word.charAt(0).toUpperCase() + word.slice(1))
      .join(' ');

    // If it looks like a code (AAM-257), keep it as-is
    if (/^[A-Z]{3}-\d{3}$/.test(segment)) {
      label = segment;
    }

    breadcrumbs.push({
      label,
      path: accumulatedPath,
    });
  });

  return (
    <div className="breadcrumbs">
      {breadcrumbs.map((crumb, index) => (
        <span key={crumb.path} className="breadcrumb-item">
          {index > 0 && <span className="breadcrumb-separator">›</span>}
          {index === breadcrumbs.length - 1 ? (
            <span className="breadcrumb-current">{crumb.label}</span>
          ) : (
            <Link to={crumb.path} className="breadcrumb-link">
              {crumb.label}
            </Link>
          )}
        </span>
      ))}
    </div>
  );
}

// Main layout wrapper
function Layout({ children }) {
  return (
    <div className="app-layout">
      <Navigation />
      <main className="main-content">
        <Breadcrumbs />
        <div className="page-content">
          {children}
        </div>
      </main>
    </div>
  );
}

// Page wrapper with common structure
function PageContainer({ title, subtitle, actions, children }) {
  return (
    <div className="page-container">
      {(title || actions) && (
        <div className="page-header">
          {title && (
            <div className="page-title-section">
              <h1 className="page-title">{title}</h1>
              {subtitle && <p className="page-subtitle">{subtitle}</p>}
            </div>
          )}
          {actions && <div className="page-actions">{actions}</div>}
        </div>
      )}
      <div className="page-body">
        {children}
      </div>
    </div>
  );
}

// Export to window
window.Components = {
  ...window.Components,
  Layout,
  PageContainer,
  Breadcrumbs,
};

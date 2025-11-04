// Client-side Router (<= 200 lines per CLAUDE.md)
// No external dependencies, uses browser History API

const { useState, useEffect, createContext, useContext } = React;

// Router Context
const RouterContext = createContext(null);

// Hook to access router
function useRouter() {
  const context = useContext(RouterContext);
  if (!context) {
    throw new Error('useRouter must be used within Router');
  }
  return context;
}

// Route matching with params support
function matchRoute(currentPath, routePattern) {
  const paramNames = [];

  // Convert pattern like "/sectors/:code" to regex
  const regexPattern = routePattern
    .replace(/\//g, '\\/')
    .replace(/:(\w+)/g, (_, paramName) => {
      paramNames.push(paramName);
      return '([^/]+)';
    });

  const regex = new RegExp(`^${regexPattern}$`);
  const match = currentPath.match(regex);

  if (!match) return null;

  // Extract params
  const params = {};
  paramNames.forEach((name, index) => {
    params[name] = match[index + 1];
  });

  return { params };
}

// Main Router component
function Router({ routes, fallback }) {
  const [currentPath, setCurrentPath] = useState(window.location.pathname);

  useEffect(() => {
    // Listen to browser back/forward
    const handlePopState = () => {
      setCurrentPath(window.location.pathname);
    };

    window.addEventListener('popstate', handlePopState);
    return () => window.removeEventListener('popstate', handlePopState);
  }, []);

  // Navigate programmatically
  function navigate(path, replace = false) {
    if (replace) {
      window.history.replaceState({}, '', path);
    } else {
      window.history.pushState({}, '', path);
    }
    setCurrentPath(path);
  }

  // Find matching route
  let matchedRoute = null;
  let params = {};

  for (const route of routes) {
    const match = matchRoute(currentPath, route.path);
    if (match) {
      matchedRoute = route;
      params = match.params;
      break;
    }
  }

  // Render matched component or fallback
  const ComponentToRender = matchedRoute ? matchedRoute.component : fallback;

  const contextValue = {
    currentPath,
    navigate,
    params,
  };

  return (
    <RouterContext.Provider value={contextValue}>
      <ComponentToRender />
    </RouterContext.Provider>
  );
}

// Link component for navigation
function Link({ to, children, className, replace = false }) {
  const { navigate } = useRouter();

  function handleClick(e) {
    e.preventDefault();
    navigate(to, replace);
  }

  return (
    <a href={to} onClick={handleClick} className={className}>
      {children}
    </a>
  );
}

// Redirect component
function Redirect({ to, replace = true }) {
  const { navigate } = useRouter();

  useEffect(() => {
    navigate(to, replace);
  }, [to, replace]);

  return null;
}

// Export to window for use in other JSX files
window.Router = {
  Router,
  useRouter,
  Link,
  Redirect,
};

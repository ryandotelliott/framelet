import { createRouter, createRoute, createRootRoute, Outlet, Link } from '@tanstack/react-router';
import Home from './routes/Home';
import TitleBar from './components/ui/TitleBar';

// Register the router instance for type safety
declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router;
  }
}

const rootRoute = createRootRoute({
  component: () => (
    <>
      <TitleBar />
      <div id="app">
        <nav className="flex items-center justify-between p-4 border-b border-gray-200 bg-gray-50">
          <Link to="/" className="text-2xl font-bold">
            Framelet
          </Link>
        </nav>
        <div className="p-4">
          <Outlet />
        </div>
      </div>
    </>
  ),
});

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/',
  component: Home,
});

const routeTree = rootRoute.addChildren([indexRoute]);
export const router = createRouter({ routeTree });

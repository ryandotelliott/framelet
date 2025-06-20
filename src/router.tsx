import { createRouter, createRoute, createRootRoute } from '@tanstack/react-router';
import RecordPage from './routes/record';
import App from './app';

// Register the router instance for type safety
declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router;
  }
}

const rootRoute = createRootRoute({
  component: App,
});

const indexRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: '/',
  component: RecordPage,
});

const routeTree = rootRoute.addChildren([indexRoute]);
export const router = createRouter({ routeTree });

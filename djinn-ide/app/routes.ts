import { type RouteConfig, index, route } from "@react-router/dev/routes";

export default [
  index("routes/home.tsx"),
  route("new", "routes/new/index.ts"),
  route("sprites", "routes/sprites.tsx"),
] satisfies RouteConfig;

import fs from "fs";
import { HttpProxy, UserConfig, defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import TOML, { TomlPrimitive } from "smol-toml";

// https://vitejs.dev/config/
export default defineConfig(({ command }) => {
  if (command === "serve") {
    return serve();
  } else {
    return build();
  }
});

function serve(): UserConfig {
  const opts = TOML.parse(fs.readFileSync("../env.toml").toString());
  const ports = getPorts(opts)!;

  return {
    plugins: [react()],
    server: {
      proxy: {
        "/api": {
          target: `http://localhost:${ports.server}`,
          changeOrigin: true,
          secure: false,
          ws: true,
          configure: configureProxy,
          rewrite: (path) => path.replace(/^\/api/, ""),
        },
      },
      port: ports.client,
    },
    css: { modules: { localsConvention: "camelCase" } },
  };
}

function configureProxy(proxy: HttpProxy.Server) {
  proxy.on("error", (err) => {
    console.log("proxy error", err);
  });
  proxy.on("proxyReq", (_proxyReq, req) => {
    console.log("Sending Request to the Target:", req.method, req.url);
  });
  proxy.on("proxyRes", (proxyRes, req) => {
    console.log(
      "Received Response from the Target:",
      proxyRes.statusCode,
      req.url,
    );
  });
}

function getPorts(opts: Record<string, TomlPrimitive>) {
  if (!isTomlRecord(opts.dev)) {
    return undefined;
  }
  if (!isTomlRecord(opts.dev.ports)) {
    return undefined;
  }
  if (typeof opts.dev.ports.server !== "number") {
    return undefined;
  }
  if (typeof opts.dev.ports.client !== "number") {
    return undefined;
  }
  return {
    server: opts.dev.ports.server,
    client: opts.dev.ports.client,
  };
}

function isTomlRecord(prim: TomlPrimitive): prim is {
  [key: string]: TomlPrimitive;
} {
  if (typeof prim !== "object") {
    return false;
  }
  if (prim instanceof TOML.TomlDate) {
    return false;
  }
  if (Array.isArray(prim)) {
    return false;
  }
  return true;
}

function build(): UserConfig {
  return {
    plugins: [react()],
    esbuild: {
      drop: ["console"],
    },
  };
}

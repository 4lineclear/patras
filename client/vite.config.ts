import { HttpProxy, UserConfig, defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig(({ command }) => {
  if (command === "serve") {
    return serve();
  } else {
    return build();
  }
});

function build(): UserConfig {
  return {
    plugins: [react()],
    esbuild: {
      drop: ["console"],
    },
    css: {
      modules: {
        localsConvention: "camelCase",
        scopeBehaviour: "local",
      },
    },
  };
}

function serve(): UserConfig {
  const env = loadEnv("all", "../");
  const ports = getPorts(env)!;

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
        },
      },
      port: ports.client,
    },
    css: {
      modules: {
        localsConvention: "camelCase",
        scopeBehaviour: "local",
      },
      preprocessorOptions: {
        scss: {
          modules: true,
        },
      },
    },
    build: {
      sourcemap: true,
    },
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

function getPorts(opts: Record<string, string>) {
  const server = parseInt(opts.VITE_DEV_SERVER_PORT);
  const client = parseInt(opts.VITE_DEV_CLIENT_PORT);
  return {
    server,
    client,
  };
}

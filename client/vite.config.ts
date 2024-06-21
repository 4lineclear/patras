import fs from "fs";
import { UserConfig, defineConfig } from "vite";
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
  function getPorts(opts: Record<string, TomlPrimitive>) {
    const local = opts.local;
    if (!check(local)) {
      return undefined;
    }
    const ports = local.ports;
    if (!check(ports)) {
      return undefined;
    }
    if (typeof ports.server !== "number") {
      return undefined;
    }
    if (typeof ports.client !== "number") {
      return undefined;
    }
    return {
      server: ports.server,
      client: ports.client,
    };
  }

  function check(prim: TomlPrimitive): prim is {
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
  const opts = TOML.parse(fs.readFileSync("../opts.toml").toString());
  const ports = getPorts(opts)!;

  return {
    plugins: [react()],
    server: {
      proxy: {
        "/api": `http://localhost:${ports.server}`,
      },
      port: ports.client,
    },
  };
}

function build(): UserConfig {
  return {
    plugins: [react()],
  };
}

import fs from "fs";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import TOML from "smol-toml";

// https://vitejs.dev/config/
export default defineConfig(() => {
  const opts = TOML.parse(fs.readFileSync("../opts.toml").toString());
  const port = opts["local.ports.client"] as number;
  return {
    plugins: [react()],
    server: {
      port,
    },
  };
});

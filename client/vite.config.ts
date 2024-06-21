import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig(() => {
  const port = parseInt(loadEnv("all", "..").VITE_PORT);
  return {
    plugins: [react()],
    server: {
      port,
    },
  };
});

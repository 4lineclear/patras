import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react";

const port = parseInt(loadEnv("all", "../").FRONTEND_PORT);

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    port,
  },
});

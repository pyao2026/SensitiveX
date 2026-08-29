import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
    plugins: [vue(), tailwindcss()],
    build: {
        rollupOptions: {
            output: {
                manualChunks(id) {
                    const normalizedId = id.replace(/\\/g, "/");

                    const pageMatch = normalizedId.match(
                        /\/src\/pages\/([^/?]+)\.vue(?:\?|$)/,
                    );
                    if (pageMatch) {
                        return `page-${pageMatch[1]}`;
                    }

                    const componentMatch = normalizedId.match(
                        /\/src\/components\/([^/?]+)\.vue(?:\?|$)/,
                    );
                    if (componentMatch) {
                        return `component-${componentMatch[1]}`;
                    }

                    if (normalizedId.includes("/node_modules/")) {
                        return "vendor";
                    }
                },
            },
        },
    },
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                  protocol: "ws",
                  host,
                  port: 1421,
              }
            : undefined,
        watch: {
            ignored: ["**/src-tauri/**"],
        },
    },
}));

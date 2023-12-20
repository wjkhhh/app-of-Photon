import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";;
import ViteWasmPlugin from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(),
    ViteWasmPlugin(), // 添加这一行
    topLevelAwait({
      promiseExportName: '__tla',
      promiseImportName: i => `__tla_${i}`
    })
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  }
}));

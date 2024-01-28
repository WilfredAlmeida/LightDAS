import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"], // Entry point of your library
  format: ["cjs", "esm"], // Output formats
  clean: true, // Clean the output directory before building
  dts: true, // Generate TypeScript declaration files
});
# rand_color static demo

This is a deliberately plain static site for trying the workspace crates in a
browser. There is no frontend framework and no design system: just HTML,
default form controls, a small JavaScript module, and a tiny `wasm-bindgen`
crate that calls the local Rust crates.

## Build

```bash
make demo-build
```

The build script:

- installs the `wasm32-unknown-unknown` Rust target if needed
- installs `wasm-bindgen-cli` into `demo/.tools` if needed
- writes generated WASM bindings into `demo/site/pkg`

## Run

```bash
make demo-serve
```

Then open http://localhost:8787.

## GitHub Pages

The repository is configured to deploy Pages from Actions. The Pages workflow
builds the WASM demo and deploys `demo/site` on pushes to `master`.

## What it demonstrates

- seeded generation across all color spaces
- crate output strings versus browser preview CSS
- RGB <-> HSL conversion and round-trip deltas
- the project caveat that generation is numeric-range sampling, not color
  management or palette design

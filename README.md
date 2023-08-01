# Welcome to the Clumpiverse

##### m1.0.0

A tryout/playground for a universe where everything is a mass-spring-damper lattice.

https://undo3d.gitlab.io/welcome-to-the-clumpiverse/


## License

Standard MIT License. See the ‘LICENSE’ file for details.


## Install build dependencies

1. Install static-server globally:  
   `npm install -g static-server`
2. Install the Rust toolchain and wasm-pack —  
   see tryout-rust-for-the-first-time.html


## Develop and build

To work on this project, run:  
`node develop.js`

The `develop.js` script will:  
1. Check that your current working directory is the root of the project
2. Copy the `LICENSE` and `VERSION` files from project root to ‘public/’
3. Immediately build ‘src/wasm/welcome-to-the-clumpiverse.rs’ to ‘public/lib/wasm/’
4. Start watching ‘src/wasm/’
   — it triggers a build whenever anything changes
5. Start a server on http://localhost:9080/ and open a browser window

There’s no automatic browser refresh when code changes. You’ll need to
manually refresh the browser to load changes.

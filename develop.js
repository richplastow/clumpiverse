/** develop.js
 * 
 * To work on this project, run:  
 * `node develop.js`
 * 
 * The `develop.js` script will:  
 * 1. Check that your current working directory is the root of the project
 * 2. Copy the `LICENSE` and `VERSION` files from project root to ‘public/’
 * 3. Immediately build ‘src/wasm/welcome-to-the-clumpiverse.rs’ to ‘public/lib/wasm/’
 * 4. Start watching ‘src/wasm/’
 *    — it triggers a build whenever anything changes
 * 5. Start a server on http://localhost:9080/ and open a browser window
 * 
 * There’s no automatic browser refresh when code changes. You’ll need to
 * manually refresh browser to load changes.
 */


// Dependencies and variables.

const fs = require('fs')
const child_process = require('child_process')
let jsDebounce, wasmDebounce


// 1. Check that your current working directory is the root of the project
if (__dirname !== process.cwd() || __dirname.slice(-27) !== '/welcome-to-the-clumpiverse')
  throw Error(`develop.js must be run from ‘/welcome-to-the-clumpiverse’, not ‘${__dirname}’`)


// 2. Copy the `LICENSE` and `VERSION` files from project root to ‘public/’
console.log('Copying the LICENSE and VERSION files')
copyLicenseAndVersion()


// 3. Immediately build ‘src/wasm/welcome-to-the-clumpiverse.rs’ to ‘public/lib/wasm/’
console.log('Building src/wasm/welcome-to-the-clumpiverse.rs')
buildWasm()


// 4. Start watching ‘src/wasm/’
//    — trigger a build whenever anything changes
fs.watch('src/wasm/', (eventType, filename) => {
  if (wasmDebounce) return
  wasmDebounce = setTimeout(function() { wasmDebounce = null }, 1000)
  buildWasm()
})


// 5. Start a server on http://localhost:9080/ and open a browser window
console.log('Manually refresh browser to load changes')
startServer()


// Utilities.

function copyLicenseAndVersion () {
  child_process.exec(
    `cp {LICENSE,VERSION} public/`,
    { stdio: ['pipe','pipe','pipe'] },
    function (err, stdout, stderr) {
      if (err) console.error(err)
      if (stdout) console.log(stdout)
      if (stderr) console.error(stderr)
    }
  )
}

function buildWasm () {
  child_process.exec(
    'wasm-pack build --no-typescript --target web --out-dir public/lib/wasm',
    { stdio: ['pipe','pipe','pipe'] },
    function (_err, stdout, stderr) {
      // Note that wasm-pack sends its normal output to stderr.
      const lines = (stdout+'\n'+stderr+'').split('\n')
      const output = lines.filter(line =>
        line.includes('Done in') ||
        line.startsWith('warning: ') ||
        line.startsWith('error[') ||
        line.includes(' --> ') ||
        line.includes('|') ||
        line.includes('  =')
      )
      console.log(output.join('\n'))
      // console.log(lines)

      // Delete files we don’t need.
      try{fs.unlinkSync('public/lib/wasm/.gitignore')}catch(e){}
      try{fs.unlinkSync('public/lib/wasm/package.json')}catch(e){}
      try{fs.unlinkSync('public/lib/wasm/README.md')}catch(e){}
    }
  )
}

function startServer () {
  child_process.exec(
    `static-server public & open http://localhost:9080/`,
    { stdio: ['pipe','pipe','pipe'] },
    function (err, stdout, stderr) {
      if (err) console.error(err)
      if (stdout) console.log(stdout)
      if (stderr) console.error(stderr)
    }
  )
}

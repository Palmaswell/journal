// Use ES module import syntax to import functionality from the module
// that we have compiled.
//
// Note that the `default` import is an initialization function which
// will "boot" the module and make it ready to use. Currently browsers
// don't support natively imported WebAssembly as an ES module, but
// eventually the manual initialization won't be required!
import init, { get_system, get_coordinates } from "./pkg/forces.js";
const PARTICLE_ELEMENTS = 4;

const drawInitialSystem = (ctx, system) => {
  for (let idx = 0; idx < system.length; idx += PARTICLE_ELEMENTS) {
    const x = system[idx];
    const y = system[idx + 1];
    const width = system[idx + 2];
    const height = system[idx + 3];

    ctx.beginPath();
    ctx.arc(x, y, width, 0, 2 * Math.PI);
    ctx.fillStyle = "rgb(247, 37, 133)";
    ctx.fill();
    ctx.stroke();
    ctx.closePath();
  }

  return ctx;
};

function loopHandler(uInt32Arr) {
  const loop = (timestamp) => {
    requestAnimationFrame(loop);
  };

  return loop;
}

(async () => {
  // First up we need to actually load the wasm file, so we use the
  // default export to inform it where the wasm file is located on the
  // server, and then we wait on the returned promise to wait for the
  // wasm to be loaded.
  //
  // It may look like this: `await init('./pkg/without_a_bundler_bg.wasm');`,
  // but there is also a handy default inside `init` function, which uses
  // `import.meta` to locate the wasm file relatively to js file.
  //
  // Note that instead of a string you can also pass in any of the
  // following things:
  //
  // * `WebAssembly.Module`
  //
  // * `ArrayBuffer`
  //
  // * `Response`
  //
  // * `Promise` which returns any of the above, e.g. `fetch("./path/to/wasm")`
  //
  // This gives you complete control over how the module is loaded
  // and compiled.
  //
  // Also note that the promise, when resolved, yields the wasm module's
  // exports which is the same as importing the `*_bg` module in other
  // modes
  try {
    await init();
  } catch (err) {
    console.error(err, "unable to load wasm file");
  }

  const canvas = document.getElementById("canvas");
  let ctx = canvas.getContext("2d");
  const system = get_system();
  ctx = drawInitialSystem(ctx, system);

  const loop = loopHandler(get_coordinates(ctx));
  requestAnimationFrame(loop);
})();

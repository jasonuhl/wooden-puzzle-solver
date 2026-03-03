let wasmModulePromise;

async function getWasmModule() {
  if (!wasmModulePromise) {
    wasmModulePromise = import("./wasm/wooden_puzzle_solver.js").then(async (module) => {
      await module.default();
      return module;
    });
  }

  return wasmModulePromise;
}

self.onmessage = async (event) => {
  const payload = event.data;
  if (!payload || payload.type !== "next") {
    return;
  }

  try {
    const wasmModule = await getWasmModule();
    const output = wasmModule.next_solution();

    if (!output) {
      self.postMessage({ type: "done" });
    } else {
      self.postMessage({ type: "solution", output });
    }
  } catch (error) {
    self.postMessage({
      type: "error",
      message: error && error.message ? error.message : String(error)
    });
  }
};

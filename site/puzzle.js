const COLOR_PALETTE = {
  1: "#ef4444",
  2: "#f97316",
  3: "#f59e0b",
  4: "#84cc16",
  5: "#22c55e",
  6: "#14b8a6",
  7: "#06b6d4",
  8: "#3b82f6",
  9: "#8b5cf6"
};

const DEFAULT_COLOR = "#9ca3af";

function parseAllLayerSets(output) {
  const lines = output.split(/\r?\n/).map((line) => line.trim());
  const allLayerSets = [];

  for (let index = 0; index < lines.length; index += 1) {
    if (lines[index] !== "We win!") {
      continue;
    }

    // Collect the 3 data lines (one per y=0,1,2), each with 9 values
    const dataLines = [];
    for (const line of lines.slice(index + 1)) {
      if (line.length === 0) continue;
      const values = line.split(/\s+/).map((token) => Number.parseInt(token, 10));
      if (values.length !== 9 || values.some((value) => Number.isNaN(value))) break;
      dataLines.push(values);
      if (dataLines.length === 3) break;
    }

    if (dataLines.length !== 3) continue;

    // dataLines[y][x*3+z] = universe[x][y][z]
    // Build layers[layerIdx][rowIdx][col] where layerIdx→z=2,1,0 and rowIdx→y=2,1,0
    const layers = [];
    for (let z = 2; z >= 0; z--) {
      const layerRows = [];
      for (let y = 2; y >= 0; y--) {
        layerRows.push([dataLines[y][z], dataLines[y][3 + z], dataLines[y][6 + z]]);
      }
      layers.push(layerRows);
    }

    allLayerSets.push(layers);
  }

  if (allLayerSets.length === 0) {
    return null;
  }

  return allLayerSets;
}

function cubesFromLayers(layers) {
  const cubes = [];

  layers.forEach((rows, layerIndex) => {
    rows.forEach((row, rowIndex) => {
      row.forEach((value, columnIndex) => {
        cubes.push({
          x: columnIndex,
          y: 2 - layerIndex,
          z: rowIndex,
          value,
          color: COLOR_PALETTE[value] || DEFAULT_COLOR
        });
      });
    });
  });

  return cubes;
}

function renderLegend(legendElement, cubes) {
  const values = [...new Set(cubes.map((cube) => cube.value))].sort((a, b) => a - b);
  legendElement.innerHTML = "";

  values.forEach((value) => {
    const item = document.createElement("div");
    item.className = "legend-item";

    const swatch = document.createElement("span");
    swatch.className = "legend-swatch";
    swatch.style.backgroundColor = COLOR_PALETTE[value] || DEFAULT_COLOR;

    const label = document.createElement("span");
    label.textContent = String(value);

    item.appendChild(swatch);
    item.appendChild(label);
    legendElement.appendChild(item);
  });
}

function wireRotation(scene, grid, rotateXInput, rotateYInput) {
  const clamp = (value, min, max) => Math.max(min, Math.min(max, value));

  const applyRotation = () => {
    grid.style.transform = `rotateX(${rotateXInput.value}deg) rotateY(${rotateYInput.value}deg)`;
  };

  let dragging = false;
  let lastX = 0;
  let lastY = 0;

  const onPointerMove = (event) => {
    if (!dragging) {
      return;
    }

    const deltaX = event.clientX - lastX;
    const deltaY = event.clientY - lastY;
    lastX = event.clientX;
    lastY = event.clientY;

    const nextRotateY = clamp(Number(rotateYInput.value) + deltaX * 0.7, -180, 180);
    const nextRotateX = clamp(Number(rotateXInput.value) - deltaY * 0.7, -90, 90);

    rotateYInput.value = String(nextRotateY);
    rotateXInput.value = String(nextRotateX);
    applyRotation();
  };

  const stopDragging = () => {
    if (!dragging) {
      return;
    }

    dragging = false;
    scene.classList.remove("dragging");
    window.removeEventListener("pointermove", onPointerMove);
    window.removeEventListener("pointerup", stopDragging);
    window.removeEventListener("pointercancel", stopDragging);
  };

  scene.addEventListener("pointerdown", (event) => {
    if (event.button !== 0) {
      return;
    }

    dragging = true;
    lastX = event.clientX;
    lastY = event.clientY;
    scene.classList.add("dragging");
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", stopDragging);
    window.addEventListener("pointercancel", stopDragging);
    event.preventDefault();
  });

  rotateXInput.addEventListener("input", applyRotation);
  rotateYInput.addEventListener("input", applyRotation);
  applyRotation();
}

function renderCubes(grid, cubes) {
  const cubeSize = 42;
  const spacing = 8;
  const unit = cubeSize + spacing;
  const offset = unit;

  grid.innerHTML = "";

  cubes.forEach((cube) => {
    const element = document.createElement("div");
    element.className = "cube";
    element.style.width = `${cubeSize}px`;
    element.style.height = `${cubeSize}px`;
    element.style.backgroundColor = cube.color;
    element.style.transform = `translate3d(${cube.x * unit - offset}px, ${(2 - cube.y) * unit - offset}px, ${cube.z * unit - offset}px)`;
    element.title = `value ${cube.value}`;
    grid.appendChild(element);
  });
}

async function boot() {
  const status = document.getElementById("solver-status");
  const nextSolutionButton = document.getElementById("next-solution");
  const autoplaySpeedInput = document.getElementById("autoplay-speed");
  const autoplaySpeedValue = document.getElementById("autoplay-speed-value");
  const scene = document.getElementById("puzzle-scene");
  const grid = document.getElementById("puzzle-grid");
  const legend = document.getElementById("puzzle-legend");
  const rotateXInput = document.getElementById("rotate-x");
  const rotateYInput = document.getElementById("rotate-y");

  if (!status || !nextSolutionButton || !autoplaySpeedInput || !autoplaySpeedValue || !scene || !grid || !legend || !rotateXInput || !rotateYInput) {
    return;
  }

  wireRotation(scene, grid, rotateXInput, rotateYInput);
  status.textContent = "Computing first solution in your browser…";

  try {
    const worker = new Worker("./puzzle_worker.js", { type: "module" });
    let currentLayers = null;
    let solutionNumber = 0;
    let computing = false;
    let done = false;
    let autoplayTimer = null;

    const autoplaySpeed = () => Number(autoplaySpeedInput.value);

    const autoplayDelayMs = () => {
      const speed = autoplaySpeed();
      if (speed <= 0) {
        return null;
      }

      const minDelay = 120;
      const maxDelay = 2000;
      const ratio = speed / 100;
      return Math.round(maxDelay - ratio * (maxDelay - minDelay));
    };

    const clearAutoplay = () => {
      if (autoplayTimer) {
        window.clearTimeout(autoplayTimer);
        autoplayTimer = null;
      }
    };

    const updateAutoplayLabel = () => {
      autoplaySpeedValue.textContent = String(autoplaySpeed());
    };

    const autoplayTick = () => {
      if (done) {
        clearAutoplay();
        return;
      }

      requestNextSolution();
      const delay = autoplayDelayMs();
      if (delay === null) {
        clearAutoplay();
        return;
      }
      autoplayTimer = window.setTimeout(autoplayTick, delay);
    };

    const startAutoplay = () => {
      clearAutoplay();
      const delay = autoplayDelayMs();
      if (delay === null) {
        return;
      }
      autoplayTimer = window.setTimeout(autoplayTick, delay);
    };

    const renderSolution = () => {
      if (!currentLayers) {
        return;
      }

      const cubes = cubesFromLayers(currentLayers);
      renderCubes(grid, cubes);
      renderLegend(legend, cubes);
      status.textContent = `Solution ${solutionNumber}`;
    };

    const setButtonState = () => {
      nextSolutionButton.disabled = computing || done;
    };

    const requestNextSolution = () => {
      if (computing || done) {
        return;
      }

      computing = true;
      setButtonState();
      worker.postMessage({ type: "next" });
    };

    nextSolutionButton.disabled = false;
    nextSolutionButton.addEventListener("click", requestNextSolution);
    autoplaySpeedInput.addEventListener("input", () => {
      updateAutoplayLabel();
      startAutoplay();
    });

    updateAutoplayLabel();

    requestNextSolution();
    startAutoplay();

    worker.onmessage = (event) => {
      const { data } = event;
      if (!data || !data.type) {
        return;
      }

      if (data.type === "solution") {
        const parsed = parseAllLayerSets(data.output || "");
        computing = false;

        if (!parsed || parsed.length === 0) {
          done = true;
          setButtonState();
          clearAutoplay();
          return;
        }

        currentLayers = parsed[0];
        solutionNumber += 1;
        renderSolution();
        setButtonState();
        return;
      }

      if (data.type === "done") {
        computing = false;
        done = true;

        if (solutionNumber === 0) {
          status.textContent = "No solutions found.";
        } else {
          renderSolution();
        }

        setButtonState();
        clearAutoplay();
        return;
      }

      if (data.type === "error") {
        computing = false;
        clearAutoplay();
        setButtonState();
        status.textContent = "Failed to run WebAssembly solver.";
        console.error(data.message);
      }
    };
  } catch (error) {
    status.textContent = "Failed to load WebAssembly solver.";
    console.error(error);
  }
}

boot();

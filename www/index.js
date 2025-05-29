import init, { Maze, Cell, MazeBuilder, CellType } from "../pkg/mazeweb.js";


// === Constants ===
const wasmInit = await init();
const memory = wasmInit.memory;
// Constants for cell types and colors
const TYPE_MASK = 0b11110000;
const OUTLINE_COLOR = "#b86134";
const DEFAULT_COLOR = "#dad7da";
const WALL_COLOR = "#5c2f18";
const START_COLOR = "#00609b"
const VISITED_COLOR = "#1d8dcc"
const GOAL_COLOR = "#847244";
const LOOKING_AT = "#c49358";
const CHANGEING = "#7c2a0b";
const CURRENT_COLOR = "#5A827E";
const BACKGROUND_COLOR = "#B9D4AA";
// Constants for wall bits
const WALL_N = 0b0001;
const WALL_E = 0b0010;
const WALL_S = 0b0100;
const WALL_W = 0b1000;
const WALL_MASK = 0b00001111;

// === Globals ===
let maze = Maze.new(50, 50); // default maze size
let builder = null;
let width = maze.width();
let height = maze.height();
let animationIntervalId = null;
let animationSpeed = 100;

// === DOM Elements ===
const canvas = document.getElementById("mazeweb-canvas");
const ctx = canvas.getContext("2d");
const widthInput = document.getElementById("width");
const heightInput = document.getElementById("height");
const generatorSelect = document.getElementById("generator");
const generateButton = document.getElementById("generate");
const playPauseButton = document.getElementById("play-pause");
const stepForwardButton = document.getElementById("step-forward");
const stepBackButton = document.getElementById("step-back");
const speedInput = document.getElementById("speed");
const stepSlider = document.getElementById("step-slider");
const stepLabel = document.getElementById("step-label");

// === Utility Functions ===
const getIndex = (row, column) => row * width + column;

const resizeCanvas = () => {
  const cell_size = getCellSize();
  canvas.height = (cell_size + 1) * height + 1;
  canvas.width = (cell_size + 1) * width + 1;
};

const drawMaze = () => {
  ctx.fillStyle = BACKGROUND_COLOR;
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  drawOutline();
  drawCells();
}

const drawOutline = () => {
  ctx.beginPath();
  ctx.strokeStyle = OUTLINE_COLOR;
  ctx.lineWidth = 2;

  const cell_size = getCellSize();
  const outlineWidth = (cell_size + 1) * width + 1;
  const outlineHeight = (cell_size + 1) * height + 1;

  // Draw a rectangle around the grid area
  ctx.rect(0, 0, outlineWidth, outlineHeight);
  ctx.stroke();
};

const drawWalls = (cellValue, row, col, size) => {
  const x = col * (size + 1) + 1;
  const y = row * (size + 1) + 1;

  ctx.strokeStyle = WALL_COLOR;
  ctx.lineWidth = 4;

  ctx.beginPath();

  if (cellValue & WALL_N) {
    ctx.moveTo(x, y);
    ctx.lineTo(x + size, y);
  }
  if (cellValue & WALL_E) {
    ctx.moveTo(x + size, y);
    ctx.lineTo(x + size, y + size);
  }
  if (cellValue & WALL_S) {
    ctx.moveTo(x, y + size);
    ctx.lineTo(x + size, y + size);
  }
  if (cellValue & WALL_W) {
    ctx.moveTo(x, y);
    ctx.lineTo(x, y + size);
  }

  ctx.stroke();
};

const cellColor = (cellValue) => {
  const type = cellValue & TYPE_MASK;

  switch (type) {
    case CellType.Default: return DEFAULT_COLOR;
    case CellType.Start: return START_COLOR;
    case CellType.End: return GOAL_COLOR;
    case CellType.Path: return CHANGEING;
    case CellType.Visited: return VISITED_COLOR;
    case CellType.LookingAt: return LOOKING_AT;
    case CellType.Current: return CURRENT_COLOR;
    case CellType.Changing: return CHANGEING;
    default: return "#111";
  }
};

const getCellSize = () => {
  const maxWidth = canvas.parentElement.clientWidth || window.innerWidth;
  const maxHeight = canvas.parentElement.clientHeight || window.innerHeight;
  return Math.floor(Math.min(maxWidth / width, maxHeight / height) - 1);
};

const drawCells = () => {
  const cellsPtr = maze.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  const cell_size = getCellSize();

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const index = getIndex(row, col);
      const cellValue = cells[index];

      ctx.fillStyle = cellColor(cellValue);
      ctx.fillRect(
        col * (cell_size + 1) + 1,
        row * (cell_size + 1) + 1,
        cell_size,
        cell_size
      );

      drawWalls(cellValue, row, col, cell_size); // << render walls
    }
  }

  ctx.stroke();
};

const safeUpdateMaze = () => {
  if (animationIntervalId !== null) return;

  updateMaze();
  builder = null; // clear builder to reset solution
  stepSlider.value = 0;
  stepSlider.max = 0;
  updateStepLabel();
};

const updateMaze = () => {
  const newWidth = parseInt(widthInput.value, 10) || 50;
  const newHeight = parseInt(heightInput.value, 10) || 50;

  maze = Maze.new(newWidth, newHeight);
  width = newWidth;
  height = newHeight;

  resizeCanvas();
  drawMaze();
};

function updateStepLabel() {
  if (!builder) {
    stepLabel.textContent = "Step: 0 / 0";
    return;
  };
  stepLabel.textContent = `Step: ${builder.current_step()} / ${builder.total_steps() - 1}`;
}

// === Event Handlers ===
canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;
  const cell_size = getCellSize();

  const row = Math.min(Math.floor(canvasTop / (cell_size + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (cell_size + 1)), width - 1);

  maze.toggle_cell(row, col);
  drawMaze();
});

widthInput.addEventListener("input", safeUpdateMaze);
heightInput.addEventListener("input", safeUpdateMaze);

speedInput.addEventListener("input", () => {
  animationSpeed = parseInt(speedInput.value, 10) || 100;
});

generateButton.addEventListener("click", () => {
  updateMaze(); // ensure maze is fresh

  // Set up the builder with the selected generator
  const generator = generatorSelect.value;
  builder = MazeBuilder.withGenerator(generator);

  // Generate the maze
  builder.generate_all(maze);
  const total = builder.total_steps();

  stepSlider.max = total > 0 ? total - 1 : 0;
  stepSlider.value = builder.current_step();
  updateStepLabel();
  startAnimation();
  drawMaze();
});

stepSlider.addEventListener("input", (e) => {
  if (!builder) return;

  stopAnimation(); // pause if playing
  goToStep(parseInt(e.target.value, 10));
});

playPauseButton.addEventListener("click", () => {
  if (animationIntervalId === null) {
    playPauseButton.textContent = "⏸";
    startAnimation();
  } else {
    playPauseButton.textContent = "▶";
    stopAnimation();
  }
});

stepForwardButton.addEventListener("click", () => {
  if (builder && builder.step_forward(maze)) {
    stepSlider.value = builder.current_step();
    updateStepLabel();
    drawMaze();
  }
});

stepBackButton.addEventListener("click", () => {
  if (builder && builder.step_backward(maze)) {
    stepSlider.value = builder.current_step();
    updateStepLabel();
    drawMaze();
  }
});

// === Animation Control ===
function disableSizeInputs(disabled) {
  widthInput.disabled = disabled;
  heightInput.disabled = disabled;
}

function updatePlayPauseButton() {
  playPauseButton.textContent = animationIntervalId === null ? "▶" : "⏸";
}

function startAnimation() {
  if (!builder || animationIntervalId) return;
  disableSizeInputs(true);

  animationIntervalId = setInterval(() => {
    const moreSteps = builder.step_forward(maze);
    stepSlider.value = builder.current_step();
    updateStepLabel();
    drawMaze();
    if (!moreSteps) stopAnimation();
  }, animationSpeed);

  updatePlayPauseButton();
}

function stopAnimation() {
  if (!animationIntervalId) return;
  disableSizeInputs(false);
  clearInterval(animationIntervalId);
  animationIntervalId = null;
  updatePlayPauseButton();
}

function goToStep(stepIndex) {
  if (!builder) return;

  // Reset maze to empty state and replay to desired step
  maze = Maze.new(width, height);
  builder.step_to(stepIndex, maze);
  updateStepLabel();
  drawMaze();
}



// === Startup ===
stepSlider.value = 0;
stepSlider.max = 0;
updateMaze();
resizeCanvas();
drawMaze();

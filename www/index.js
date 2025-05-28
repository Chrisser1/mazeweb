import init, { Maze, Cell, MazeBuilder } from "../pkg/mazeweb.js";


// === Constants ===
const wasmInit = await init();
const memory = wasmInit.memory;
const OUTLINE_COLOR = "#b86134";
const EMPTY_COLOR = "#dad7da";
const WALL_COLOR = "#5c2f18";
const START_COLOR = "#00609b"
const VISITED_COLOR = "#1d8dcc"
const GOAL_COLOR = "#847244";
const LOOKING_AT = "#c49358";
const CHANGEING = "#7c2a0b";

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

const cellColor = (cellValue) => {
  switch (cellValue) {
    case Cell.Empty: return EMPTY_COLOR;
    case Cell.Wall: return WALL_COLOR;
    case Cell.Start: return START_COLOR;
    case Cell.End: return GOAL_COLOR;
    case Cell.Path: return CHANGING;
    case Cell.Visited: return VISITED_COLOR;
    case Cell.LookingAt: return LOOKING_AT;
    case Cell.Changeing: return CHANGEING;
    default: return "#000000"; // fallback for unknown values
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
      ctx.fillStyle = cellColor(cells[index]);

      ctx.fillRect(
        col * (cell_size + 1) + 1,
        row * (cell_size + 1) + 1,
        cell_size,
        cell_size
      );
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
  drawOutline();
  drawCells();
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
  drawOutline();
  drawCells();
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
  drawOutline();
  drawCells();
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
    drawOutline();
    drawCells();
  }
});

stepBackButton.addEventListener("click", () => {
  if (builder && builder.step_backward(maze)) {
    stepSlider.value = builder.current_step();
    updateStepLabel();
    drawOutline();
    drawCells();
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
    drawOutline();
    drawCells();
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
  drawOutline();
  drawCells();
}



// === Startup ===
stepSlider.value = 0;
stepSlider.max = 0;
updateMaze();
resizeCanvas();
drawOutline();
drawCells();

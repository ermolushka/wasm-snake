import { Universe, Cell, Direction } from "wasm-snake";
import { memory } from "wasm-snake/wasm_snake_bg";


const CELL_SIZE = 8; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("snake-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

let animationId = null;


let currentDirection = Direction.Right; // Default direction

document.addEventListener('keydown', event => {
  if(['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(event.key)) {
    event.preventDefault();
  }

  const prevDirection = currentDirection;
  
  switch (event.key) {
    case 'ArrowUp':
      if (currentDirection !== Direction.Down && prevDirection !== Direction.Down) {
        currentDirection = Direction.Up;
        universe.change_snake_direction(Direction.Up);
      }
      break;
    case 'ArrowRight':
      if (currentDirection !== Direction.Left && prevDirection !== Direction.Left) {
        currentDirection = Direction.Right;
        universe.change_snake_direction(Direction.Right);
      }
      break;
    case 'ArrowDown':
      if (currentDirection !== Direction.Up && prevDirection !== Direction.Up) {
        currentDirection = Direction.Down;
        universe.change_snake_direction(Direction.Down);
      }
      break;
    case 'ArrowLeft':
      if (currentDirection !== Direction.Right && prevDirection !== Direction.Right) {
        currentDirection = Direction.Left;
        universe.change_snake_direction(Direction.Left);
      }
      break;
  }
});

const renderLoop = () => {
  drawGrid();
  drawCells();

  universe.tick();
  if (universe.snake_is_alive()) {
    animationId = setTimeout(renderLoop, 500);
  } else {
    gameStatus.textContent = "game over"
    pause();
  }
};

const gameStatus = document.getElementById("game-status");

const play = () => {
  gameStatus.textContent = "playing"
  renderLoop();
};

const pause = () => {
  cancelAnimationFrame(animationId);
  animationId = null;
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
  
    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
  
    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
  
    ctx.stroke();
  };

  const getIndex = (row, column) => {
    return row * width + column;
  };
  
  const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  
    ctx.beginPath();
  
    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);
  
        ctx.fillStyle = cells[idx] === Cell.Empty
          ? DEAD_COLOR
          : ALIVE_COLOR;
  
        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }
  
    ctx.stroke();
  };

drawGrid();
drawCells();
play();
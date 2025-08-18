import init, { Universe } from "./pkg/rgol_wasm.js";

let memory;

const CELL_SIZE = 8;  // px
const GAP = 1;        // px
let universe, ctx, width, height;

async function run() {
  const wasm = await init();
  memory = wasm.memory;

  const canvas = document.getElementById("life");
  ctx = canvas.getContext("2d");
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;

  width = Math.floor(canvas.width / (CELL_SIZE + GAP));
  height = Math.floor(canvas.height / (CELL_SIZE + GAP));

  universe = new Universe(width, height, 0.9); // 90% live cells

  requestAnimationFrame(renderLoop);
}

function renderLoop() {
  universe.tick();

  const ptr = universe.cells_ptr();
  const cells = new Uint8Array(memory.buffer, ptr, width * height);

  // reseed if almost dead
  const liveCount = cells.reduce((a, b) => a + b, 0);
  if (liveCount < (width * height) * 0.05) {
    universe.randomize(0.3);
  }

  drawCells(cells);
  requestAnimationFrame(renderLoop);
}

function drawCells(cells) {
  ctx.fillStyle = "#000";
  ctx.fillRect(0, 0, width * (CELL_SIZE + GAP), height * (CELL_SIZE + GAP));

  ctx.fillStyle = "#fff";
  let i = 0;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++, i++) {
      if (cells[i] === 1) {
        ctx.fillRect(
          col * (CELL_SIZE + GAP),
          row * (CELL_SIZE + GAP),
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }
  }
}

run();
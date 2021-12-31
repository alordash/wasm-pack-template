import { Universe, set_panic_hook, FillOptions } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

set_panic_hook();

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const WIDTH = 500;
const HEIGHT = 64;

const universe = Universe.new(WIDTH, HEIGHT, FillOptions.i2i7_Pattern);
const width = universe.width();
const height = universe.height();
const size = width * height / 8;
const cellsPtr = universe.cells();
const cells = new Uint8Array(memory.buffer, cellsPtr, size);
console.log(`size = ${size}`);

const canvas = document.getElementById("game-of-life-canvas");
canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height + 1;

const ctx = canvas.getContext('2d');

let skipped_frames = 1;
let anim_speed_range = document.getElementById("anim-speed");
anim_speed_range.addEventListener("change", () => {
    skipped_frames = +anim_speed_range.max - +anim_speed_range.value + 1;
});

document.getElementById("tick-button").addEventListener("click", () => {
    tick();
});

document.getElementById("clear-button").addEventListener("click", () => {
    universe.fill(FillOptions.AllDead);
    draw();
});

document.getElementById("random-button").addEventListener("click", () => {
    universe.fill(FillOptions.Random);
    draw();
});

function tick() {
    universe.tick();
    draw();
}

function draw() {
    fps.render();

    drawGrid();
    drawCells();
}

let animationId = null;

function isPaused() {
    return animationId === null;
}

let frame_num = 0;
function renderLoop() {
    //    console.log(`frame_num: ${frame_num}, skipped_frames: ${skipped_frames}, %: ${frame_num % skipped_frames}`);
    if (frame_num % skipped_frames == 0) {
        tick();
    }
    frame_num = (frame_num + 1) % skipped_frames;
    animationId = requestAnimationFrame(renderLoop);
}
const playPauseButton = document.getElementById("play-pause");

function play() {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

function pause() {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

function drawGrid() {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

function bitIsSet(n, arr) {
    const index = Math.floor(n / 8);
    const offset = n % 8;
    const mask = 1 << offset;
    return (arr[index] & mask) === mask;
}

function drawCells() {
    ctx.beginPath();

    ctx.fillStyle = ALIVE_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = universe.get_index(row, col);
            let cell = bitIsSet(idx, cells);
            if (!cell) continue;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.fillStyle = DEAD_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = universe.get_index(row, col);
            let cell = bitIsSet(idx, cells);
            if (cell) continue;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

const glider = [[-1, 0], [0, 1], [1, -1], [1, 0], [1, 1]];
const pulsar = [[-6, 4], [-6, 3], [-6, 2], [-6, -2], [-6, -3], [-6, -4], [-4, 6], [-3, 6], [-2, 6], [-4, 1], [-3, 1], [-2, 1], [-4, -1], [-3, -1], [-2, -1], [-4, -6], [-3, -6], [-2, -6], [-1, 4], [-1, 3], [-1, 2], [-1, -4], [-1, -3], [-1, -2], [1, 4], [1, 3], [1, 2], [1, -4], [1, -3], [1, -2], [4, 6], [3, 6], [2, 6], [4, 1], [3, 1], [2, 1], [4, -1], [3, -1], [2, -1], [4, -6], [3, -6], [2, -6], [6, 4], [6, 3], [6, 2], [6, -4], [6, -3], [6, -2]];

function fixOffset(n, bound) {
    let offset = Math.max(0, Math.ceil(-n / bound) * bound);
    console.log(`n: ${n}, offset: ${offset}`);
    n += offset;
    return n;
}

function drawFigure(start, points) {
    let xs = points.map((p) => start[0] + p[0]);
    xs = new Uint32Array(xs.map((x) => fixOffset(x, height)));
    let ys = points.map((p) => start[1] + p[1]);
    ys = new Uint32Array(ys.map((y) => fixOffset(y, width)));
    console.log(`xs: ${xs}
ys: ${ys}`);
    universe.set_cells(true, xs, ys);
}

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    if (event.ctrlKey && event.shiftKey) {
        drawFigure([row, col], pulsar);
    } else if (event.ctrlKey) {
        drawFigure([row, col], glider);
    } else {
        universe.toggle_cell(row, col);
    }

    drawGrid();
    drawCells();
});

class FPS {
    constructor() {
        this.fps = document.getElementById("fps");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {
        // Convert the delta time since the last frame render into a measure
        // of frames per second.
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp;
        this.lastFrameTimeStamp = now;
        const fps = 1 / delta * 1000;

        // Save only the latest 100 timings.
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        // Find the max, min, and mean of our 100 latest timings.
        let min = Infinity;
        let max = -Infinity;
        let sum = 0;
        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(this.frames[i], min);
            max = Math.max(this.frames[i], max);
        }
        let mean = sum / this.frames.length;

        // Render the statistics.
        this.fps.textContent = `
  Frames per Second:
           latest = ${Math.round(fps)}
  avg of last 100 = ${Math.round(mean)}
  min of last 100 = ${Math.round(min)}
  max of last 100 = ${Math.round(max)}
  `.trim();
    }
};

const fps = new FPS();

play();
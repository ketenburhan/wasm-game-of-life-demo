import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 8;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const uni = Universe.new();
const width = uni.width();
const height = uni.height();

const canvas = document.querySelector("#game-of-life-canvas");
canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height + 1;

const c = canvas.getContext("2d");


const loop = () => {
    uni.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(loop);
};

function drawGrid() {
    c.beginPath();
    c.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
	const x = i * (CELL_SIZE + 1) + 1;
	c.moveTo(x, 0);
	c.lineTo(x, canvas.height);
    }
    
    for (let i = 0; i <= height; i++) {
	const y = i * (CELL_SIZE + 1) + 1;
	c.moveTo(0, y);
	c.lineTo(canvas.width, y);
    }
    c.stroke();
}

const getIndex = (row, col) => row * width + col;

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
}

function drawCells() {
    const cellsPtr = uni.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width*height / 8);

    c.beginPath();

    for (let row = 0; row < height; row++) {
	for (let col = 0; col < width; col++) {
	    const index = getIndex(row, col);

	    c.fillStyle = bitIsSet(index, cells)
		? ALIVE_COLOR
		: DEAD_COLOR;

	    c.fillRect(
		(col * (CELL_SIZE + 1) + 1),
		(row * (CELL_SIZE + 1) + 1),
		CELL_SIZE,
		CELL_SIZE,
	    );
	}	
    }

    c.stroke();
}

// start loop
requestAnimationFrame(loop);

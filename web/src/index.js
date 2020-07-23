import { Engine } from 'bandori-puyopuyo';
import { memory } from 'bandori-puyopuyo/bandori_puyopuyo_bg';
import * as sprites from './sprites';

const width = 9;
const height = 20;

const engine = Engine.new(width, height);

const spriteDataPtr = engine.get_sprite_data();
const directionDataPtr = engine.get_direction_data();
const piecePartDataPtr = engine.get_piece_part_data();
const spriteData = new Uint8Array(memory.buffer, spriteDataPtr, width * height);
const directionData = new Uint8Array(memory.buffer, directionDataPtr, width * height);
const piecePartData = new Uint8Array(memory.buffer, piecePartDataPtr, width * height);

const canvas = document.getElementById('game-canvas');
const ctx = canvas.getContext('2d');
ctx.canvas.width = 32 * width;
ctx.canvas.height = 32 * height;
const scoreboard = document.getElementById('scoreboard');
const resetBut = document.getElementById('resetBut');

window.onkeydown = e => {

    if (!engine.get_is_game_over() && !engine.get_is_clearing()) {
        switch (e.keyCode) {
            case 39:
                engine.move_piece_right();
                break;
            case 40:
                engine.move_piece_down();
                break;
            case 37:
                engine.move_piece_left();
                break;
            case 90:
                engine.rotate_piece();
        }
    }
}

resetBut.onclick = () => {
    engine.reset();
}

/**
 * Draws grid
 */
function drawGrid() {
    ctx.beginPath();
    ctx.strokeStyle = 'black';

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * 32, 0);
        ctx.lineTo(i * 32, 32 * height);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * 32);
        ctx.lineTo(32 * width, j * 32);
    }

    ctx.stroke();
}

/**
 * Draws pieces
 */
function drawPieces() {

    for (let row = 0; row < engine.get_height(); row++) {
        for (let col = 0; col < engine.get_width(); col++) {
            const idx = engine.get_index(row, col);
            const memberNum = spriteData[idx];
            if (memberNum < 25) {
                const directionNum = directionData[idx];
                const piecePart = piecePartData[idx];
                sprites.drawPiece(ctx, memberNum, piecePart === 0, 32 * col, 32 * row, directionNum);
            }
        }
    }
}

let startTime = new Date().getTime();
const UPDATE_DELTA = 300;

/**
 * Tick to update game
 */
function tick() {

    scoreboard.innerHTML = `Score: ${engine.get_score()}`;

    // Render to canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawGrid();
    drawPieces();

    if (engine.get_is_game_over()) {
        ctx.fillStyle = 'rgba(0, 0, 0, 0.5)';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = 'white';
        ctx.font = '30px Arial';
        ctx.textAlign = 'center';
        ctx.fillText("GAME OVER", canvas.width / 2, canvas.height / 2);
    }

    const currTime = new Date().getTime();
    const delta = currTime - startTime;
    if (delta >= UPDATE_DELTA) {
        engine.tick();
        startTime = currTime;
    }

    requestAnimationFrame(tick);
}

requestAnimationFrame(tick);
import { Engine, Affiliation } from 'bandori-puyopuyo';
import { memory } from 'bandori-puyopuyo/bandori_puyopuyo_bg';

const board = document.getElementById('board');
const engine = Engine.new(9, 20);

window.onkeydown = e => {
    switch (e.keyCode) {
        case 38:
            break
        case 39:
            engine.move_piece_right();
            break;
        case 40:
            break;
        case 37:
            engine.move_piece_left();
            break;
    }
}

let startTime = new Date().getTime();
const UPDATE_DELTA = 1000;

function tick() {

    board.innerHTML = engine.render();

    const currTime = new Date().getTime();
    const delta = currTime - startTime;
    if (delta >= UPDATE_DELTA) {
        engine.tick();
        startTime = currTime;
    }

    requestAnimationFrame(tick);
}

requestAnimationFrame(tick);
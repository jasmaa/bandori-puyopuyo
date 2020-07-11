import { Engine, Affiliation } from 'bandori-puyopuyo';
import { memory } from 'bandori-puyopuyo/bandori_puyopuyo_bg';

const board = document.getElementById('board');
const engine = Engine.new(9, 20);

window.onkeydown = e => {

    if (!engine.get_is_clearing()) {
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

    //console.log(e);
}

let startTime = new Date().getTime();
const UPDATE_DELTA = 500;

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
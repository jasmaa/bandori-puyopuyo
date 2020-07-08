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

function tick() {
    board.innerHTML = engine.render();
    requestAnimationFrame(tick);
}

requestAnimationFrame(tick);
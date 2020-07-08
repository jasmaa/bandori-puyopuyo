import { Engine, Affiliation } from 'bandori-puyopuyo';
import { memory } from 'bandori-puyopuyo/bandori_puyopuyo_bg';

console.log(Affiliation);

const engine = Engine.new(10, 20);

console.log(engine.get_width());

//engine.move_piece_right();

console.log(engine.render());
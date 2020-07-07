import { Engine } from 'bandori-puyopuyo';
import { memory } from 'bandori-puyopuyo/bandori_puyopuyo_bg';

const engine = Engine.new(10, 20);

console.log(engine.get_width());
console.log(engine.render());
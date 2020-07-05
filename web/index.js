import { Engine } from 'bandori-puyopuyo';
import { memory } from 'bandori-puyopuyo/bandori_puyopuyo_bg';

const engine = Engine.new(80, 90);

console.log(engine.get_width());
import * as wasm from './bandori_puyopuyo_bg.wasm';

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }
/**
*/
export const PiecePart = Object.freeze({ Head:0,Tail:1, });
/**
*/
export const Affiliation = Object.freeze({ Popipa:0,Afterglow:1,Pasupare:2,Roselia:3,Harohapi:4, });
/**
*/
export const Direction = Object.freeze({ Up:0,Right:1,Down:2,Left:3, });
/**
*/
export const Sprite = Object.freeze({ Kasumi:0,Tae:1,Rimi:2,Saaya:3,Arisa:4,Ran:5,Moca:6,Himari:7,Tomoe:8,Tsugumi:9,Aya:10,Hina:11,Chisato:12,Maya:13,Eve:14,Yukina:15,Sayo:16,Lisa:17,Ako:18,Rinko:19,Kokoro:20,Kaoru:21,Hagumi:22,Kanon:23,Misaki:24, });
/**
*/
export class Engine {

    static __wrap(ptr) {
        const obj = Object.create(Engine.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_engine_free(ptr);
    }
    /**
    * @param {number} width
    * @param {number} height
    * @returns {Engine}
    */
    static new(width, height) {
        var ret = wasm.engine_new(width, height);
        return Engine.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    get_width() {
        var ret = wasm.engine_get_width(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    get_height() {
        var ret = wasm.engine_get_height(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    get_score() {
        var ret = wasm.engine_get_score(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {boolean}
    */
    get_is_clearing() {
        var ret = wasm.engine_get_is_clearing(this.ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    get_is_game_over() {
        var ret = wasm.engine_get_is_game_over(this.ptr);
        return ret !== 0;
    }
    /**
    * @returns {number}
    */
    get_sprite_data() {
        var ret = wasm.engine_get_sprite_data(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    get_direction_data() {
        var ret = wasm.engine_get_direction_data(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    get_piece_part_data() {
        var ret = wasm.engine_get_piece_part_data(this.ptr);
        return ret;
    }
    /**
    * @param {number} row
    * @param {number} col
    * @returns {number}
    */
    get_index(row, col) {
        var ret = wasm.engine_get_index(this.ptr, row, col);
        return ret >>> 0;
    }
    /**
    */
    move_piece_right() {
        wasm.engine_move_piece_right(this.ptr);
    }
    /**
    */
    move_piece_left() {
        wasm.engine_move_piece_left(this.ptr);
    }
    /**
    */
    move_piece_down() {
        wasm.engine_move_piece_down(this.ptr);
    }
    /**
    */
    rotate_piece() {
        wasm.engine_rotate_piece(this.ptr);
    }
    /**
    */
    tick() {
        wasm.engine_tick(this.ptr);
    }
    /**
    */
    reset() {
        wasm.engine_reset(this.ptr);
    }
}

export const __wbg_random_3fe3216a972fe49a = typeof Math.random == 'function' ? Math.random : notDefined('Math.random');

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};


/* tslint:disable */
/* eslint-disable */
/**
*/
export enum PiecePart {
  Head,
  Tail,
}
/**
*/
export enum Affiliation {
  Popipa,
  Afterglow,
  Pasupare,
  Roselia,
  Harohapi,
}
/**
*/
export enum Direction {
  Up,
  Right,
  Down,
  Left,
}
/**
*/
export enum Sprite {
  Kasumi,
  Tae,
  Rimi,
  Saaya,
  Arisa,
  Ran,
  Moca,
  Himari,
  Tomoe,
  Tsugumi,
  Aya,
  Hina,
  Chisato,
  Maya,
  Eve,
  Yukina,
  Sayo,
  Lisa,
  Ako,
  Rinko,
  Kokoro,
  Kaoru,
  Hagumi,
  Kanon,
  Misaki,
}
/**
*/
export class Engine {
  free(): void;
/**
* @param {number} width
* @param {number} height
* @returns {Engine}
*/
  static new(width: number, height: number): Engine;
/**
* @returns {number}
*/
  get_width(): number;
/**
* @returns {number}
*/
  get_height(): number;
/**
* @returns {number}
*/
  get_score(): number;
/**
* @returns {boolean}
*/
  get_is_clearing(): boolean;
/**
* @returns {boolean}
*/
  get_is_game_over(): boolean;
/**
* @returns {number}
*/
  get_sprite_data(): number;
/**
* @returns {number}
*/
  get_direction_data(): number;
/**
* @returns {number}
*/
  get_piece_part_data(): number;
/**
* @param {number} row
* @param {number} col
* @returns {number}
*/
  get_index(row: number, col: number): number;
/**
*/
  move_piece_right(): void;
/**
*/
  move_piece_left(): void;
/**
*/
  move_piece_down(): void;
/**
*/
  rotate_piece(): void;
/**
*/
  tick(): void;
/**
*/
  reset(): void;
}

// sprites.js
// Sprite drawing

const spritesheet = document.getElementById('spritesheet');

// Popipa
const KASUMI = {
    head: { sx: 0x0, sy: 0x0 },
    tail: { sx: 0x0, sy: 0x20 },
};
const TAE = {
    head: { sx: 0x20, sy: 0x0 },
    tail: { sx: 0x0, sy: 0x20 },
};
const RIMI = {
    head: { sx: 0x40, sy: 0x0 },
    tail: { sx: 0x0, sy: 0x20 },
};
const SAAYA = {
    head: { sx: 0x60, sy: 0x0 },
    tail: { sx: 0x0, sy: 0x20 },
};
const ARISA = {
    head: { sx: 0x80, sy: 0x0 },
    tail: { sx: 0x40, sy: 0x20 },
};

// Afterglow
const RAN = {
    head: { sx: 0x60, sy: 0x20 },
    tail: { sx: 0x60, sy: 0x40 },
};
const MOCA = {
    head: { sx: 0x80, sy: 0x20 },
    tail: { sx: 0x60, sy: 0x40 },
};
const HIMARI = {
    head: { sx: 0x0, sy: 0x40 },
    tail: { sx: 0x60, sy: 0x40 },
};
const TOMOE = {
    head: { sx: 0x20, sy: 0x40 },
    tail: { sx: 0x60, sy: 0x40 },
};
const TSUGUMI = {
    head: { sx: 0x40, sy: 0x40 },
    tail: { sx: 0x60, sy: 0x40 },
};

// Pasupare
const AYA = {
    head: { sx: 0x80, sy: 0x40 },
    tail: { sx: 0x80, sy: 0x60 },
};
const HINA = {
    head: { sx: 0x0, sy: 0x60 },
    tail: { sx: 0x0, sy: 0x80 },
};
const CHISATO = {
    head: { sx: 0x20, sy: 0x60 },
    tail: { sx: 0x80, sy: 0x60 },
};
const MAYA = {
    head: { sx: 0x40, sy: 0x60 },
    tail: { sx: 0x0, sy: 0x80 },
};
const EVE = {
    head: { sx: 0x60, sy: 0x60 },
    tail: { sx: 0x80, sy: 0x60 },
};

// Roselia
const YUKINA = {
    head: { sx: 0x20, sy: 0x80 },
    tail: { sx: 0x60, sy: 0xA0 },
};
const SAYO = {
    head: { sx: 0x40, sy: 0x80 },
    tail: { sx: 0x20, sy: 0xA0 },
};
const LISA = {
    head: { sx: 0x60, sy: 0x80 },
    tail: { sx: 0x80, sy: 0xA0 },
};
const AKO = {
    head: { sx: 0x80, sy: 0x80 },
    tail: { sx: 0x40, sy: 0xA0 },
};
const RINKO = {
    head: { sx: 0x0, sy: 0xA0 },
    tail: { sx: 0x20, sy: 0xA0 },
};

// Harohapi
const KOKORO = {
    head: { sx: 0x0, sy: 0xC0 },
    tail: { sx: 0x20, sy: 0xE0 },
};
const KAORU = {
    head: { sx: 0x20, sy: 0xC0 },
    tail: { sx: 0x40, sy: 0xE0 },
};
const HAGUMI = {
    head: { sx: 0x40, sy: 0xC0 },
    tail: { sx: 0x0, sy: 0xE0 },
};
const KANON = {
    head: { sx: 0x60, sy: 0xC0 },
    tail: { sx: 0x0, sy: 0xE0 },
};
const MISAKI = {
    head: { sx: 0x80, sy: 0xC0 },
    tail: { sx: 0x0, sy: 0xE0 },
};

const spriteLookup = [
    KASUMI, TAE, RIMI, SAAYA, ARISA,
    RAN, MOCA, HIMARI, TOMOE, TSUGUMI,
    AYA, HINA, CHISATO, MAYA, EVE,
    YUKINA, SAYO, LISA, AKO, RINKO,
    KOKORO, KAORU, HAGUMI, KANON, MISAKI,
];

/**
 * Draws sprite on canvas
 * 
 * @param {*} ctx 
 * @param {*} sprite 
 * @param {*} dx 
 * @param {*} dy 
 */
export function drawPiece(ctx, spriteNum, isHead, dx, dy, rotation) {

    const member = spriteLookup[spriteNum];
    const sprite = isHead ? member.head : member.tail;

    ctx.save();
    ctx.translate(dx, dy);
    ctx.rotate(rotation);

    ctx.drawImage(
        spritesheet,
        sprite.sx, sprite.sy, 32, 32,
        -16, -16, 32, 32,
    );

    ctx.restore();
}
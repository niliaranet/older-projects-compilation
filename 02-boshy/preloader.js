import * as ASSETS from "./src/assets.js"

function preload(a) {
    var tempImg = new Image()
    tempImg.src=a
}

function preloadAudio(a) {
    var tempAud = new Audio(a)
    tempAud.load()
}

export function preloadAll() {
Array.from([ASSETS.BOSHY.BULLET, ASSETS.BOSHY.NORMAL, ASSETS.BOSHY.BLOOD]).map(preload)
Object.keys(ASSETS.BOSHY.SOUNDS).map(key => preloadAudio(ASSETS.BOSHY.SOUNDS[key]))
Object.keys(ASSETS.MUSIC).map(key => preloadAudio(ASSETS.MUSIC[key]))
ASSETS.PORTAL.map(preload)
ASSETS.HELLO_KITTY.MAIN.map(preload)
ASSETS.HELLO_KITTY.SMALL.map(preload)
preload(ASSETS.HELLO_KITTY.BULLET)
Object.keys(ASSETS.HELLO_KITTY.SOUNDS).map(key => preloadAudio(ASSETS.HELLO_KITTY.SOUNDS[key]))
preload(ASSETS.WHITE)
preload(ASSETS.GAME_OVER)
preload(ASSETS.ANTICHEESE)
preload(ASSETS.BACKGROUND.KITTY)
}

preloadAll()

///////////////////// IMPORTS //////////////////////
import { Boshy } from "./entities/boshy.js"
import { SCENE, SCENES } from "./scenes.js"
import { HPBar } from "./entities/hp_bar.js"
import { GameOver } from "./entities/game_over.js"
import { Engine } from "./engine.js"
import { MUSIC } from "./assets.js"


///////////////////// GLOBALS //////////////////////
export let boshy
export const entities = {
    list: [],
    remove: instance => { entities.list = entities.list.filter(b => b != instance) },
    clear: _ => { entities.list.forEach(b => b.remove()) }
}
export let enemy

///////////////////// GAME LOGIC //////////////////////
export function start() {
    console.log("game start")
    SCENE.load(SCENES.PREQUEL)
}

export function update() {
    entities.list.forEach((entity) => entity.update())
    SCENE.update()
}

export function draw() {
    entities.list.forEach((entity) => entity.draw())
    SCENE.draw()
}


///////////////////// METHODS //////////////////////
export function reload() {
    if (boshy != undefined) boshy.remove()
    entities.clear()
    enemy = undefined
    boshy = new Boshy()
}

export function setEnemy(e) { enemy = e; new HPBar() }
export function boshyDied() {
    Engine.radio.playMusic(MUSIC.LOLUDIED)
    new GameOver()
}

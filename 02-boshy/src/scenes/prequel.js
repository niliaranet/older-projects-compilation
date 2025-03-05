import { Engine } from "../engine.js"
import { MUSIC } from "../assets.js"
import { Portal } from "../entities/portal.js"

let portal

export function start() {
    Engine.radio.playMusic(MUSIC.PREQUEL)
    portal = new Portal()
}

export function update() {
    portal.update()
}

export function draw() {
    portal.draw()
}

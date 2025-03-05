import { Engine } from "../engine.js"
import { MUSIC } from "../assets.js"
import { HelloKittyBackground } from "../entities/hello_kitty_background.js"
import { HelloKitty } from "../entities/hello_kitty.js"
import * as anticheese from "../anticheese.js"

export function start() {
    Engine.radio.playMusic(MUSIC.HELL)
    new HelloKittyBackground()
    new HelloKitty()
}


export function update() {
    anticheese.update()
}

export function draw() {
}

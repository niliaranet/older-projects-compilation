import * as PREQUEL from  "./scenes/prequel.js"
import * as HELLO_KITTY from  "./scenes/hello_kitty.js"
import { reload as reloadMain } from "./main.js"
import { Engine } from "./engine.js"

export const SCENES = {
    PREQUEL: "prequel",
    HELLO_KITTY: "hello kitty",
}

export const SCENE = {
    load: loadScene,
    update: null,
    draw: null,
    restart: restartScene,
}

export let currentScene

function loadScene(scene) {
    Engine.frame = 0
    currentScene = scene
    reloadMain()

    if (currentScene == SCENES.PREQUEL) {
        PREQUEL.start()
        SCENE.update = PREQUEL.update
        SCENE.draw = PREQUEL.draw
    }

    if (currentScene == SCENES.HELLO_KITTY) {
        HELLO_KITTY.start()
        SCENE.update = HELLO_KITTY.update
        SCENE.draw = HELLO_KITTY.draw
    }
}

function restartScene() { loadScene (currentScene) }

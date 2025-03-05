import { SCENE, SCENES } from "../scenes.js"

export function setUpKeyboard() {
    document.addEventListener("keyup", k => trigger(k))
    document.addEventListener("keydown", k => trigger(k))
}

export const keys = {
    right: false,
    left: false,
    up: false,
    down: false,
    shoot: false,
}

function trigger(event) {
    let on = event.type == "keydown"
    switch (event.key) {
        case "ArrowRight": keys.right = on; break;
        case "ArrowLeft": keys.left = on; break;
        case "ArrowUp": keys.up = on; break;
        case "ArrowDown": keys.down = on; break;
        case "z": case "Z": keys.shoot = on; break;
        case "r": case "R": if (on) SCENE.load(SCENES.PREQUEL); break;
    }
}

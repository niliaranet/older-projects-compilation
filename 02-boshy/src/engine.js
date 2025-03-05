import { start, update, draw } from "./main.js"
import { Screen } from "./engine/screen.js"
import { keys, setUpKeyboard } from "./engine/keyboard.js"
import { startClock } from "./engine/clock.js"
import { Radio } from "./engine/radio.js"
import { Collision } from "./engine/collision.js"

export class Engine {
    static frame = 0
    static screen = new Screen()
    static radio = new Radio()
    static collision = new Collision()
    static keys = keys

    static startEngine() {
        setUpKeyboard()
        start()
        startClock()
    }

    static updateEngine() {
        update()
        Engine.screen.update()
        draw()
    }

    static random(min, max) { 
        return Math.floor(Math.random() * (max + 1 - min) + min);
    }
}

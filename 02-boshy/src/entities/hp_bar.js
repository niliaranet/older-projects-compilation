import { WHITE } from "../assets.js"
import { Vector } from "../engine/vector.js"
import { Engine } from "../engine.js"
import { Entity } from "./entity.js"
import { enemy } from "../main.js"

export class HPBar extends Entity {
    constructor() {
        let size = new Vector(Engine.screen.viewport.x, 20)
        super({
            size: new Vector(Engine.screen.viewport.x, 20),
            position: new Vector(size.x / 2, size.y / 2),
            sprite: WHITE,
            pixelated: true,
        })

        this.span.style.zIndex = 1
        this.lasthp = enemy.hp
    }

    update() {
        if (enemy.hp != this.lasthp) {
            this.lasthp = enemy.hp

            let hpScale = enemy.hp / enemy.max_hp
            let viewport = Engine.screen.viewport

            this.size.x = hpScale * viewport.x
            this.position.x = (viewport.x / 2) - (1 - hpScale) / 2 * viewport.x
        }
    }
}


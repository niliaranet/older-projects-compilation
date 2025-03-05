import { ANTICHEESE } from "../assets.js"
import { Vector } from "../engine/vector.js"
import { Engine } from "../engine.js"
import { Entity } from "./entity.js"
import { boshy } from "../main.js"

export class BigDump extends Entity {
    constructor(bottom = false) {
        let position = bottom ? 
            new Vector(0, boshy.position.y) :
            new Vector(boshy.position.x, 0)

        super({
            size: new Vector(200, 10),
            position: position,
            sprite: ANTICHEESE,
            degrees: bottom ? 270 : 0,
        })

        this.bottom = bottom
    }

    speed = 10
    maxSize = 200

    update() {
        if (this.size.y < this.maxSize) this.size.y += this.speed * 0.75
        if (this.bottom) this.position.x += this.speed
        else this.position.y += this.speed

        this.updateAsHazard()
        this.checkBounds()
    }
}

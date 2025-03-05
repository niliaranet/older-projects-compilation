import { BOSHY } from "../assets.js"
import { Vector } from "../engine/vector.js"
import { Engine } from "../engine.js"
import { Entity } from "./entity.js"

export class Blood extends Entity {
    constructor(position) {
        super({
            size: new Vector(6, 6),
            position: position,
            sprite: BOSHY.BLOOD,
        })

        let max_speed = 20
        this.velocity = new Vector(Engine.random(-max_speed, max_speed),Engine.random(-max_speed, max_speed))
        this.gravity = 1
    }


    update() {
        this.position.add(this.velocity)
        if (Engine.screen.isOffLimits(this.position, this.size)) this.remove()
        this.velocity.y += this.gravity
    }
}

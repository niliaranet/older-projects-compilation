import { HELLO_KITTY } from "../../assets.js"
import { Vector } from "../../engine/vector.js"
import { Engine } from "../../engine.js"
import { Entity } from "../entity.js"

export class BulletParticle extends Entity {
    constructor(position) {
        super({
            size: new Vector(20, 20),
            position: position,
            sprite: HELLO_KITTY.BULLET,
        })
        this.rotation = Engine.random(0, 360)
        this.rotationSpeed = Engine.random(-10, 10)
        this.velocity = new Vector(
            Engine.random(-2, 2),
            Engine.random(-2, 2),
        )
    }

    update() {
        this.position.add(this.velocity)
        this.rotation += this.rotationSpeed
        this.size.x = this.size.y -= 1
        if (this.size.x <= 0) {
            this.remove()
        }
    }
}

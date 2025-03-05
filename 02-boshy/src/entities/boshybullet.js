import { BOSHY } from "../assets.js"
import { Vector } from "../engine/vector.js"
import { Entity } from "./entity.js"
import { enemy } from "../main.js"
import { Engine } from "../engine.js"

export class BoshyBullet extends Entity {
    speed = 20

    constructor(position = new Vector(0,0)) {
        super({
            size: new Vector(5, 5),
            position: position,
            sprite: BOSHY.BULLET,
        })
    }

    update() {
        this.position.y -= this.speed
        if (this.position.y < 0) {
            this.remove()
        }

        if (Engine.collision.collidingWithEnemy(this.position, this.hitbox)) {
            enemy.hit()
            this.remove()
        }
    }
}

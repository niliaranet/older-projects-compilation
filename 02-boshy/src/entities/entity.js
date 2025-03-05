import { Vector } from "../engine/vector.js"
import { Engine } from "../engine.js"
import { entities, boshy } from "../main.js"

export class Entity {
    rotation = 0

    constructor({
        size,
        hitbox = size,
        sprite,
        position = new Vector(0,0),
        pixelated = false,
        degrees = 0,
    }) {
        this.size = size
        this.hitbox = hitbox
        this.position = position
        this.rotation = degrees

        this.span = document.createElement("img")
        this.span.src = sprite
        Engine.screen.append(this.span)

        entities.list.push(this)
        if (pixelated) this.span.className = "pixelated"
    }

    update() {}

    draw() {
        Engine.screen.draw(this.span, this.position, this.size, this.rotation)
    }


    remove() {
        this.span.remove()
        entities.remove(this)
    }

    updateAsHazard() {
        if (Engine.collision.collidingWithBoshy(this.position, this.hitbox)) {
            boshy.die()
        }
    }

    checkBounds() {
        if (Engine.screen.isOffLimits(this.position, this.size)) {
            this.remove()
        }
    }
}

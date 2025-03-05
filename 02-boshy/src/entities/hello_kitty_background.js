import { Vector } from "../engine/vector.js"
import { BACKGROUND } from "../assets.js"
import { Entity } from "../entities/entity.js"
import { Engine } from "../engine.js"

// this is the single worst code in the game
// i'm sorry to whoever is reading this

export class HelloKittyBackground extends Entity {
    speed = 20

    constructor(position = null) {
        let first = (position == null)
        let size = new Vector(1300, 2080)
        let x_margin = 20
        if (position == null) position = new Vector(
                size.x / 2 + Engine.screen.viewport / 2 - x_margin,
                - (size.y / 2) + Engine.screen.viewport.y,
        )
        super({
            size: size,
            position: position,
            sprite: BACKGROUND.KITTY,
        })
        this.span.style.zIndex = -3
        this.maxDownBeforeSpawningNew = (this.size.y / 2)
        this.maxDown = this.size.y / 2 + Engine.screen.viewport.y
        this.x_margin = x_margin
        this.stage = first ? 0 : 1
    }

    update() {
        this.position.y += this.speed
        if (this.stage == 0) { 
            if (this.position.y < this.maxDownBeforeSpawningNew) return
            let margin = this.position.y - this.maxDownBeforeSpawningNew
            let newPos = new Vector(
                this.size.x / 2 - this.x_margin,
                - (this.size.y / 2) + margin
            )

            new HelloKittyBackground(newPos)
            this.stage++
            return
        }
        if (this.position.y >= this.maxDown) {
            this.position.y -= this.size.y * 2
        }
    }
}

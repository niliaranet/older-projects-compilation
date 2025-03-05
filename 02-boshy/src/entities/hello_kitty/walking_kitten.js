import { HELLO_KITTY } from "../../assets.js"
import { Vector } from "../../engine/vector.js"
import { Engine } from "../../engine.js"
import { Entity } from "../entity.js"

export class WalkingKitten extends Entity {
    constructor() {
        let direction = Engine.random(0, 1)
        super({
            size: new Vector(40, 40),
            hitbox: new Vector(20, 40),
            position: new Vector(direction ? 0 : Engine.screen.viewport.x, Engine.screen.viewport.y - 20),
            sprite: HELLO_KITTY.SMALL[0],
            pixelated: true,
        })

        this.velocity = new Vector(direction ? 1 : -1, 0)
        if (direction) this.span.style.transform = "scaleX(-1)"
        this.bottomPosition = Engine.screen.viewport.y - 20
    }

    update() {
        this.updateJump()
        this.position.add(this.velocity)
        this.checkBounds()
        this.updateAsHazard()
    }

    jumping = true
    timeUntilNextJump = 0
    gravity = 0.1

    updateJump() {
        if (this.jumping) {
            if (this.position.y >= this.bottomPosition) {
                this.velocity.y = 0
                this.velocity.x *= 3
                this.position.y = this.bottomPosition
                this.jumping = false
                this.timeUntilNextJump = Engine.random(-30, 300)
                return
            }
            this.velocity.y += this.gravity
            return
        }

        this.timeUntilNextJump--
        if (!this.timeUntilNextJump) {
            this.velocity.x /= 3
            this.velocity.y = -7
            this.jumping = true
        }
    }

    delayBetweenFrames = 20
    timeUntilNextFrame = this.delayBetweenFrames
    frameAlternator = false

    draw() {
        if (!this.timeUntilNextFrame) {
            this.timeUntilNextFrame = this.delayBetweenFrames
            this.span.src = this.frameAlternator ?
                HELLO_KITTY.SMALL[0] :
                HELLO_KITTY.SMALL[1]
            this.frameAlternator = !this.frameAlternator
        }
        this.timeUntilNextFrame--

        super.draw()
    }

    checkBounds() {
        if (
            this.position.x + this.size.x / 2 < 0 ||
            this.position.x - this.size.x / 2 > Engine.screen.viewport.x
        ) {
            this.remove()
        }
    }
}

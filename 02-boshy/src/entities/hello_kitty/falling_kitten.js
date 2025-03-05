import { HELLO_KITTY } from "../../assets.js"
import { Vector } from "../../engine/vector.js"
import { Engine } from "../../engine.js"
import { Entity } from "../entity.js"

export class FallingKitten extends Entity {
    constructor() {
        let direction = Engine.random(0, 1)
        super({
            size: new Vector(40, 40),
            hitbox: new Vector(20, 40),
            position: new Vector(Engine.random(0 + 22, Engine.screen.viewport.x - 22), 0),
            sprite: HELLO_KITTY.SMALL[0],
            pixelated: true,
        })

        this.velocity = new Vector(direction ? 1 : -1, 2)

        this.pointRightDirection()
        this.maxBottomPosition = Engine.screen.viewport.y + this.size.y / 2
    }

    update() {
        this.position.add(this.velocity)
        if (
            this.position.x - this.size.x / 2 < 0 ||
            this.position.x > Engine.screen.viewport.x - this.size.x / 2
        ) {
            this.velocity.x *= -1
            this.pointRightDirection()
        }

        if (this.position.y > this.maxBottomPosition) this.remove()
        this.updateAsHazard()
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

    pointRightDirection() {
        if (this.velocity.x > 0) this.span.style.transform = "scaleX(-1)"
        else this.span.style.transform = "scaleX(1)"
    }
}

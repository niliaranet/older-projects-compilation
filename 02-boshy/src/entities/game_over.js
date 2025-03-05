import { Vector } from "../engine/vector.js"
import { Entity } from "./entity.js"
import { GAME_OVER } from "../assets.js"

export class GameOver extends Entity {
    fadeWaitTime = 40
    opacity = 0
    stage = 0

    zoomDelay = 7
    zoomWaitTime = this.zoomDelay
    zoomGrowth = 10
    zoomedIn = false
    initialSize = 400
    maxSize = 550

    constructor() {
        super({
            size: new Vector(400, 400),
            position: new Vector(325, 250),
            sprite: GAME_OVER,
            pixelated: true,
        })

        this.span.style.zIndex = 5
        this.span.style.opacity = 0
    }

    update() {
        if (this.stage == 0) this.fadeIn()
        this.zoomAnimation()
    }

    fadeIn() {
        if (this.fadeWaitTime > 0) { this.fadeWaitTime--; return; }
        if (this.opacity < 1) {
            this.opacity += 0.05
            this.span.style.opacity = this.opacity
            return
        }
        this.stage = 1
    }

    zoomAnimation() {
        if (this.zoomWaitTime > 0) { this.zoomWaitTime--; return; }
        if (this.zoomedIn) this.size.add(new Vector(-this.zoomGrowth, -this.zoomGrowth))
        else this.size.add(new Vector(this.zoomGrowth, this.zoomGrowth))
        if (
            this.size.x == this.initialSize ||
            this.size.x == this.maxSize
        ) {
            this.zoomedIn = !this.zoomedIn
            this.zoomWaitTime = this.zoomDelay
        }

    }
}


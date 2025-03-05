import { WHITE, HELLO_KITTY } from "../../assets.js"
import { Vector } from "../../engine/vector.js"
import { Engine } from "../../engine.js"
import { Entity } from "../entity.js"
import { Bullet } from "./bullet.js"

export class Laser extends Entity {
    attacking = false
    loadingTimeLeft = 70
    attackTimeLeft = 40

    attackSize = 150
    shrinkSpeed = 15

    constructor(x, delayed) {
        super({
            size: new Vector(40, Engine.screen.viewport.y),
            position: new Vector(x, Engine.screen.viewport.y / 2),
            sprite: WHITE,
            pixelated: true,
        })
        this.span.style.zIndex = -1
        if (delayed) {
            this.size.x = 60
            this.loadingTimeLeft = 100
        }
    }

    update() {
        if (this.loadingTimeLeft) {
            if (this.size.x) this.size.x -= 1
            this.loadingTimeLeft--
            if (!this.loadingTimeLeft) {
                this.attacking = true
                this.size.x = 100
                Engine.radio.playSound(HELLO_KITTY.SOUNDS.LASER)
                Engine.screen.shake()
                this.spawnBullets()
            }
            return
        }

        if (this.attackTimeLeft) {
            if (this.size.x == this.attackSize) this.size.x += 10
            else this.size.x = this.attackSize
            this.attackTimeLeft--
            this.updateAsHazard()
            return
        }

        this.size.x -= this.shrinkSpeed
        if (this.size.x <= 0) this.remove()

    }

    spawnBullets() {
        let bulletCenter = new Vector(this.position.x, Engine.screen.viewport.y)
        let randomness_angle = Engine.random(-20, 20)
        for (let i = -1; i < 2; i++) {
            new Bullet({
                position: bulletCenter.clone(),
                speed: 6,
                spinning: true,
                degrees: 270 + 40 * i + randomness_angle,
                gravity: 0.15,
            })
        }
    }
}

import { HELLO_KITTY } from "../../assets.js"
import { Vector } from "../../engine/vector.js"
import { Entity } from "../entity.js"
import { BulletParticle } from "./bullet_particles.js"

export class Bullet extends Entity {
    constructor({
        position,
        speed,
        spinning = false,
        degrees = 0,
        gravity = 0,
    }) {
        super({
            size: new Vector(30, 30),
            hitbox: new Vector(20, 20),
            position: position,
            sprite: HELLO_KITTY.BULLET,
            degrees: degrees
        })

        this.spinning = spinning
        this.velocity = Vector.fromDeg(degrees)
        this.velocity.multiply(speed)
        // this.velocity.floor()

        this.gravity = gravity

        if (speed > 6) this.emittingParticles = true
    }

    particleReloadTime = 10
    particleTimeWait = this.particleReloadTime
    spinSpeed = 10

    update() {
        this.velocity.y += this.gravity
        this.position.add(this.velocity)

        if (this.emittingParticles) {
            if (!this.particleTimeWait) {
                this.particleTimeWait = this.particleReloadTime
                new BulletParticle(this.position.clone())
            }
            this.particleTimeWait--
        }

        if (this.spinning) this.rotation += this.spinSpeed

        this.updateAsHazard()
        this.checkBounds()
    }
}

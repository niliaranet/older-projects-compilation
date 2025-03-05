import { Entity } from "./entity.js"
import { PORTAL } from "../assets.js"
import { Vector } from "../engine/vector.js"
import { Engine } from "../engine.js"
import { SCENE, SCENES } from "../scenes.js"

export class Portal extends Entity {
    constructor() {
        super({
            size: new Vector(150, 150),
            position: new Vector(325, 100),
            sprite: PORTAL[0],
        })

        frames = PORTAL.length
    }

    update() {
        if (Engine.collision.collidingWithBoshy(this.position, this.hitbox)) {
            SCENE.load(SCENES.HELLO_KITTY)
        }
    }

    draw() {
        this.span.src = PORTAL[Math.floor(Engine.frame / 2) % frames]
        super.draw()
    }
}

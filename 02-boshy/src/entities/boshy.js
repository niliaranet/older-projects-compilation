import { BOSHY } from "../assets.js"
import { Vector } from "../engine/vector.js"
import { Engine } from "../engine.js"
import { BoshyBullet } from "./boshybullet.js"
import { Entity } from "./entity.js"
import { SCENES, currentScene } from "../scenes.js"
import { boshyDied } from "../main.js"
import { Blood } from "./boshyblood.js"

export class Boshy extends Entity {
    reloadTime = 6
    lastShot = 0
    speed = 4

    canmove = true
    dead = false

    constructor() {
        super({
            size: new Vector(30, 30),
            hitbox: new Vector(20, 20),
            position: new Vector(325, 400),
            sprite: BOSHY.NORMAL,
        })

        /* play initial sound */
        switch (currentScene) {
            case SCENES.PREQUEL:
            Engine.radio.playSound(BOSHY.SOUNDS.INTRO2)
            break;

            case SCENES.HELLO_KITTY:
            Engine.radio.playSound(BOSHY.SOUNDS.INTRO)
            break;
        }
    }

    update() {
        if (!this.canmove) return
        let keys = Engine.keys
        let viewport = Engine.screen.viewport

        /* get movement direction */
        let move = new Vector()
        let speed = this.speed
        if (keys.right) move.x += 1
        if (keys.left) move.x -= 1
        if (keys.up) move.y -= 1
        if (keys.down) move.y += 1
        if (keys.shoot) this.shoot()

        /* apply movement */
        move.normalize()
        move.multiply(speed)
        this.position.add(move)

        /* check limits */
        if (this.position.x < 0) this.position.x = 0
        if (this.position.y < 0) this.position.y = 0
        if (this.position.x > viewport.x) this.position.x = viewport.x
        if (this.position.y > viewport.y) this.position.y = viewport.y
    }

    shoot() {
        /* check if it can shoot */
        if (Engine.frame < this.lastShot + this.reloadTime) {
            return
        }

        this.lastShot = Engine.frame /*(reset reload)*/

        /* it was supposed to shoot twice */
        // let bullet_a_position = this.position.clone()
        // let bullet_b_position = this.position.clone()

        // bullet_a_position.x = bullet_a_position.x + 5
        // bullet_b_position.x = bullet_b_position.x - 5

        // new BoshyBullet(bullet_a_position)
        // new BoshyBullet(bullet_b_position)

        /* but once again, we need to save resources */
        new BoshyBullet(this.position.clone())
        Engine.radio.playSound(BOSHY.SOUNDS.SHOOT)
    }

    die() {
        if (this.dead) return
        this.dead = true
        this.playDeathSounds()
        this.spawnBlood()
        Engine.screen.shake()
        
        this.position.x = 0
        this.position.y = Engine.screen.viewport.y
        this.remove()
        boshyDied()
    }

    playDeathSounds() {
        Engine.radio.playSound(BOSHY.SOUNDS.DEAD)
        
        switch (Engine.random(1, 4)) {
            case 1: Engine.radio.playSound(BOSHY.SOUNDS.DEADQUOTE1); break;
            case 2: Engine.radio.playSound(BOSHY.SOUNDS.DEADQUOTE2); break;
            case 3: Engine.radio.playSound(BOSHY.SOUNDS.DEADQUOTE3); break;
            case 4: Engine.radio.playSound(BOSHY.SOUNDS.DEADQUOTE4); break;
        }
    }

    spawnBlood() {
        for (let i = 0; i<20; i++) {
            new Blood(this.position.clone())
        }
    }
}

import { Vector } from "../engine/vector.js"
import { Engine } from "../engine.js"
import { Entity } from "./entity.js"
import { HELLO_KITTY } from "../assets.js"
import { setEnemy, boshy } from "../main.js"
import { Bullet } from "./hello_kitty/bullet.js"
import { Laser } from "./hello_kitty/laser.js"
import { WalkingKitten } from "./hello_kitty/walking_kitten.js"
import { FallingKitten } from "./hello_kitty/falling_kitten.js"
import { Blood } from "./boshyblood.js"

export class HelloKitty extends Entity {
    max_hp = 200
    hp = this.max_hp
    alive = true

    stage = 0
    justGotHit = false
    turningRight = false

    bulletMarginY = 40

    constructor() {
        super({
            size: new Vector(200, 200),
            hitbox: new Vector(100, 200),
            position: new Vector(325, 140),
            sprite: HELLO_KITTY.MAIN[0],
            pixelated: true,
        })


        this.span.style.zIndex = 1
        setEnemy(this)
        this.nextStage()
    }

    update() {
        if (!this.alive) {
            this.deathAnimation()
            return
        }
        switch (this.stage) {
            case 1: this.stage1(); break
            case 2: this.stage2Lasers(); break
            case 3: this.stage3Kittens(); break
            case 4: this.stage4Touhou(); break
            default: this.nextStage()
        }
        this.updateAsHazard()
    }

    delayBetweenFrames = 10
    timeUntilNextFrame = this.delayBetweenFrames
    frameAlternator = false
    draw() {
        if (!this.alive) { super.draw(); return }
        /* hit effect */
        if (this.justGotHit) {
            this.span.style.filter = "brightness(500%)"
            this.justGotHit = false
        } else {
            this.span.style.filter = "brightness(100%)"
        }

        if (this.speed > 0) this.span.style.transform = "scaleX(-1)"
        else if (this.speed < 0) this.span.style.transform = "scaleX(1)"

        /* frame */
        /* this is what it should be doing */
        // this.span.src = (Math.floor(Engine.frame / 10 % 2) == 0) ?
        //     HELLO_KITTY.MAIN[0] :
        //     HELLO_KITTY.MAIN[1]
        /* but no hi ha pressupost for it */
        /* here's the alternative: */
        if (!this.timeUntilNextFrame) {
            this.timeUntilNextFrame = this.delayBetweenFrames
            this.span.src = this.frameAlternator ?
                HELLO_KITTY.MAIN[0] :
                HELLO_KITTY.MAIN[1]
            this.frameAlternator = !this.frameAlternator
        }
        this.timeUntilNextFrame--

        /* draw */
        super.draw()
    }

    normalAttackDelay = 100
    attackWaitTime = this.normalAttackDelay

    nextStage() {
        this.stage++

        switch (this.stage) {
            case 1:
                this.attacksUntilNextStage = 6
                this.attackWaitTime = this.normalAttackDelay
                this.prepareMoving()
                break
            case 2:
                this.innerStage = 0
                this.timeUntilNextInnerStage = 0
                this.attacksUntilNextStage = 4
                break
            case 3:
                this.attacksUntilNextStage = 12
                break
            case 4:
                this.innerStage = 0
                this.timeUntilNextInnerStage = 120
                break
            default:
                this.stage = 0
                this.nextStage()
        }
    }

    /* single shoots stage */
    stage1() {
        this.move()

        if (!this.attackWaitTime) {
            if (!this.attacksUntilNextStage) {
                this.nextStage()
                return
            }

            this.normalAttack()
            this.attackWaitTime = this.normalAttackDelay
            this.attacksUntilNextStage--

        }
        this.attackWaitTime--

    }

    innerStage = 0
    timeUntilNextInnerStage = 0

    /* laser stage */
    stage2Lasers() {
        switch (this.innerStage) {
            case 0: /*(preparation)*/
                this.laserAttackDelay = 120
                this.attackWaitTime = this.laserAttackDelay + 100

                this.timeUntilNextInnerStage = 120
                this.innerStage++
            case 1: /*(chasing player)*/
                this.moveToPlayer()
                this.timeUntilNextInnerStage--
                if (!this.timeUntilNextInnerStage) {
                    this.timeUntilNextInnerStage = 100
                    this.innerStage++
                    this.laserAttack(true)
                }
                return
            case 2: /*(first laser shot, waiting)*/
                this.timeUntilNextInnerStage--
                if (!this.timeUntilNextInnerStage) {
                    this.prepareMoving()
                    this.innerStage++
                }
                return
        }

        /* post-preparation behaviour */
        this.move()
        if (!this.attackWaitTime) {
            if (!this.attacksUntilNextStage) {
                this.nextStage()
                return
            }

            this.laserAttack()

            this.attackWaitTime = this.laserAttackDelay
            this.laserAttackDelay -= 30
            this.attacksUntilNextStage--
        }
        this.attackWaitTime--
    }

    kittenAttackDelay = 40
    stage3Kittens() {
        this.move()

        if (!this.attackWaitTime) {
            if (!this.attacksUntilNextStage) {
                this.nextStage()
                return
            }
            this.kittenAttack()
            this.attackWaitTime = this.kittenAttackDelay
            this.attacksUntilNextStage--
        }
        this.attackWaitTime--
    }

    /* bullets!!! */
    stage4Touhou() {
        switch (this.innerStage) {
            case 0: /*(still moving)*/
                this.move()

                this.timeUntilNextInnerStage--
                if (!this.timeUntilNextInnerStage) {
                    this.innerStage++
                    this.timeUntilNextInnerStage = 30
                }
                return

            case 1: /*(go to center)*/
                this.moveToCenter()
                this.timeUntilNextInnerStage--
                if (!this.timeUntilNextInnerStage) {
                    this.position.x = 325 /*(center of the screen)*/
                    this.innerStage++
                    this.attacksUntilNextInnerStage = 5
                    this.attackReload = 40
                    this.attackWaitTime = 20
                }
                return

            case 2: /*(circle attacks)*/
                this.attackWaitTime--
                if (this.attackWaitTime > 0) return

                this.circleAttack()
                this.attackWaitTime = this.attackReload

                this.attacksUntilNextInnerStage--
                if (!this.attacksUntilNextInnerStage) {
                    this.innerStage++
                    this.attacksUntilNextInnerStage = 8
                    this.attackReload = 30
                    this.extraAttackReload = 10
                    this.isNextAttackNormal = true
                }
                return

            case 3: /*(circle attacks + kaboom)*/
                this.attackWaitTime--
                if (this.attackWaitTime > 0) return

                if (this.isNextAttackNormal) {
                    this.circleAttack()
                    this.attackWaitTime = this.extraAttackReload
                } else {
                    this.circleAttackFaster()
                    this.attackWaitTime = this.attackReload
                }
                this.isNextAttackNormal = !this.isNextAttackNormal

                this.attacksUntilNextInnerStage--
                if (!this.attacksUntilNextInnerStage) {
                    this.innerStage++
                    this.attacksUntilNextInnerStage = 6
                    this.attackReload = 30
                    this.extraAttackReload = 10
                }
                return

            default:
                this.nextStage()
                return
        }
    }


    speed = 9
    acceleration = 0.2

    prepareMoving() {
        this.position.x = Engine.screen.viewport.x / 2
        this.speed = 9
    }

    moveInterval = 26
    move() {
        this.position.x += this.speed
        if (this.position.x > 325) this.speed -= this.acceleration
        else this.speed += this.acceleration
    }

    moveToPlayer() {
        let distance = Math.floor(this.position.x - boshy.position.x)

        switch (true) {
            case (distance < 10 && distance > -10): this.speed = 0; break
            case (distance < -50): this.speed = 10; break
            case (distance < -20): this.speed = 5; break
            case (distance < 0): this.speed = 2; break
            case (distance > 50): this.speed = -10; break
            case (distance > 20): this.speed = -5; break
            case (distance > 0): this.speed = -2; break
        }
        this.position.x += this.speed
    }

    moveToCenter() {
        let distance = this.position.x - 325 /*(viewport.y / 2)*/
        if (distance < 2 && distance > -2) return

        switch (true) {
            case (distance < -50): this.speed = 10; break
            case (distance < -20): this.speed = 5; break
            case (distance < 0): this.speed = 2; break
            case (distance > 50): this.speed = -10; break
            case (distance > 20): this.speed = -5; break
            case (distance > 0): this.speed = -2; break
        }
        this.position.x += this.speed
    }

    normalAttack() {
        let angleToPlayer = this.getBulletAngleToPlayer()
        let bulletCenter = this.position.clone()
        bulletCenter.y += this.bulletMarginY
        new Bullet({
            position: bulletCenter.clone(),
            speed: 10,
            degrees: angleToPlayer + Engine.random(-10, 10),
        })

        Engine.screen.shake()
    }

    laserAttack(delayed = false) {
        new Laser(boshy.position.x, delayed)
    }

    kittenAttack() {
        new WalkingKitten()
        new FallingKitten()
    }

    circleAttack() {
        let bulletCenter = this.position.clone()
        bulletCenter.y += this.bulletMarginY
        let bullets = 10
        let interval = 360 / bullets
        let initialAngle = this.getBulletAngleToPlayer()

        for (let i = 0; i < bullets; i++) {
            new Bullet({
                position: bulletCenter.clone(),
                speed: 3,
                degrees: initialAngle + interval * i,
                spinning: true,
            })
        }

        Engine.radio.playSound(HELLO_KITTY.SOUNDS.SHOOT)
    }

    circleAttackFaster() {
        let bulletCenter = this.position.clone()
        bulletCenter.y += this.bulletMarginY
        let bullets = 12
        let interval = 360 / bullets
        let initialAngle = Engine.random(0, 360)

        for (let i = 0; i < bullets; i++) {
            new Bullet({
                position: bulletCenter.clone(),
                speed: 7,
                degrees: initialAngle + interval * i,
                spinning: true,
            })
        }

        Engine.radio.playSound(HELLO_KITTY.SOUNDS.SHOOT_LOUD)
        Engine.screen.shake()
    }

    getBulletAngleToPlayer() {
        let v1 = this.position.clone()
        v1.y += this.bulletMarginY

        let v2 = boshy.position
        let diff = Vector.substraction(v2, v1)
        return Math.floor(diff.toDeg())
    }

    hit() {
        if (!this.alive) return

        this.hp--

        if (this.hp == 0) { this.die(); return }

        if (!this.justGotHit) Engine.radio.playSound(HELLO_KITTY.SOUNDS.HIT)
        this.justGotHit = true
    }

    async die() {
        this.alive = false
        Engine.radio.stopMusic()
        await Engine.radio.playSoundUntilEnd(HELLO_KITTY.SOUNDS.DEATH)
        this.explode()
    }

    deathAnimationDelay = 60
    deathAnimation() {
        if (this.deathAnimationDelay) {
            this.deathAnimationDelay--
            return
        }

        this.size.x += 1
    }

    async explode() {
        for (let i = 0; i < 40; i++) {
            new Blood(this.position.clone())
        }
        this.remove()
        await Engine.radio.playSoundUntilEnd(HELLO_KITTY.SOUNDS.BLOOD_RIP)
        Engine.radio.playSound(HELLO_KITTY.SOUNDS.WIN)
    }
}

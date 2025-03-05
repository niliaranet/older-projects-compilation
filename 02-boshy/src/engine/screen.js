import { Vector } from "./vector.js"

export class Screen {
    viewport = new Vector(650, 500)

    ///////////////////// START //////////////////////
    constructor() {
        this.createHTMLElements()
        this.setUpTriggers()
        this.resize()
    }

    createHTMLElements() {
        this.div
        this.spawn

        /* create main div */
        this.div = document.createElement("div")
        this.div.id = "screen"
        this.div.style.backgroundColor = "black"
        this.div.style.zIndex = -11

        /* create relative div to append children */
        this.spawn = document.createElement("div")
        this.div.appendChild(this.spawn)
        document.body.appendChild(this.div)
    }

    setUpTriggers() {
        addEventListener("resize", () => { this.resize() });
    }


    ///////////////////// RESIZE //////////////////////
    real_position = new Vector(0, 0)
    real_size = new Vector(0, 0)
    scale = new Vector(0, 0)

    resize() {
        let w = new Vector(window.innerWidth, window.innerHeight)

        /* get max size keeping aspect ratio */
        if (w.y / this.viewport.y > w.x / this.viewport.x) {
            this.real_size.x = w.x
            this.real_size.y = Math.floor(w.x / this.viewport.x * this.viewport.y)
        } else {
            this.real_size.y = w.y
            this.real_size.x = Math.floor(w.y / this.viewport.y * this.viewport.x)
        }
        this.div.style.width = `${this.real_size.x}px`
        this.div.style.height = `${this.real_size.y}px`

        /* get centered position */
        this.real_position.x = w.x / 2 - this.real_size.x / 2
        this.real_position.y = w.y / 2 - this.real_size.y / 2
        this.real_position.floor()

        this.div.style.left = `${this.real_position.x}px`
        this.div.style.top = `${this.real_position.y}px`

        /* set scale */
        this.scale = this.real_size.x / this.viewport.x
    }


    ///////////////////// COMMANDS //////////////////////
    append(element) { this.spawn.appendChild(element) }

    draw(span, position, size, rotation) {
        span.style.width = `${size.x * this.scale}px`
        span.style.height = `${size.y * this.scale}px`
        span.style.top = `${(position.y - size.y / 2 + this.additionalPosition.y) * this.scale}px`
        span.style.left = `${(position.x - size.x / 2 + this.additionalPosition.x) * this.scale}px`
        span.style.rotate = `${rotation}deg`
    }

    isOffLimits(position, size) {
        return (
            position.x + size.x / 2 < 0 ||
            position.y + size.y / 2 < 0 ||
            position.x - size.x / 2 > this.viewport.x ||
            position.y - size.y / 2 > this.viewport.y
        )
    }

    shaking = false
    additionalPosition = new Vector(0, 0)
    shake() {
        this.shaking = true
        this.shakingTimeLeft = 6
    }

    update() {
        if (this.shaking) {
            switch (this.shakingTimeLeft) {
                case 6:
                this.additionalPosition.x = 9
                this.additionalPosition.y = -3
                break

                case 5:
                this.additionalPosition.x = -8
                this.additionalPosition.y = 2
                break

                case 4:
                this.additionalPosition.x = 5
                this.additionalPosition.y = 1
                break

                case 3:
                this.additionalPosition.x = -4
                this.additionalPosition.y = -2
                break

                case 2:
                this.additionalPosition.x = 3
                this.additionalPosition.y = -1
                break

                case 1:
                this.additionalPosition.x = 0
                this.shaking = false
                break
            }
            this.shakingTimeLeft --
        }
    }
}

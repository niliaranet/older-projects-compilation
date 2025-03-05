{
    const carrousel = document.getElementById("svg-section")
    const svgNodes = Array.from(carrousel.querySelectorAll(".svg"))
    let carrouselX = carrousel.getBoundingClientRect().width
    let time = 0

    function update() {
        setInterval(() => {
            svgs.map(svg => {
                svg.update()
            })

            time++;
        }, 10);
    }


    class Svg {
        static id = 0
        static jump = svgNodes.length * 300 - carrouselX
        static defaultSpeed = -2
        static speed = this.defaultSpeed

        constructor(element) {
            this.element = element
            this.id = Svg.id++
            this.x = 300 * this.id - 300
            this.width = this.element.getBoundingClientRect().width
            this.fixY()
        }

        fixY() {
            let rect = this.element.getBoundingClientRect()
            let maxH = carrousel.getBoundingClientRect().height
            if (rect.height < maxH) this.element.style.top = `${(maxH - rect.height) / 2}px`
        }

        toMiddle() {
            this.x = 0
            this.updatePosition()
        }

        update() {
            this.move(Svg.speed)
            this.checkLimit()
        }

        move(x) {
            this.x += x
            this.updatePosition()
        }

        updatePosition() {
            let newLeft = (carrouselX - this.width) / 2 + this.x
            this.element.style.left = `${newLeft}px`
        }

        checkLimit() {
            if (Svg.speed < 0) {

            } else if (Svg.speed < 0) {
            } else {
            }

            let s = Svg.speed < 0 ? -1 : Svg.speed > 0 ? 1 : 0
            if (s == 0) return


            if (
                this.x * s > 0 &&
                this.x * s > (carrouselX + 300) / 2
            ) this.x += -(carrouselX + Svg.jump) * s
        }
    }

    const svgs = svgNodes.map(element => new Svg(element))


    window.addEventListener("resize", _ => {
        carrouselX = carrousel.getBoundingClientRect().width
        Svg.jump = svgNodes.length * 300 - carrouselX
    })

    carrousel.addEventListener("mouseleave", _ => {
        Svg.speed = Svg.defaultSpeed
    })

    carrousel.addEventListener("mousemove", e => {
        let carrouselAxis = carrousel.getBoundingClientRect().x
        let posX = e.pageX - carrouselAxis
        let percentage = (posX * 100) / carrouselX

        let unscaledSpeed = -Math.trunc(percentage / 10) + 4
        console.log(unscaledSpeed)
        if (Math.abs(unscaledSpeed) < 2) unscaledSpeed = 0
        if (Math.abs(unscaledSpeed) > 3) unscaledSpeed += Math.abs(unscaledSpeed) / unscaledSpeed
        Svg.speed = unscaledSpeed
    })

    update()
}

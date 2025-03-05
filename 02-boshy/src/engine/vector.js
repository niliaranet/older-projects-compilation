export class Vector {
    constructor(x=0, y=0) {
        this.x = x
        this.y = y
    }

    add(newVector) {
        this.x += newVector.x
        this.y += newVector.y
    }

    normalize() {
        if (this.x != 0 && 0 != this.y) {
            /* this is what it should be doing: */
            // let hyp = Math.abs(this.x)
            // let cSquared = Math.pow(hyp, 2) / 2
            // let c = Math.sqrt(cSquared, 2)

            // this.x = this.x / Math.abs(this.x) * c
            // this.y = this.y / Math.abs(this.x) * c

            /* but we need to save resources */
            this.x = this.x / 4 * 3
            this.y = this.y / 4 * 3
        }
    }

    multiply(multiplier) {
        this.x *= multiplier
        this.y *= multiplier
    }

    clone() {
        return new Vector(this.x, this.y)
    }

    static substraction(vector_a, vector_b) {
        return new Vector(
            vector_a.x - vector_b.x,
            vector_a.y - vector_b.y
        )
    }

    static division(vector_a, vector_b) {
        return new Vector(
            vector_a.x / vector_b.x,
            vector_a.y / vector_b.y
        )
    }

    floor() {
        this.x = Math.floor(this.x)
        this.y = Math.floor(this.y)
        return this
    }

    toDeg() {
        var angle = Math.atan2(this.y, this.x)   //radians
        var degrees = 180*angle/Math.PI  //degrees
        return (360+Math.round(degrees))%360 //round number, avoid decimal fragments
    }

    static fromDeg(degrees) {
        let radians = degrees*Math.PI/180
        return new Vector(
            Math.cos(radians),
            Math.sin(radians)
        )
    }
}

import { boshy, enemy } from "../main.js"
import { Vector } from "./vector.js"

export class Collision {
    collidingWithBoshy(position, hitbox = position) {
        if (boshy == undefined) return false
        return this.areColliding({
            position_a: position,
            position_b: boshy.position,
            hitbox_a: hitbox,
            hitbox_b: boshy.hitbox,
        })
    }
    
    collidingWithEnemy(position, hitbox = position) {
        if (enemy == undefined) return false
        return this.areColliding({
            position_a: position,
            position_b: enemy.position,
            hitbox_a: hitbox,
            hitbox_b: enemy.hitbox,
        })
    }

    areColliding({
        position_a,
        position_b,
        hitbox_a= position_a,
        hitbox_b= position_b,
    }) {
        let hitbox_value = new Vector(
            (Math.abs(hitbox_a.x) + Math.abs(hitbox_b.x)) / 2,
            (Math.abs(hitbox_a.y) + Math.abs(hitbox_b.y)) / 2,
        )
        let position_difference = new Vector(
            Math.abs( Math.abs(position_a.x) - Math.abs(position_b.x)),
            Math.abs( Math.abs(position_a.y) - Math.abs(position_b.y)),
        )

        return (
        position_difference.x <= hitbox_value.x &&
        position_difference.y <= hitbox_value.y
        )
    }
}

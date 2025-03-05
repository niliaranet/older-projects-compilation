import { boshy } from "./main.js"
import { Engine } from "./engine.js"
import { BigDump } from "./entities/bigdump.js"

const maxTime = 300
const margin = 30
let timeH = 0
let timeV = 0

export function update() {
    if (boshy.dead) return

    timeH = (
        boshy.position.x > Engine.screen.viewport.x - margin ||
        boshy.position.x < margin
    ) ? timeH + 1 : 0

    timeV = (
        boshy.position.y > Engine.screen.viewport.y - margin ||
        boshy.position.y < margin
    ) ? timeV + 1 : 0

    if (timeH == maxTime) new BigDump(false)
    if (timeV == maxTime) new BigDump(true)
}

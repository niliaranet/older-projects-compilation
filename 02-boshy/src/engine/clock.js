import { Engine } from "../engine.js"
const fps = 60

/* cap the game's fps */
let msPrev = window.performance.now()
const msPerFrame = 1000 / fps
function tick() {
    window.requestAnimationFrame(tick)
    
    const msNow = window.performance.now()
    const msPassed = msNow - msPrev
    
    if (msPassed < msPerFrame) return
    
    const excessTime = msPassed % msPerFrame
    msPrev = msNow - excessTime
    
    Engine.frame++
    Engine.updateEngine()
}

export function startClock() {
    tick()
}

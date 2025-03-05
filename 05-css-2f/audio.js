const audioPlayer = document.getElementById("audio-player")
const audio = document.getElementById("audio");

//----------//
// Waveform //
//----------//
{
    if (typeof Wave != typeof undefined) {
        let canvasElement = document.getElementById("wave");
        let wave = new Wave(audio, canvasElement);

        wave.addAnimation(new wave.animations.Wave({
            lineWidth: 10,
            lineColor: { gradient: ["#ff9966", "#ff5e62"] },
            count: 20
        }));
    }
}


//-------------//
// Progressbar //
//-------------//
{
    const progress = audioPlayer.querySelector(".progress")
    const progressBarTop = progress.querySelector(".bar-top")
    audio.addEventListener("timeupdate", function() {
        progressBarTop.style.width = `${audio.currentTime / audio.duration * 100}%`
    });

    progress.onclick = e => {
        let mousePos = e.pageX - progress.getBoundingClientRect().x
        audio.currentTime = audio.duration * mousePos / progress.offsetWidth
    }

    const progressBall = progress.querySelector(".ball")
    let mouseOverProgress = false

    progress.onmouseenter = _ => {
        mouseOverProgress = true
    }

    progress.onmouseleave = _ => {
        mouseOverProgress = false
    }

    progress.onmousemove = e => {
        if (mouseOverProgress) {
            let newPos = e.pageX - progress.getBoundingClientRect().x
            newPos -= progressBall.getBoundingClientRect().width / 2
            progressBall.style.left = `${newPos}px`
        }
    }
}


//----------//
// controls //
//----------//
{
    audioPlayer.querySelector(".controls").onmousedown = e => {
        e.preventDefault()
    }

    const bLoop = audioPlayer.querySelector(".bLoop")
    const bHardBackward = audioPlayer.querySelector(".bHardBack")
    const bBackward = audioPlayer.querySelector(".bBack")
    const bPlay = audioPlayer.querySelector(".bPlay")
    const bForward = audioPlayer.querySelector(".bForward")
    const bHardForward = audioPlayer.querySelector(".bHardForward")
    const bVolume = audioPlayer.querySelector(".bSettings")

    const state = audioPlayer.querySelector(".state")
    bPlay.addEventListener("click", _ => {
        audio.paused ? audio.play() : audio.pause()
    })

    bForward.addEventListener("click", _ => {
        audio.currentTime += 10
    })

    bBackward.addEventListener("click", _ => {
        audio.currentTime -= 10
    })

    bHardForward.addEventListener("click", _ => {
        audio.currentTime = audio.duration
    })

    bHardBackward.addEventListener("click", _ => {
        audio.currentTime = 0
    })

    bLoop.addEventListener("click", e => {
        audio.loop = !audio.loop
        audio.loop ?
            e.target.classList.add("triggered") :
            e.target.classList.remove("triggered")
    })

    audio.volume = 0.75


    const decos = [
        audioPlayer.querySelectorAll(".deco2"),
        audioPlayer.querySelectorAll(".deco3"),
        audioPlayer.querySelectorAll(".deco4"),
    ]

    audio.onplay = (_ => {
        bPlay.innerText = "ll"
        state.innerText = "playing"

        let delay = 0
        decos.map(deco => {
            delay += 0.3
            Array.from(deco).map(d => { d.style.animationDelay = `${delay}s` })
        })
    })

    audio.onpause = (_ => {
        bPlay.innerText = "▷"
        state.innerText = "paused"
        decos.map(deco => { Array.from(deco).map(d => { d.style.animationDelay = "0s" }) })
    })

    const audioControl = audioPlayer.querySelector(".audioControl")
    const barBack = audioControl.querySelector(".bar-back")
    const barTop = audioControl.querySelector(".bar-top")
    const volumeBall = audioControl.querySelector(".ball")
    let mouseOverVolume = false

    audioControl.onmouseenter = _ => {
        mouseOverVolume = true
    }

    audioControl.onmouseleave = _ => {
        mouseOverVolume = false
    }

    audioControl.onmousemove = e => {
        if (mouseOverVolume) {
            let newPos = audioControl.getBoundingClientRect().top + window.pageYOffset + audioControl.getBoundingClientRect().height - e.pageY
            newPos -= volumeBall.getBoundingClientRect().height / 2

            let em = parseFloat(getComputedStyle(audioControl).fontSize)
            newPos += 0.5 * em


            if (newPos < 0) newPos = 0

            let max = barBack.getBoundingClientRect().height - 0.5 * em
            if (newPos > max) newPos = max
            volumeBall.style.bottom = `calc(${newPos}px - 0.25em)`
        }
    }

    audioControl.onclick = e => {
        let newPos = audioControl.getBoundingClientRect().top + window.pageYOffset + audioControl.getBoundingClientRect().height - e.pageY
        let em = parseFloat(getComputedStyle(audioControl).fontSize)
        newPos += 0.5 * em

        if (newPos < 0) newPos = 0

        let max = barBack.getBoundingClientRect().height - 0.5 * em
        if (newPos > max) newPos = max

        audio.volume = newPos / max
    }

    audio.onvolumechange = e => {
        barTop.style.height = `calc(${e.target.volume * 100}% - 1.25em)`
    }

    let volButtonTriggered = false
    bVolume.onclick = e => {
        if (e.target != bVolume) return
        volButtonTriggered = !volButtonTriggered
        switch (volButtonTriggered) {
            case true:
                bVolume.classList.add("triggered")
                break
            case false:
                bVolume.classList.remove("triggered")
                break
        }
    }

    audioPlayer.onclick = e => {
        if (
            volButtonTriggered &&
            e.target != bVolume
        ) {
            volButtonTriggered = false
            bVolume.classList.remove("triggered")
        }
    }
}

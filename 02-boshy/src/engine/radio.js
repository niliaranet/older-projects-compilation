export class Radio {
    playSound(file) {
        let audio = new Audio(file)
        audio.play()
    }

    playSoundUntilEnd(file) {
        let audio = new Audio(file)
        return new Promise(res => {
            audio.play()
            audio.onended = res
        })
    }

    bgm
    playMusic(file) {
        if (this.bgm != undefined) this.bgm.pause()
        this.bgm = new Audio(file)
        this.bgm.play()
        this.bgm.loop = true
    }

    stopMusic() {
        this.bgm.pause()
    }
}

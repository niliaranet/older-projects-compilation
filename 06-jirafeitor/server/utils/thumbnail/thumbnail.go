package thumbnail

import (
	"log"
	"os"
	"os/exec"

	"github.com/nil0j/jirafeitor/config"
)

func Gen(input, outputDir string) {
	thumbnailOutput := outputDir + "thumbnail.png"
	err := exec.Command(
		"ffmpeg",
		"-i",
		input,
		"-ss",
		"00:00:10.000",
		"-vframes",
		"1",
		thumbnailOutput,
	).Run()

	if err != nil {
		log.Println(err)
	}

	if _, err := os.Stat(thumbnailOutput); err != nil {
		exec.Command(
			"cp",
			config.Data.Static+"placeholder.png",
			thumbnailOutput,
		).Run()
	}
}

package handlers

import (
	"fmt"
	"os"

	"github.com/gin-gonic/gin"
	"github.com/nil0j/jirafeitor/config"
	"github.com/nil0j/jirafeitor/models/postgres"
	"github.com/nil0j/jirafeitor/models/responses"
	"github.com/nil0j/jirafeitor/repository"
	"github.com/nil0j/jirafeitor/utils/thumbnail"
)

// @Tags Videos
// @Param name formData string true "Video name"
// @Param description formData string true "Video description"
// @Param video formData file true "Video file"
// @Success 200 {object} responses.jsonSuccess
// @Failure 400 {object} responses.jsonError
// @Security JWT
// @Router /upload [post]
func UploadVideo(c *gin.Context) {
	name := c.PostForm("name")
	description := c.PostForm("description")

	file, err := c.FormFile("video")
	if err != nil {
		c.JSON(400, gin.H{
			"message": fmt.Sprintf("%v", err),
		})
		return
	}

	inputId, _ := c.Get("UserID")

	userId := 0
	switch t := inputId.(type) {
	case int:
		userId = t
	}

	id, err := repository.CreateVideo(postgres.Video{
		Name:        name,
		Description: description,
		UserID:      userId,
	})

	if err != nil {
		c.JSON(400, fmt.Sprintf("error: %v", err))
		return
	}

	videoOutFolderPath := config.Data.Filesystem + fmt.Sprintf("%d", id) + "/"
	if err := os.MkdirAll(videoOutFolderPath, 0755); err != nil {
		c.JSON(400, fmt.Sprintf("error: %v", err))
		return
	}

	videoOutPath := videoOutFolderPath + file.Filename
	c.SaveUploadedFile(file, videoOutPath)

	thumbnail.Gen(videoOutPath, videoOutFolderPath)
	responses.HandleSuccess(c, "File uploaded correctly!")
}

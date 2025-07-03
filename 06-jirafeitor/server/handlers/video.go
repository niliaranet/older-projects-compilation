package handlers

import (
	"net/http"
	"os"
	"path/filepath"
	"strconv"

	"github.com/gin-gonic/gin"
	"github.com/nil0j/jirafeitor/config"
	"github.com/nil0j/jirafeitor/models/responses"
	"github.com/nil0j/jirafeitor/repository"
)

// @Tags Videos
// @Param id path int true "Video ID"
// @Success 200 {file} file "Video file"
// @Success 400 {object} responses.jsonError
// @Router /video/source/{id} [get]
func GetVideo(c *gin.Context) {
	folderPath := config.Data.Filesystem + c.Param("id")
	files, err := os.ReadDir(folderPath)
	if err != nil {
		responses.HandleError(c, err)
		return
	}

	if len(files) == 0 {
		responses.HandleError(c, err)
		return
	}

	var targetFileName string
	if filepath.Ext(files[0].Name()) == ".png" {
		targetFileName = files[1].Name()
	} else {
		targetFileName = files[0].Name()
	}

	_ = filepath.Join(folderPath, targetFileName)
	c.Redirect(http.StatusTemporaryRedirect, "https://giraffe.niliara.net/api/video/realsource/"+c.Param("id")+"/"+targetFileName)
	// c.File(filePath)
}

// @Tags Videos
// @Param id path int true "Video ID"
// @Success 200 {file} file "Image file"
// @Success 400 {object} responses.jsonError
// @Router /video/thumbnail/{id} [get]
func GetVideoThumbnail(c *gin.Context) {
	folderPath := config.Data.Filesystem + c.Param("id")
	files, err := os.ReadDir(folderPath)
	if err != nil {
		responses.HandleError(c, err)
		return
	}

	if len(files) == 0 {
		responses.HandleError(c, err)
		return
	}

	filePath := filepath.Join(folderPath, "thumbnail.png")
	c.File(filePath)
}

// @Tags Videos
// @Param id path int true "Video ID"
// @Success 200 {object} postgres.Video
// @Success 400 {object} responses.jsonError
// @Router /video/info/{id} [get]
func GetVideoInfo(c *gin.Context) {
	id, err := strconv.Atoi(c.Param("id"))
	if err != nil {
		responses.HandleError(c, err)
		return
	}

	videoInfo, err := repository.GetVideoInfo(id)
	if err != nil {
		responses.HandleError(c, err)
		return
	}

	responses.HandleVideoInfo(c, videoInfo)
}

// @Tags Videos
// @Success 200 {object} []postgres.Video
// @Success 400 {object} responses.jsonError
// @Router /videos [get]
func GetRecentVideos(c *gin.Context) {
	videos, err := repository.GetRecentVideos()
	if err != nil {
		responses.HandleError(c, err)
		return
	}

	c.JSON(200, videos)
}

// @Tags Videos
// @Param term query string false "Search term"
// @Param limit query string false "Search results limit amount"
// @Success 200 {object} []postgres.Video
// @Success 400 {object} responses.jsonError
// @Router /videos/search [get]
func Search(c *gin.Context) {
	term := c.Query("term")

	var limit int
	{
		limitInput := c.Query("limit")

		var err error
		limit, err = strconv.Atoi(limitInput)
		if err != nil {
			limit = 10 // default limit
		}
	}

	videos, err := repository.Search(term, limit)
	if err != nil {
		responses.HandleError(c, err)
		return
	}

	c.JSON(200, videos)
}

package responses

import (
	"fmt"

	"github.com/gin-gonic/gin"
	"github.com/nil0j/jirafeitor/models/postgres"
)

type jsonError struct {
	Error string `json:"error"`
}

type jsonSuccess struct {
	Message string `json:"message"`
}

type jsonJWTSuccess struct {
	Message string `json:"message"`
	Token   string `json:"token"`
}

func HandleError(c *gin.Context, err error) {
	c.JSON(400, jsonError{Error: fmt.Sprintf("%v", err)})
}

func HandleSuccess(c *gin.Context, message string) {
	c.JSON(200, jsonSuccess{Message: message})
}

func HandleJWTLogin(c *gin.Context, username, token string) {
	c.JSON(200, jsonJWTSuccess{
		Message: fmt.Sprintf("Successfully logged in as %s!", username),
		Token:   token,
	})
}

func HandleVideoInfo(c *gin.Context, videoInfo postgres.Video) {
	c.JSON(200, videoInfo)
}

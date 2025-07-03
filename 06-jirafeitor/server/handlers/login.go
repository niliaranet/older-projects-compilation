package handlers

import (
	"encoding/json"
	"io"

	"github.com/gin-gonic/gin"
	"github.com/nil0j/jirafeitor/models/errorresponses"
	"github.com/nil0j/jirafeitor/models/postgres"
	"github.com/nil0j/jirafeitor/models/responses"
	"github.com/nil0j/jirafeitor/repository"
)

// @Tags Login
// @Param request body postgres.PostgresUserPost true "JSON request body"
// @Success 200 {object} responses.jsonJWTSuccess
// @Success 400 {object} responses.jsonError
// @Router /register [post]
func Register(c *gin.Context) {
	jsonData, err := io.ReadAll(c.Request.Body)
	if err != nil {
		responses.HandleError(c, err)
		return
	}

	user := postgres.PostgresUserPost{}
	if err := json.Unmarshal(jsonData, &user); err != nil {
		responses.HandleError(c, err)
		return
	}

	token, err := repository.CreateUser(user)
	if err != nil {
		responses.HandleError(c, err)
		return
	}

	responses.HandleJWTLogin(c, user.Username, token)
}

// @Tags Login
// @Param request body postgres.PostgresUserPost true "JSON request body"
// @Success 200 {object} responses.jsonJWTSuccess
// @Success 400 {object} responses.jsonError
// @Router /login [post]
func Login(c *gin.Context) {
	jsonData, err := io.ReadAll(c.Request.Body)
	if err != nil {
		responses.HandleError(c, err)
		return
	}

	user := postgres.PostgresUserPost{}
	if err := json.Unmarshal(jsonData, &user); err != nil {
		responses.HandleError(c, err)
		return
	}

	token, err := repository.LogIn(user)
	if err != nil {
		responses.HandleError(c, err)
		return
	}

	responses.HandleJWTLogin(c, user.Username, token)
}

// @Tags Login
// @Success 200 {object} postgres.PostgresUser
// @Success 400 {object} responses.jsonError
// @Security JWT
// @Router /user [get]
func GetUser(c *gin.Context) {
	inputId, _ := c.Get("UserID")
	id := -1

	switch v := inputId.(type) {
	case int:
		id = v
	}

	if id == -1 {
		responses.HandleError(c, errorresponses.UserNotFound)
		return
	}

	user, err := repository.GetUser(id)
	if err != nil {
		responses.HandleError(c, err)
		return
	}

	c.JSON(200, user)
}

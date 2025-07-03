package middlewares

import (
	"log"
	"strings"

	"github.com/gin-gonic/gin"
	"github.com/nil0j/jirafeitor/models/errorresponses"
	"github.com/nil0j/jirafeitor/models/responses"
	"github.com/nil0j/jirafeitor/utils/auth"
)

func JWT() gin.HandlerFunc {
	return func(c *gin.Context) {
		authData := strings.Split(c.GetHeader("Authorization"), "Bearer ")
		if len(authData) < 2 {
			responses.HandleError(c, errorresponses.JWTMissing)
			c.Abort()
			return
		}

		tokenString := authData[1]
		auth.TokenIsValid(tokenString)
		c.Set("UserID", auth.GetTokenUserID(tokenString))
		log.Println(auth.GetTokenUserID(tokenString))
		c.Next()
	}
}

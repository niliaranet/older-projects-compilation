package middlewares

import (
	"os"

	"github.com/gin-gonic/gin"
	"github.com/nil0j/jirafeitor/models/errorresponses"
	"github.com/nil0j/jirafeitor/models/responses"
)

func Ultrapass() gin.HandlerFunc {
	return func(c *gin.Context) {
		if os.Getenv("ULTRAPASS") == "" {
			c.Next()
		}

		ultrapass := c.GetHeader("Ultrapass")
		if ultrapass != os.Getenv("ULTRAPASS") {
			responses.HandleError(c, errorresponses.NoAccess)
			c.Abort()
			return
		}

		c.Next()
	}
}

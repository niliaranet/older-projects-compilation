package routes

import (
	"github.com/nil0j/jirafeitor/handlers"
	"github.com/nil0j/jirafeitor/middlewares"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	swaggerFiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
)

func Run() {
	router := gin.Default()
	config := cors.DefaultConfig()
	config.AllowOrigins = []string{"*"}
	config.AllowMethods = []string{"POST", "OPTIONS"}
	config.AllowHeaders = []string{"Origin", "Content-Type", "Accept", "Authorization"}
	router.Use(cors.New(config))

	router.GET("/api/docs/*any", ginSwagger.WrapHandler(swaggerFiles.Handler))

	baseGroup := router.Group("api")

	baseGroup.POST("login", handlers.Login)
	baseGroup.POST("register", middlewares.Ultrapass(), handlers.Register)
	baseGroup.GET("user", middlewares.JWT(), handlers.GetUser)

	baseGroup.POST("upload", middlewares.JWT(), handlers.UploadVideo)

	baseGroup.GET("videos", handlers.GetRecentVideos)
	baseGroup.GET("videos/search", handlers.Search)
	{
		videoGroup := baseGroup.Group("video")
		videoGroup.GET("source/:id", handlers.GetVideo)
		videoGroup.GET("info/:id", handlers.GetVideoInfo)
		videoGroup.GET("thumb/:id", handlers.GetVideoThumbnail)
	}

	router.SetTrustedProxies(nil)
	gin.SetMode(gin.ReleaseMode)
	router.Run("localhost:8080")
}

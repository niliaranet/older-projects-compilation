package main

import (
	"log"

	"github.com/nil0j/jirafeitor/config"
	_ "github.com/nil0j/jirafeitor/docs" // Add this line
	"github.com/nil0j/jirafeitor/repository"
	"github.com/nil0j/jirafeitor/routes"
)

// @title Jirafeitor API
// @version	1.0
// @description A video service made using gin-gonic

// @securityDefinitions.apikey JWT
// @in header
// @name Authorization

// @host giraffe.niliara.net
// @BasePath /api
func main() {
	if err := config.Setup(); err != nil {
		log.Panic(err)
	}

	err := repository.Setup()
	if err != nil {
		log.Panic(err)
	}

	routes.Run()
}

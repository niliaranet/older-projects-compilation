package config

import (
	"errors"
	"log"
	"os"

	"github.com/joho/godotenv"
)

var Data = appData{}

type appData struct {
	Filesystem string
	Static     string
	Db_url     string
	Secret     string
}

func Setup() error {
	err := godotenv.Load()
	if err != nil {
		log.Println("Env file not found, skipping...")
	}

	if os.Getenv("DB_URL") == "" {
		return errors.New("Environment variable DB_URL not set")
	}

	if os.Getenv("SECRET") == "" {
		return errors.New("Environment variable SECRET not set")
	}

	if os.Getenv("JIRAFEITOR_ROOT") == "" {
		return errors.New("Environment variable FILESYSTEM not set")
	}

	filesystemPath := os.Getenv("JIRAFEITOR_ROOT") + "filesystem/"
	staticPath := os.Getenv("JIRAFEITOR_ROOT") + "static/"

	os.MkdirAll(filesystemPath, 0755)
	os.MkdirAll(staticPath, 0755)

	Data = appData{
		Filesystem: filesystemPath,
		Static:     staticPath,
		Db_url:     os.Getenv("DB_URL"),
		Secret:     os.Getenv("SECRET"),
	}

	return nil
}

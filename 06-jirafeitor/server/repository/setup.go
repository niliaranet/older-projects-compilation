package repository

import (
	"context"
	"log"

	"github.com/jackc/pgx/v5"
	"github.com/nil0j/jirafeitor/config"
)

var conn *pgx.Conn

func Setup() error {
	log.Println("Setting up database")

	var err error
	conn, err = pgx.Connect(context.Background(), config.Data.Db_url)
	if err != nil {
		return err
	}

	createInitialTables()
	return nil
}

func createInitialTables() {
	_, err := conn.Exec(context.Background(), `
		CREATE SCHEMA IF NOT EXISTS jirafeitor;
	`)
	if err != nil {
		log.Fatal(err)
	}

	_, err = conn.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS jirafeitor.users (
			id SERIAL PRIMARY KEY,
			password VARCHAR(255) NOT NULL,
			name VARCHAR(255) NOT NULL,
			created_at TIMESTAMP NOT NULL DEFAULT NOW()
		);
	`)
	if err != nil {
		log.Fatal(err)
	}

	_, err = conn.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS jirafeitor.videos (
			id SERIAL PRIMARY KEY,
			title VARCHAR(255) NOT NULL,
			description TEXT NOT NULL,
			user_id INTEGER REFERENCES jirafeitor.users(id) NOT NULL,
			created_at TIMESTAMP NOT NULL DEFAULT NOW()
		);
	`)
	if err != nil {
		log.Fatal(err)
	}

	initialPopulate()
}

func initialPopulate() {
	_, err := conn.Exec(context.Background(), `
		INSERT INTO jirafeitor.users (password, name) VALUES ('password', 'Admin');
	`)
	if err != nil {
		log.Fatal(err)
	}

	_, err = conn.Exec(context.Background(), `
		INSERT INTO jirafeitor.videos (title, description, user_id) VALUES ('Video 1', 'Description 1', 1);
	`)
	if err != nil {
		log.Fatal(err)
	}
}

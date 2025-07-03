package repository

import (
	"context"
	"log"

	"github.com/nil0j/jirafeitor/models/errorresponses"
	"github.com/nil0j/jirafeitor/models/postgres"
	"github.com/nil0j/jirafeitor/utils/auth"
)

func CreateUser(user postgres.PostgresUserPost) (string, error) {
	var tmpUsername string
	err := conn.QueryRow(context.Background(), "SELECT username FROM users WHERE username = $1", user.Username).Scan(&tmpUsername)
	if err == nil {
		return "", errorresponses.UserAlreadyExists
	}

	passwordHash, err := auth.Hash(user.Password)
	if err != nil {
		return "", err
	}

	var id int
	query := "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id"
	if err := conn.QueryRow(context.Background(), query, user.Username, passwordHash).Scan(&id); err != nil {
		return "", err
	}

	token, err := auth.GenerateToken(id)
	if err != nil {
		return "", err
	}

	return token, nil
}

func LogIn(user postgres.PostgresUserPost) (string, error) {
	var response postgres.PostgresUser
	log.Println(user)
	err := conn.QueryRow(context.Background(), "SELECT id, username, password FROM users WHERE username = $1", user.Username).Scan(&response.ID, &response.Username, &response.Password)
	if err != nil {
		return "", err
	}

	if auth.CheckPasswordHash(user.Password, response.Password) {
		token, err := auth.GenerateToken(response.ID)
		if err != nil {
			return "", err
		}

		return token, nil
	}

	return "", errorresponses.PasswordDontMatch
}

func GetUser(id int) (postgres.PostgresUserInfo, error) {
	var user postgres.PostgresUserInfo
	log.Println(id)
	err := conn.QueryRow(context.Background(), "SELECT id, username FROM users WHERE id = $1", id).Scan(&user.ID, &user.Username)
	if err != nil {
		return postgres.PostgresUserInfo{}, err
	}

	return user, nil
}

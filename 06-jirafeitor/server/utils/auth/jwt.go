package auth

import (
	"time"

	"github.com/golang-jwt/jwt"
	"github.com/nil0j/jirafeitor/config"
)

type Claims struct {
	jwt.StandardClaims
	UserID int
}

func GenerateToken(id int) (string, error) {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, Claims{
		StandardClaims: jwt.StandardClaims{
			ExpiresAt: int64(500 * time.Hour),
		},
		UserID: id,
	})

	tokenString, err := token.SignedString([]byte(config.Data.Secret))
	if err != nil {
		return "", err
	}

	return tokenString, nil
}

func GetTokenUserID(tokenString string) int {
	claims := &Claims{}
	jwt.ParseWithClaims(tokenString, claims, func(token *jwt.Token) (any, error) {
		return []byte(config.Data.Secret), nil
	})

	return claims.UserID
}

func TokenIsValid(tokenString string) bool {
	token, err := jwt.Parse(tokenString, func(token *jwt.Token) (any, error) {
		return []byte(config.Data.Secret), nil
	})
	if err != nil {
		return false
	}

	return token.Valid
}

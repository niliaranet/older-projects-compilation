package postgres

type PostgresUser struct {
	ID       int
	Username string
	Password string
}

type PostgresUserPost struct {
	Username string
	Password string
}

type PostgresUserInfo struct {
	ID       int
	Username string
}

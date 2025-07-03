package errorresponses

import "errors"

var UserNotFound = errors.New("User not found")
var UserAlreadyExists = errors.New("This username is already taken")
var PasswordDontMatch = errors.New("Passwords don't match")
var JWTMissing = errors.New("No JWT token provided")
var NoAccess = errors.New("You do not have access.")

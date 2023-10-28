package database

import (
	"database/sql"
	"os"

	_ "modernc.org/sqlite"
)

var dbConn *sql.DB

func ConnectToDB() error {
	if _, err := os.Stat("authenticator.sqlite"); err != nil {
		if _, err := os.Create("authenticator.sqlite"); err != nil {
			return err
		}
	}
	db, err := sql.Open("sqlite", "authenticator.sqlite")
	if err != nil {
		return err
	}
	dbConn = db
	if err := initTable(); err != nil {
		return err
	}
	return nil
}

func initTable() error {
	query := `CREATE TABLE IF NOT EXISTS "accounts" (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		username TEXT NOT NULL DEFAULT '',
		issuer text NOT NULL DEFAULT '',
		secret text NOT NULL DEFAULT '',
		"type" text NOT NULL DEFAULT '',
		hash_function text NOT NULL DEFAULT '',
		period integer NOT NULL DEFAULT 30,
		digits integer NOT NULL DEFAULT 6,
		usage_count integer NOT NULL DEFAULT 0,
		note text NOT NULL DEFAULT ''
	)`
	if _, err := dbConn.Exec(query); err != nil {
		return err
	}
	return nil
}

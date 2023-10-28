package database

import (
	"errors"

	"github.com/blockloop/scan/v2"
)

type Account struct {
	ID           int    `db:"id" json:"id"`
	Username     string `db:"username" json:"username"`
	Issuer       string `db:"issuer" json:"issuer"`
	Secret       string `db:"secret" json:"secret"`
	Type         string `db:"type" json:"type"`
	HashFunction string `db:"hash_function" json:"hash_function"`
	Period       int    `db:"period" json:"period"`
	Digits       int    `db:"digits" json:"digits"`
	UsageCount   int    `db:"usage_count" json:"usage_count"`
	Note         string `db:"note" json:"note"`
}

func GetAccounts() ([]Account, error) {
	if dbConn == nil {
		return nil, errors.New("db not initialized")
	}
	rows, err := dbConn.Query(`select * from accounts`)
	if err != nil {
		return nil, err
	}
	var accs []Account
	if err := scan.Rows(&accs, rows); err != nil {
		return nil, err
	}
	return accs, nil
}

func InsertAccount(acc Account) error {
	if dbConn == nil {
		return errors.New("db not initialized")
	}
	query := `
	insert into accounts (username, issuer, secret, type, hash_function, period, digits, note)
	values (?, ?, ?, ?, ?, ?, ?, ?);
	`
	_, err := dbConn.Exec(
		query,
		acc.Username,
		acc.Issuer,
		acc.Secret,
		acc.Type,
		acc.HashFunction,
		acc.Period,
		acc.Digits,
		acc.Note,
	)
	return err
}

func UpdateAccount(acc Account) error {
	if dbConn == nil {
		return errors.New("db not initialized")
	}
	query := `
		update accounts set username=?, issuer=?, secret=?, type=?, hash_function=?, period=?, digits=?, note=?
		where id=?;
	`
	_, err := dbConn.Exec(
		query,
		acc.Username,
		acc.Issuer,
		acc.Secret,
		acc.Type,
		acc.HashFunction,
		acc.Period,
		acc.Digits,
		acc.Note,
		acc.ID,
	)
	return err
}

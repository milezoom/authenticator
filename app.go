package main

import (
	"changeme/database"
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/pquerna/otp"
	"github.com/pquerna/otp/totp"
	"github.com/wailsapp/wails/v2/pkg/runtime"
)

type App struct {
	ctx context.Context
}

func NewApp() *App {
	return &App{}
}

func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
	if err := database.ConnectToDB(); err != nil {
		runtime.LogErrorf(ctx, "error when connecting to DB: %s", err.Error())
	}
}

type accDataForList struct {
	ID       int    `json:"id"`
	Issuer   string `json:"issuer"`
	Username string `json:"username"`
	Code     string `json:"code"`
}

func (a *App) GetAccounts() string {
	acc, err := database.GetAccounts()
	if err != nil {
		runtime.LogErrorf(a.ctx, "error happened: %s", err.Error())
		return ""
	}
	accs := []accDataForList{}
	for _, v := range acc {
		code, err := totp.GenerateCodeCustom(
			v.Secret,
			time.Now(),
			totp.ValidateOpts{
				Period:    uint(v.Period),
				Skew:      1,
				Digits:    otp.DigitsSix,     // TODO: adjust based on db entry
				Algorithm: otp.AlgorithmSHA1, // TODO: adjust based on db entry
			},
		)
		if err != nil {
			runtime.LogErrorf(a.ctx, "error happened: %s", err.Error())
			code = ""
		}
		accs = append(accs, accDataForList{
			ID:       v.ID,
			Issuer:   v.Issuer,
			Username: v.Username,
			Code:     code,
		})
	}
	bytes, err := json.Marshal(accs)
	if err != nil {
		runtime.LogErrorf(a.ctx, "error happened: %s", err.Error())
		return ""
	}
	return string(bytes)
}

func (a *App) UpsertAccount(jsonAcc string) string {
	var acc database.Account
	if err := json.Unmarshal([]byte(jsonAcc), &acc); err != nil {
		return fmt.Sprintf("ERROR: %s", err.Error())
	}
	if acc.ID != 0 {
		if err := database.UpdateAccount(acc); err != nil {
			return fmt.Sprintf("ERROR: %s", err.Error())
		}
	} else {
		if err := database.InsertAccount(acc); err != nil {
			return fmt.Sprintf("ERROR: %s", err.Error())
		}
	}
	return "OK"
}

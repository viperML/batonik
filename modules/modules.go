package modules

import "os"

type Directory struct {}

func (d *Directory) Run() string {
	cwd, err := os.Getwd()
	if err != nil {
		return "Error getting directory"
	}
	return cwd
}
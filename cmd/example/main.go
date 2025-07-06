package main

import (
	"github.com/viperML/batonik"
	. "github.com/viperML/batonik/modules"
)

func main() {

	app := &batonik.App{
		Separator:  " ",
		AddNewline: true,
		Modules: []batonik.Module{
			&Directory{},
			&Env{
				Variable: "SHELL",
			},
		},
	}

	app.Run()

}

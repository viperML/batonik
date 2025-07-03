package main

import (
	"github.com/viperML/batonik"
	"github.com/viperML/batonik/colors"
	. "github.com/viperML/batonik/modules"
)

func main() {

	app := &batonik.App{
		Separator:  " ",
		AddNewline: true,
		Modules: []batonik.Module{
			S(colors.Red + "Hello" + colors.Reset),
			S("Hello"),
			S("World"),
			&Directory{},
		},
	}

	app.Run()

}

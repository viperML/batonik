package main

import (
	"github.com/viperML/batonik"
	. "github.com/viperML/batonik/modules"
)

func main() {
	app := batonik.App().AddModules(&Directory{}, &Git{}, &Env{
		Variable: "SHELL",
	}).AddModules(&Character{})

	app.Run()
}

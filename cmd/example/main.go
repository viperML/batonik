package main

import (
	"fmt"

	"github.com/viperML/batonik"
	. "github.com/viperML/batonik/modules"
)

func main() {
	app := batonik.App().AddModules(&Directory{}, &Env{
		Variable: "SHELL",
	}).AddModules(&Character{})

	fmt.Println(app.Run())
}

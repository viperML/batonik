package modules

import "github.com/viperML/batonik/colors"

type Character struct {
	SuccessSymbol string
	ErrorSymbol   string
}

func (mod *Character) Run() string {
	if mod.SuccessSymbol == "" {
		mod.SuccessSymbol = colors.Bold + colors.Green + "➜ " + colors.Reset
	}
	if mod.ErrorSymbol == "" {
		mod.ErrorSymbol = colors.Bold + colors.Red + "✖ " + colors.Reset
	}

	// TODO: Get exit status

	exit := 0

	if exit == 0 {
		return mod.SuccessSymbol
	} else {
		return mod.ErrorSymbol
	}
}

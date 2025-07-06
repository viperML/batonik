package modules

import (
	"os"

	"github.com/viperML/batonik/colors"
)

type Env struct {
	Variable string
	Style    func(*Env, string) string
}

func defaultEnvStyle(mod *Env, res string) string {
	return "with " + colors.DimWhite + res + colors.Reset
}

func (mod *Env) Run() string {
	if mod.Style == nil {
		mod.Style = defaultEnvStyle
	}

	res, found := os.LookupEnv(mod.Variable)

	if found {
		return mod.Style(mod, res)
	} else {
		return ""
	}
}

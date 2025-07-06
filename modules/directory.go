package modules

import (
	"os"
	"strings"

	"github.com/viperML/batonik/colors"
)

type Directory struct {
	Style func(string) string
}

func defaultDirectoryStyle(res string) string {
	return colors.Bold + colors.BrightCyan + res + colors.Reset
}

func (mod *Directory) Run() string {
	if mod.Style == nil {
		mod.Style = defaultDirectoryStyle
	}

	home, err := os.UserHomeDir()
	if err != nil {
		return "Error getting home directory"
	}

	cwd, err := os.Getwd()
	if err != nil {
		return "Error getting directory"
	}

	// Replace home directory with ~
	cwd = strings.Replace(cwd, home, "~", 1)

	return mod.Style(cwd)
}

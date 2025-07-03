package modules

import (
	"os"
	"strings"

	"github.com/viperML/batonik/colors"
)

type Directory struct {
	Style func(string) string
}

func defaultStyle(text string) string {
	return colors.Blue + text + colors.Reset
}

func (d *Directory) Run() string {
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

	if d.Style == nil {
		d.Style = defaultStyle
	}

	return d.Style(cwd)
}
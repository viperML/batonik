package modules

import (
	"github.com/go-git/go-git/v5"
	"github.com/viperML/batonik/colors"
)

type Git struct {
	Style func(string) string
}

func defaultGitStyle(branch string) string {
	return "on " + colors.Bold + colors.BrightMagenta + " " + branch + colors.Reset
}

func (g *Git) Run() string {
	repo, err := git.PlainOpen(".")
	if err != nil {
		return ""
	}
	ref, err := repo.Head()
	if err != nil {
		return ""
	}
	if !ref.Name().IsBranch() {
		return ""
	}
	branch := ref.Name().Short()
	if g.Style == nil {
		g.Style = defaultGitStyle
	}
	return g.Style(branch)
}
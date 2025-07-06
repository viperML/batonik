package batonik

import "fmt"

type App struct {
	Separator  string
	AddNewline bool
	Modules    []Module
}

type Module interface {
	Run() string
}

func (a *App) Run() {

	results := make(map[int]string)

	for i, module := range a.Modules {
		results[i] = module.Run()
	}

	for i := range a.Modules {
		fmt.Print(results[i])
		if i < len(a.Modules)-1 {
			fmt.Print(a.Separator)
		}
	}
}

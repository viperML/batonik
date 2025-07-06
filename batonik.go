package batonik

import (
	"fmt"
	"strings"
	"sync"
)

type Module interface {
	Run() string
}

type app struct {
	autoSeparate bool
	modules      [][]Module
}

func App() *app {
	return &app{
		autoSeparate: true,
		modules:      make([][]Module, 0),
	}
}

func (a *app) AddModules(modules ...Module) *app {
	if len(modules) > 0 {
		a.modules = append(a.modules, modules)
	}

	return a
}

func (a *app) Run() {
	groupResults := make([]string, len(a.modules))
	var groupWg sync.WaitGroup

	for gi, group := range a.modules {
		groupWg.Add(1)
		go func(gi int, group []Module) {
			defer groupWg.Done()
			results := make([]string, len(group))
			var wg sync.WaitGroup
			for i, mod := range group {
				wg.Add(1)
				go func(i int, m Module) {
					defer wg.Done()
					results[i] = m.Run()
				}(i, mod)
			}
			wg.Wait()
			groupResults[gi] = strings.Join(results, " ")
		}(gi, group)
	}
	groupWg.Wait()
	res := strings.Join(groupResults, "\n")
	fmt.Println(res)
}

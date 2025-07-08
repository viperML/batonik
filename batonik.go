package batonik

import (
	"context"
	"fmt"
	"os"
	"strings"
	"sync"

	"github.com/urfave/cli/v3"
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

func (a *app) Render() string {
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
	return res
}

func (a *app) Run() {
	var shell string

	(&cli.Command{
		Name:  "batonik",
		Usage: "The programmable shell prompt",
		Commands: []*cli.Command{
			{
				Name:  "init",
				Usage: "prints the shell function used to execute batonik",
				Arguments: []cli.Argument{
					&cli.StringArg{
						Name: "shell",
						Destination: &shell,
					},
				},
				Action: func(ctx context.Context, c *cli.Command) error {
					err := init_shell(shell)

					if err != nil {
						return cli.Exit(err, 1)
					}

					return nil
				},
			},
			{
				Name:  "prompt",
				Usage: "prints the full prompt",
				Action: func(ctx context.Context, cmd *cli.Command) error {
					fmt.Println(a.Render())
					return nil
				},
			},
		},
	}).Run(context.Background(), os.Args)
}

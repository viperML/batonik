package batonik

import "sync"

type Module interface {
	Render() string
}

type Renderer struct {
	Modules []Module
}


func Render(renderer *Renderer) string {
	results := make([]string, len(renderer.Modules))
	var wg sync.WaitGroup

	for i, module := range renderer.Modules {
		wg.Add(1)
		go func(index int, mod Module) {
			defer wg.Done()
			results[index] = mod.Render()
		}(i, module)
	}

	wg.Wait()

	result := ""
	for _, res := range results {
		result += res
	}

	return result
}
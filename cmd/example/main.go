package main

import (
	"fmt"

	"github.com/viperML/batonik/batonik"
)

type SampleModule struct{}

func NewSampleModule() *SampleModule {
	return &SampleModule{}
}

func (sm *SampleModule) Render() string {
	return "Hello"
}

func main() {
	// fmt.Println("Hello world!")

	renderer := &batonik.Renderer{
		Modules: []batonik.Module{},
	}

	renderer.Modules = append(renderer.Modules, NewSampleModule())
	fmt.Printf("%#v\n", renderer)

	for _, module := range renderer.Modules {
		result := module.Render()
		fmt.Println(result)
	}
}

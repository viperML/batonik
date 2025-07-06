package modules

import "github.com/viperML/batonik"

type WhenMod struct {
	inner     batonik.Module
	condition func() bool
}

func When(mod batonik.Module, condition func() bool) WhenMod {
	return WhenMod{
		inner:     mod,
		condition: condition,
	}
}

func (mod *WhenMod) Run() string {
	res := mod.condition()
	if mod.inner == nil || !res {
		return ""
	} else {
		return mod.inner.Run()
	}
}

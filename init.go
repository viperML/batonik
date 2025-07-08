package batonik

import (
	_ "embed"
	"fmt"
	"os"
	"path"
)

func init_shell(shell string) error {

	arg0 := os.Args[0]
	cmd := path.Clean(arg0)

	fmt.Println(cmd)

	switch shell {
	case "bash":
		// handle bash init
		// (placeholder: print or return nil)
		init_bash()
		return nil
	case "zsh":
		// handle zsh init
		return nil
	case "fish":
		// handle fish init
		return nil
	case "":
		return fmt.Errorf("pass one of bash, zsh or fish to batonik init")
	default:
		return fmt.Errorf("batonik init accepts the following shells: bash, zsh, fish. \"%s\" is not supported", shell)
	}
}

func init_bash() {}
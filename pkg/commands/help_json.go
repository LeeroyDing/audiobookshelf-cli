package commands

import (
	"encoding/json"
	"fmt"

	"github.com/spf13/cobra"
	"github.com/spf13/pflag"
)

type CommandInfo struct {
	Name        string        `json:"name"`
	Use         string        `json:"use"`
	Short       string        `json:"short"`
	Long        string        `json:"long,omitempty"`
	Example     string        `json:"example,omitempty"`
	Aliases     []string      `json:"aliases,omitempty"`
	Flags       []FlagInfo    `json:"flags,omitempty"`
	GlobalFlags []FlagInfo    `json:"globalFlags,omitempty"`
	Commands    []CommandInfo `json:"commands,omitempty"`
}

type FlagInfo struct {
	Name      string `json:"name"`
	Shorthand string `json:"shorthand,omitempty"`
	Usage     string `json:"usage"`
	Type      string `json:"type"`
	Default   string `json:"default,omitempty"`
}

func getFlagInfo(flagSet *pflag.FlagSet) []FlagInfo {
	var flags []FlagInfo
	if flagSet == nil {
		return flags
	}
	flagSet.VisitAll(func(f *pflag.Flag) {
		flags = append(flags, FlagInfo{
			Name:      f.Name,
			Shorthand: f.Shorthand,
			Usage:     f.Usage,
			Type:      f.Value.Type(),
			Default:   f.DefValue,
		})
	})
	return flags
}

func buildCommandInfo(c *cobra.Command) CommandInfo {
	info := CommandInfo{
		Name:    c.Name(),
		Use:     c.UseLine(),
		Short:   c.Short,
		Long:    c.Long,
		Example: c.Example,
		Aliases: c.Aliases,
	}

	info.Flags = getFlagInfo(c.NonInheritedFlags())
	info.GlobalFlags = getFlagInfo(c.InheritedFlags())

	for _, child := range c.Commands() {
		if child.IsAvailableCommand() || child.Name() == "help" {
			info.Commands = append(info.Commands, buildCommandInfo(child))
		}
	}

	return info
}

func printJSONHelp(c *cobra.Command) error {
	info := buildCommandInfo(c)
	b, err := json.MarshalIndent(info, "", "  ")
	if err != nil {
		return err
	}
	fmt.Fprintln(c.OutOrStdout(), string(b))
	return nil
}

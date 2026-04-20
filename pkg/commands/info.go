package commands

import (
	"github.com/LeeroyDing/audiobookshelf-cli/pkg/utils"
	"github.com/spf13/cobra"
)

var (
	pingCmd = &cobra.Command{
		Use:   "ping",
		Short: "Check server connectivity",
		RunE: func(cmd *cobra.Command, args []string) error {
			client, err := getClient()
			if err != nil {
				return err
			}
			println("Pinging server...")
			if err := client.Ping(); err != nil {
				return err
			}
			println("Server is reachable!")
			return nil
		},
	}

	meCmd = &cobra.Command{
		Use:   "me",
		Short: "Get current user info",
		RunE: func(cmd *cobra.Command, args []string) error {
			client, err := getClient()
			if err != nil {
				return err
			}
			if err := ensureAuth(client); err != nil {
				return err
			}
			me, err := client.GetMe()
			if err != nil {
				return err
			}
			utils.PrintJSON(me)
			return nil
		},
	}

	infoCmd = &cobra.Command{
		Use:   "info",
		Short: "Get server status info",
		RunE: func(cmd *cobra.Command, args []string) error {
			client, err := getClient()
			if err != nil {
				return err
			}
			if err := ensureAuth(client); err != nil {
				return err
			}
			status, err := client.GetStatus()
			if err != nil {
				return err
			}

			if jsonOutput {
				utils.PrintJSON(status)
				return nil
			}

			data := status.(map[string]interface{})
			table := utils.NewTable([]string{"Property", "Value"})
			utils.AppendRow(table, []string{"Initialized", utils.FormatBool(data["isInit"])})
			utils.AppendRow(table, []string{"Default Language", utils.GetString(data, "defaultLanguage")})
			utils.AppendRow(table, []string{"Config Path", utils.GetString(data, "configPath")})
			utils.AppendRow(table, []string{"Metadata Path", utils.GetString(data, "metadataPath")})
			utils.RenderTable(table)
			return nil
		},
	}
)

func init() {
	rootCmd.AddCommand(pingCmd, meCmd, infoCmd)
}

package commands

import (
	"fmt"

	"github.com/LeeroyDing/audiobookshelf-cli/pkg/utils"
	"github.com/spf13/cobra"
)

var (
	forceScan bool
	libsCmd   = &cobra.Command{
		Use:   "libraries",
		Short: "Manage libraries",
	}

	libsListCmd = &cobra.Command{
		Use:   "list",
		Short: "List all libraries on the server",
		RunE: func(cmd *cobra.Command, args []string) error {
			client, err := getClient()
			if err != nil {
				return err
			}
			if err := ensureAuth(client); err != nil {
				return err
			}

			libs, err := client.GetLibraries()
			if err != nil {
				return err
			}

			if jsonOutput {
				utils.PrintJSON(libs)
				return nil
			}

			data := libs.(map[string]interface{})
			libraries := data["libraries"].([]interface{})

			table := utils.NewTable([]string{"ID", "Name", "Media Type"})
			for _, l := range libraries {
				lib := l.(map[string]interface{})
				utils.AppendRow(table, []string{
					utils.GetString(lib, "id"),
					utils.GetString(lib, "name"),
					utils.GetString(lib, "mediaType"),
				})
			}
			utils.RenderTable(table)
			return nil
		},
	}

	libsScanCmd = &cobra.Command{
		Use:   "scan <id>",
		Short: "Trigger a scan for a library",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			id := args[0]
			client, err := getClient()
			if err != nil {
				return err
			}
			if err := ensureAuth(client); err != nil {
				return err
			}

			fmt.Printf("Triggering scan for library %s (force=%v)...\n", id, forceScan)
			_, err = client.ScanLibrary(id, forceScan)
			if err != nil {
				return err
			}
			fmt.Println("Scan triggered successfully!")
			return nil
		},
	}
)

func init() {
	libsScanCmd.Flags().BoolVarP(&forceScan, "force", "f", false, "Force a full rescan")
	libsCmd.AddCommand(libsListCmd, libsScanCmd)
	rootCmd.AddCommand(libsCmd)
}

package commands

import (
	"github.com/LeeroyDing/audiobookshelf-cli/pkg/utils"
	"github.com/spf13/cobra"
)

var (
	searchCmd = &cobra.Command{
		Use:   "search <query>",
		Short: "Global search across all libraries",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			query := args[0]
			client, err := getClient()
			if err != nil {
				return err
			}
			if err := ensureAuth(client); err != nil {
				return err
			}

			results, err := client.Search(query)
			if err != nil {
				return err
			}

			if jsonOutput {
				utils.PrintJSON(results)
				return nil
			}

			data := results.(map[string][]interface{})
			table := utils.NewTable([]string{"Type", "ID", "Name/Title"})

			categories := []string{"book", "podcast", "author", "series", "collection", "playlist"}
			for _, cat := range categories {
				if items, ok := data[cat]; ok {
					for _, i := range items {
						item := i.(map[string]interface{})
						name := utils.GetString(item, "name")
						if name == "N/A" {
							name = utils.GetNestedString(item, "media", "metadata", "title")
						}

						utils.AppendRow(table, []string{
							cat,
							utils.GetString(item, "id"),
							name,
						})
					}
				}
			}
			utils.RenderTable(table)
			return nil
		},
	}
)

func init() {
	rootCmd.AddCommand(searchCmd)
}

package commands

import (
	"fmt"
	"strings"

	"github.com/LeeroyDing/audiobookshelf-cli/pkg/utils"
	"github.com/spf13/cobra"
)

var (
	titleFlag    string
	subtitleFlag string
	authorFlag   string
	narratorFlag string
	seriesFlag   string
	genresFlag   string
	tagsFlag     string
	yearFlag     int

	itemsCmd = &cobra.Command{
		Use:   "items",
		Short: "Manage library items",
	}

	itemsListCmd = &cobra.Command{
		Use:   "list <library_id>",
		Short: "List items in a library",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			libID := args[0]
			client, err := getClient()
			if err != nil {
				return err
			}
			if err := ensureAuth(client); err != nil {
				return err
			}

			itemsResp, err := client.GetLibraryItems(libID)
			if err != nil {
				return err
			}

			if jsonOutput {
				utils.PrintJSON(itemsResp)
				return nil
			}

			data := itemsResp.(map[string]interface{})
			items := data["results"].([]interface{})

			table := utils.NewTable([]string{"ID", "Title", "Media Type"})
			for _, i := range items {
				item := i.(map[string]interface{})
				title := utils.GetNestedString(item, "media", "metadata", "title")
				if title == "N/A" {
					title = utils.GetNestedString(item, "media", "metadata", "name")
				}

				utils.AppendRow(table, []string{
					utils.GetString(item, "id"),
					title,
					utils.GetString(item, "mediaType"),
				})
			}
			utils.RenderTable(table)
			return nil
		},
	}

	itemsGetCmd = &cobra.Command{
		Use:   "get <item_id>",
		Short: "Get item details",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			itemID := args[0]
			client, err := getClient()
			if err != nil {
				return err
			}
			if err := ensureAuth(client); err != nil {
				return err
			}

			item, err := client.GetItem(itemID)
			if err != nil {
				return err
			}
			utils.PrintJSON(item)
			return nil
		},
	}

	itemsUpdateCmd = &cobra.Command{
		Use:   "update <item_id>",
		Short: "Update item metadata",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			itemID := args[0]
			client, err := getClient()
			if err != nil {
				return err
			}
			if err := ensureAuth(client); err != nil {
				return err
			}

			meta := make(map[string]interface{})
			if titleFlag != "" {
				meta["title"] = titleFlag
			}
			if subtitleFlag != "" {
				meta["subtitle"] = subtitleFlag
			}
			if authorFlag != "" {
				meta["authorName"] = authorFlag
			}
			if narratorFlag != "" {
				meta["narratorName"] = narratorFlag
			}
			if seriesFlag != "" {
				meta["seriesName"] = seriesFlag
			}
			if yearFlag != 0 {
				meta["publishedYear"] = yearFlag
			}
			if genresFlag != "" {
				meta["genres"] = strings.Split(genresFlag, ",")
			}
			if tagsFlag != "" {
				meta["tags"] = strings.Split(tagsFlag, ",")
			}

			if len(meta) == 0 {
				return fmt.Errorf("no metadata fields provided for update")
			}

			fmt.Printf("Updating metadata for item %s...\n", itemID)
			result, err := client.UpdateItemMetadata(itemID, meta)
			if err != nil {
				return err
			}
			fmt.Println("Item updated successfully!")
			if jsonOutput {
				utils.PrintJSON(result)
			}
			return nil
		},
	}

	itemsMatchCmd = &cobra.Command{
		Use:   "match <item_id>",
		Short: "Quick match an item",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			itemID := args[0]
			client, err := getClient()
			if err != nil {
				return err
			}
			if err := ensureAuth(client); err != nil {
				return err
			}

			fmt.Printf("Matching item %s...\n", itemID)
			_, err = client.MatchItem(itemID)
			if err != nil {
				return err
			}
			fmt.Println("Matching triggered successfully!")
			return nil
		},
	}

	itemsUnmatchCmd = &cobra.Command{
		Use:   "unmatch <item_id>",
		Short: "Unmatch an item",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			itemID := args[0]
			client, err := getClient()
			if err != nil {
				return err
			}
			if err := ensureAuth(client); err != nil {
				return err
			}

			fmt.Printf("Unmatching item %s...\n", itemID)
			_, err = client.UnmatchItem(itemID)
			if err != nil {
				return err
			}
			fmt.Println("Item unmatched successfully!")
			return nil
		},
	}

	itemsBulkUpdateCmd = &cobra.Command{
		Use:   "bulk-update <ids>",
		Short: "Bulk update items",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			ids := strings.Split(args[0], ",")
			for i, id := range ids {
				ids[i] = strings.TrimSpace(id)
			}

			client, err := getClient()
			if err != nil {
				return err
			}
			if err := ensureAuth(client); err != nil {
				return err
			}

			meta := make(map[string]interface{})
			if titleFlag != "" {
				meta["title"] = titleFlag
			}
			if subtitleFlag != "" {
				meta["subtitle"] = subtitleFlag
			}
			if authorFlag != "" {
				meta["authorName"] = authorFlag
			}
			if narratorFlag != "" {
				meta["narratorName"] = narratorFlag
			}
			if seriesFlag != "" {
				meta["seriesName"] = seriesFlag
			}
			if yearFlag != 0 {
				meta["publishedYear"] = yearFlag
			}
			if genresFlag != "" {
				meta["genres"] = strings.Split(genresFlag, ",")
			}
			if tagsFlag != "" {
				meta["tags"] = strings.Split(tagsFlag, ",")
			}

			if len(meta) == 0 {
				return fmt.Errorf("no metadata fields provided for update")
			}

			fmt.Printf("Performing bulk update for %d items...\n", len(ids))
			result, err := client.BatchUpdateItems(ids, meta)
			if err != nil {
				return err
			}
			fmt.Println("Bulk update completed successfully!")
			if jsonOutput {
				utils.PrintJSON(result)
			}
			return nil
		},
	}
)

func init() {
	updateFlags := func(cmd *cobra.Command) {
		cmd.Flags().StringVar(&titleFlag, "title", "", "Update the title")
		cmd.Flags().StringVar(&subtitleFlag, "subtitle", "", "Update the subtitle")
		cmd.Flags().StringVar(&authorFlag, "author", "", "Update author(s)")
		cmd.Flags().StringVar(&narratorFlag, "narrator", "", "Update narrator(s)")
		cmd.Flags().StringVar(&seriesFlag, "series", "", "Update the series name")
		cmd.Flags().StringVar(&genresFlag, "genres", "", "Update genres (comma-separated)")
		cmd.Flags().StringVar(&tagsFlag, "tags", "", "Update tags (comma-separated)")
		cmd.Flags().IntVar(&yearFlag, "year", 0, "Update the published year")
	}

	updateFlags(itemsUpdateCmd)
	updateFlags(itemsBulkUpdateCmd)

	itemsCmd.AddCommand(itemsListCmd, itemsGetCmd, itemsUpdateCmd, itemsMatchCmd, itemsUnmatchCmd, itemsBulkUpdateCmd)
	rootCmd.AddCommand(itemsCmd)
}

package commands

import (
	"fmt"

	"github.com/LeeroyDing/audiobookshelf-cli/pkg/utils"
	"github.com/spf13/cobra"
)

var (
	uploadLibFlag    string
	uploadFolderFlag string
	uploadTitleFlag  string
	uploadAuthorFlag string
	uploadSeriesFlag string

	uploadCmd = &cobra.Command{
		Use:   "upload <files...>",
		Short: "Upload books to a library",
		Args:  cobra.MinimumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			files := args
			client, err := getClient()
			if err != nil {
				return err
			}
			if err := ensureAuth(client); err != nil {
				return err
			}

			if uploadLibFlag == "" {
				return fmt.Errorf("--library is required")
			}
			if uploadTitleFlag == "" {
				return fmt.Errorf("--title is required")
			}

			// If folder is omitted, we might need to resolve it (same logic as Rust but simplified for now)
			// For now, let's assume it's provided or handle it in the client
			
			var author *string
			if uploadAuthorFlag != "" {
				author = &uploadAuthorFlag
			}
			var series *string
			if uploadSeriesFlag != "" {
				series = &uploadSeriesFlag
			}

			fmt.Printf("Uploading %d files to library %s...\n", len(files), uploadLibFlag)
			result, err := client.Upload(uploadLibFlag, uploadFolderFlag, uploadTitleFlag, author, series, files)
			if err != nil {
				return err
			}
			fmt.Println("Upload completed successfully!")
			if jsonOutput {
				utils.PrintJSON(result)
			}
			return nil
		},
	}
)

func init() {
	uploadCmd.Flags().StringVarP(&uploadLibFlag, "library", "l", "", "The UUID of the library")
	uploadCmd.Flags().StringVarP(&uploadFolderFlag, "folder", "f", "", "The UUID of the folder")
	uploadCmd.Flags().StringVarP(&uploadTitleFlag, "title", "t", "", "The title of the book")
	uploadCmd.Flags().StringVarP(&uploadAuthorFlag, "author", "a", "", "The author's name")
	uploadCmd.Flags().StringVarP(&uploadSeriesFlag, "series", "s", "", "The series name")

	rootCmd.AddCommand(uploadCmd)
}

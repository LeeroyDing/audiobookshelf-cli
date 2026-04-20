package commands

import (
	"fmt"

	"github.com/spf13/cobra"
	"github.com/zalando/go-keyring"
)

var (
	apiKeyFlag string
	authCmd    = &cobra.Command{
		Use:   "auth",
		Short: "Manage authentication",
	}

	loginCmd = &cobra.Command{
		Use:   "login",
		Short: "Save your API key securely in the system keyring",
		RunE: func(cmd *cobra.Command, args []string) error {
			if apiKeyFlag == "" {
				return fmt.Errorf("api-key is required")
			}
			err := keyring.Set("audiobookshelf-cli", "api_key", apiKeyFlag)
			if err != nil {
				return fmt.Errorf("failed to save API key to keyring: %v", err)
			}
			fmt.Println("API key saved successfully to system keyring.")
			return nil
		},
	}

	logoutCmd = &cobra.Command{
		Use:   "logout",
		Short: "Remove your API key from the system keyring",
		RunE: func(cmd *cobra.Command, args []string) error {
			err := keyring.Delete("audiobookshelf-cli", "api_key")
			if err != nil {
				return fmt.Errorf("failed to remove API key from keyring: %v", err)
			}
			fmt.Println("API key removed from system keyring.")
			return nil
		},
	}
)

func init() {
	loginCmd.Flags().StringVarP(&apiKeyFlag, "api-key", "k", "", "The API key to save")
	authCmd.AddCommand(loginCmd, logoutCmd)
	rootCmd.AddCommand(authCmd)
}

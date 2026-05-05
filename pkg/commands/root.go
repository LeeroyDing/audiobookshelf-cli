package commands

import (
	"fmt"
	"os"

	"github.com/LeeroyDing/audiobookshelf-cli/pkg/client"
	"github.com/LeeroyDing/audiobookshelf-cli/pkg/config"
	"github.com/spf13/cobra"
)

var (
	Version    = "dev"
	jsonOutput bool
	rootCmd    = &cobra.Command{
		Use:     "abs",
		Version: Version,
		Short:   "Audiobookshelf CLI",
		Long:    `A powerful command-line interface for interacting with your Audiobookshelf server.`,
	}
)

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func init() {
	rootCmd.PersistentFlags().BoolVarP(&jsonOutput, "json", "j", false, "Output raw JSON instead of human-readable tables")
}

func getClient() (*client.AbsClient, error) {
	cfg, err := config.Load()
	if err != nil {
		return nil, err
	}

	if cfg.ServerURL == "" {
		return nil, fmt.Errorf("server URL not found. Set AUDIOBOOKSHELF_SERVER_URL or configure it in config.yaml")
	}

	return client.New(cfg.ServerURL, cfg.APIKey), nil
}

func ensureAuth(c *client.AbsClient) error {
	if c.APIKey == "" {
		return fmt.Errorf("API Key not found. Please run 'abs auth login --api-key <KEY>' or set AUDIOBOOKSHELF_API_KEY")
	}
	return nil
}

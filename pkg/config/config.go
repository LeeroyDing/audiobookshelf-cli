package config

import (
	"os"
	"path/filepath"

	"github.com/adrg/xdg"
	"github.com/joho/godotenv"
	"github.com/spf13/viper"
	"github.com/zalando/go-keyring"
)

type Config struct {
	ServerURL string `mapstructure:"server_url"`
	APIKey    string `mapstructure:"api_key"`
}

func Load() (*Config, error) {
	// Load .env if it exists
	_ = godotenv.Load()

	v := viper.New()

	// 1. Load from global config file
	configPath := filepath.Join(xdg.ConfigHome, "abs", "config.yaml")
	if _, err := os.Stat(configPath); err == nil {
		v.SetConfigFile(configPath)
		if err := v.ReadInConfig(); err != nil {
			return nil, err
		}
	}

	// 2. Load from environment variables
	v.SetEnvPrefix("AUDIOBOOKSHELF")
	v.AutomaticEnv()

	var cfg Config
	if err := v.Unmarshal(&cfg); err != nil {
		return nil, err
	}

	// 3. Fallbacks and overrides
	if cfg.ServerURL == "" {
		cfg.ServerURL = os.Getenv("AUDIOBOOKSHELF_SERVER_URL")
	}
	if cfg.APIKey == "" {
		cfg.APIKey = os.Getenv("AUDIOBOOKSHELF_API_KEY")
	}

	// 4. Try Keyring if still missing
	if cfg.APIKey == "" {
		if pw, err := keyring.Get("audiobookshelf-cli", "api_key"); err == nil {
			cfg.APIKey = pw
		}
	}

	return &cfg, nil
}

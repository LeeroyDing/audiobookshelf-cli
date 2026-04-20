package client

import (
	"fmt"
	"os"
	"strings"

	"github.com/go-resty/resty/v2"
)

type AbsClient struct {
	BaseURL string
	APIKey  string
	client  *resty.Client
}

func New(baseURL, apiKey string) *AbsClient {
	c := resty.New()
	baseURL = strings.TrimSuffix(baseURL, "/")

	return &AbsClient{
		BaseURL: baseURL,
		APIKey:  apiKey,
		client:  c.SetBaseURL(baseURL).SetAuthToken(apiKey),
	}
}

func (c *AbsClient) Ping() error {
	resp, err := c.client.R().Get("/ping")
	if err != nil {
		return err
	}
	if !resp.IsSuccess() {
		return fmt.Errorf("ping failed with status: %s", resp.Status())
	}
	return nil
}

func (c *AbsClient) GetLibraries() (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get("/api/libraries")
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get libraries: %s", resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetUsers() (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get("/api/users")
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get users: %s", resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetLibraryItems(libraryID string) (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get(fmt.Sprintf("/api/libraries/%s/items", libraryID))
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get items for library %s: %s", libraryID, resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetItem(itemID string) (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get(fmt.Sprintf("/api/items/%s", itemID))
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get item %s: %s", itemID, resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetMe() (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get("/api/me")
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get current user: %s", resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetAuthors() (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get("/api/authors")
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get authors: %s", resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetAuthor(id string) (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get(fmt.Sprintf("/api/authors/%s", id))
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get author %s: %s", id, resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetCollections() (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get("/api/collections")
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get collections: %s", resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetCollection(id string) (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get(fmt.Sprintf("/api/collections/%s", id))
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get collection %s: %s", id, resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetPlaylists() (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get("/api/playlists")
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get playlists: %s", resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetPlaylist(id string) (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get(fmt.Sprintf("/api/playlists/%s", id))
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get playlist %s: %s", id, resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetSeriesList() (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get("/api/series")
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get series: %s", resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetSeries(id string) (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get(fmt.Sprintf("/api/series/%s", id))
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get series %s: %s", id, resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetTags() (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get("/api/tags")
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get tags: %s", resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetGenres() (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get("/api/genres")
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get genres: %s", resp.String())
	}
	return result, nil
}

func (c *AbsClient) GetStatus() (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Get("/status")
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to get status: %s", resp.String())
	}
	return result, nil
}

func (c *AbsClient) ScanLibrary(id string, force bool) (interface{}, error) {
	req := c.client.R()
	if force {
		req.SetQueryParam("force", "true")
	}
	var result interface{}
	resp, err := req.
		SetResult(&result).
		Post(fmt.Sprintf("/api/libraries/%s/scan", id))
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to scan library %s: %s", id, resp.String())
	}
	return result, nil
}

func (c *AbsClient) Search(query string) (interface{}, error) {
	// First get libraries to search each
	libsRaw, err := c.GetLibraries()
	if err != nil {
		return nil, err
	}

	libs, ok := libsRaw.(map[string]interface{})
	if !ok {
		return nil, fmt.Errorf("unexpected response format for libraries")
	}

	libraries, ok := libs["libraries"].([]interface{})
	if !ok {
		return nil, fmt.Errorf("unexpected libraries format")
	}

	aggregated := make(map[string][]interface{})

	for _, libRaw := range libraries {
		lib, ok := libRaw.(map[string]interface{})
		if !ok {
			continue
		}
		id, _ := lib["id"].(string)
		if id == "" {
			continue
		}

		var results map[string]interface{}
		resp, err := c.client.R().
			SetQueryParam("q", query).
			SetResult(&results).
			Get(fmt.Sprintf("/api/libraries/%s/search", id))

		if err == nil && resp.IsSuccess() {
			for key, val := range results {
				if arr, ok := val.([]interface{}); ok {
					aggregated[key] = append(aggregated[key], arr...)
				}
			}
		}
	}

	return aggregated, nil
}

func (c *AbsClient) UpdateItemMetadata(id string, metadata interface{}) (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetBody(map[string]interface{}{"metadata": metadata}).
		SetResult(&result).
		Patch(fmt.Sprintf("/api/items/%s/media", id))
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to update metadata for item %s: %s", id, resp.String())
	}
	return result, nil
}

func (c *AbsClient) MatchItem(id string) (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Post(fmt.Sprintf("/api/items/%s/match", id))
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to match item %s: %s", id, resp.String())
	}
	return result, nil
}

func (c *AbsClient) UnmatchItem(id string) (interface{}, error) {
	var result interface{}
	resp, err := c.client.R().
		SetResult(&result).
		Delete(fmt.Sprintf("/api/items/%s/match", id))
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to unmatch item %s: %s", id, resp.String())
	}
	return result, nil
}

func (c *AbsClient) BatchUpdateItems(ids []string, metadata interface{}) (interface{}, error) {
	payload := make([]interface{}, len(ids))
	for i, id := range ids {
		payload[i] = map[string]interface{}{
			"id": id,
			"mediaPayload": map[string]interface{}{
				"metadata": metadata,
			},
		}
	}

	var result interface{}
	resp, err := c.client.R().
		SetBody(payload).
		SetResult(&result).
		Post("/api/items/batch/update")
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to batch update items: %s", resp.String())
	}
	return result, nil
}

func (c *AbsClient) Upload(libraryID, folderID, title string, author, series *string, files []string) (interface{}, error) {
	req := c.client.R().
		SetFormData(map[string]string{
			"library": libraryID,
			"folder":  folderID,
			"title":   title,
		})

	if author != nil {
		req.SetFormData(map[string]string{"author": *author})
	}
	if series != nil {
		req.SetFormData(map[string]string{"series": *series})
	}

	for i, path := range files {
		file, err := os.Open(path)
		if err != nil {
			return nil, err
		}
		defer file.Close()
		req.SetFileReader(fmt.Sprintf("file_%d", i), path, file)
	}

	var result interface{}
	resp, err := req.
		SetResult(&result).
		Post("/api/upload")
	if err != nil {
		return nil, err
	}
	if !resp.IsSuccess() {
		return nil, fmt.Errorf("failed to upload book: %s", resp.String())
	}
	return result, nil
}

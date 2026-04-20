package utils

import (
	"encoding/json"
	"fmt"
	"os"

	"github.com/olekukonko/tablewriter"
)

func PrintJSON(data interface{}) {
	b, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		fmt.Printf("Error marshaling JSON: %v\n", err)
		return
	}
	fmt.Println(string(b))
}

func NewTable(headers []string) *tablewriter.Table {
	table := tablewriter.NewWriter(os.Stdout)
	for _, h := range headers {
		table.Header(h)
	}
	return table
}

func AppendRow(t *tablewriter.Table, row []string) {
	for _, col := range row {
		t.Append(col)
	}
}

func RenderTable(t *tablewriter.Table) {
	t.Render()
}

func FormatBool(b interface{}) string {
	if val, ok := b.(bool); ok {
		if val {
			return "true"
		}
		return "false"
	}
	return "N/A"
}

func GetString(m map[string]interface{}, key string) string {
	if val, ok := m[key].(string); ok {
		return val
	}
	return "N/A"
}

func GetNestedString(m map[string]interface{}, path ...string) string {
	var current interface{} = m
	for _, p := range path {
		if next, ok := current.(map[string]interface{}); ok {
			current = next[p]
		} else {
			return "N/A"
		}
	}
	if s, ok := current.(string); ok {
		return s
	}
	return "N/A"
}

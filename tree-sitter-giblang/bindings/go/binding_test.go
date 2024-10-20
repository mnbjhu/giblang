package tree_sitter_giblang_test

import (
	"testing"

	tree_sitter "github.com/smacker/go-tree-sitter"
	"github.com/tree-sitter/tree-sitter-giblang"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_giblang.Language())
	if language == nil {
		t.Errorf("Error loading Giblang grammar")
	}
}

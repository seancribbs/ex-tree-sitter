defmodule TreeSitter.Tree do
  defstruct [:tree, :language, :source]

  def root(%TreeSitter.Tree{tree: tree}) do
    TreeSitter.NIF.tree_root_node(tree)
  end

  def pre_walk(%TreeSitter.Tree{tree: tree}) do
    TreeSitter.NIF.tree_pre_walk(tree)
  end

  def query(%TreeSitter.Tree{tree: tree, language: language, source: source}, query) do
    TreeSitter.NIF.query_matches(tree, language, query, source)
  end

  def edit(
        %TreeSitter.Tree{tree: tree},
        %TreeSitter.InputEdit{} = edit
      ) do
    TreeSitter.NIF.tree_edit(tree, edit)
  end
end

defmodule TreeSitter.Language do
  def supported?(language) do
    TreeSitter.NIF.language_supported(language)
  end

  def queries(language) do
    TreeSitter.NIF.language_queries(language)
  end
end

defmodule TreeSitter.Parser do
  defstruct [:parser, :language]

  def new(language) do
    case TreeSitter.NIF.parser_new(language) do
      {:error, _} = err -> err
      parser -> {:ok, %TreeSitter.Parser{parser: parser, language: language}}
    end
  end

  def parse(%TreeSitter.Parser{parser: parser, language: language}, source) do
    case TreeSitter.NIF.parser_parse(parser, source) do
      {:error, _} = err ->
        err

      tree ->
        {:ok, struct(TreeSitter.Tree, tree: tree, language: language, source: source)}
    end
  end

  def reparse(
        %TreeSitter.Parser{parser: parser, language: language},
        tree,
        source
      )
      when is_struct(tree, TreeSitter.Tree) do
    case TreeSitter.NIF.parser_reparse(parser, tree.tree, source) do
      {:error, _} = err ->
        err

      tree ->
        {:ok, struct(TreeSitter.Tree, tree: tree, language: language, source: source)}
    end
  end
end

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
        edit
      )
      when is_struct(edit, TreeSitter.InputEdit) do
    TreeSitter.NIF.tree_edit(tree, edit)
  end
end

defmodule TreeSitter.Node do
  defstruct [
    :id,
    :text,
    :range,
    :kind,
    :kind_id,
    :is_named,
    :is_extra,
    :has_changes,
    :has_error,
    :is_error,
    :is_missing,
    :child_count
  ]
end

defmodule TreeSitter.Range do
  defstruct [:start_byte, :end_byte, :start_point, :end_point]
end

defmodule TreeSitter.Point do
  defstruct [:row, :column]
end

defmodule TreeSitter.InputEdit do
  defstruct [
    :start_byte,
    :old_end_byte,
    :new_end_byte,
    :start_position,
    :old_end_position,
    :new_end_position
  ]
end

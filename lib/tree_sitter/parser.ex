defmodule TreeSitter.Parser do
  defstruct [:parser, :language]

  def new(language) do
    case TreeSitter.NIF.parser_new(language) do
      {:error, _} = err -> err
      parser -> {:ok, %TreeSitter.Parser{parser: parser, language: language}}
    end
  end

  def parse(%TreeSitter.Parser{parser: parser, language: language}, source, old_tree \\ nil) do
    tree =
      if is_struct(old_tree, TreeSitter.Tree) do
        old_tree.tree
      end

    case TreeSitter.NIF.parser_parse(parser, source, tree) do
      {:error, _} = err ->
        err

      tree ->
        {:ok, %TreeSitter.Tree{tree: tree, language: language, source: source}}
    end
  end

  def set_language(%TreeSitter.Parser{parser: parser}, language) do
    case TreeSitter.NIF.parser_set_language(parser, language) do
      {:error, _} = err ->
        err

      _ ->
        {:ok, %TreeSitter.Parser{parser: parser, language: language}}
    end
  end

  def set_included_ranges(%TreeSitter.Parser{parser: parser}, ranges) when is_list(ranges) do
    unless Enum.all?(ranges, &match?(%TreeSitter.Range{}, &1)) do
      raise ArgumentError, "all given ranges must be TreeSitter.Range structs"
    end

    case TreeSitter.NIF.parser_set_included_ranges(parser, ranges) do
      {:error, _} = err ->
        err

      _ ->
        :ok
    end
  end
end

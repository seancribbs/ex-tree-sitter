defmodule TreeSitter.Parser do
  defstruct [:parser, :language]

  def new(language) do
    case TreeSitter.NIF.parser_new(language) do
      {:error, _} = err -> err
      parser -> {:ok, %TreeSitter.Parser{parser: parser, language: language}}
    end
  end

  def parse(%TreeSitter.Parser{parser: parser, language: language}, source, old_tree \\ nil) do
    case TreeSitter.NIF.parser_parse(parser, source, old_tree) do
      {:error, _} = err ->
        err

      tree ->
        {:ok, %TreeSitter.Tree{tree: tree, language: language, source: source}}
    end
  end
end

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
        {:ok, %TreeSitter.Tree{tree: tree, language: language, source: source}}
    end
  end

  def reparse(
        %TreeSitter.Parser{parser: parser, language: language},
        %TreeSitter.Tree{tree: tree},
        source
      ) do
    case TreeSitter.NIF.parser_reparse(parser, tree, source) do
      {:error, _} = err ->
        err

      tree ->
        {:ok, %TreeSitter.Tree{tree: tree, language: language, source: source}}
    end
  end
end

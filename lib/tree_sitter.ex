defmodule TreeSitter.Language do
  def supported?(language) do
    TreeSitter.NIF.language_supported(language)
  end

  def queries(language) do
    TreeSitter.NIF.language_queries(language)
  end
end

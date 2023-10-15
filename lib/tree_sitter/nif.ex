# This module is the low-level NIF API and is not meant to be used directly.
defmodule TreeSitter.NIF do
  @moduledoc false
  use Rustler,
    otp_app: :tree_sitter,
    crate: "ex_tree_sitter"

  def language_supported(_lang) do
    :erlang.nif_error(:nif_not_loaded)
  end

  def parser_new(_lang) do
    :erlang.nif_error(:nif_not_loaded)
  end

  def parser_parse(_parser, _text) do
    :erlang.nif_error(:nif_not_loaded)
  end
end

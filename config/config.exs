import Config

config :tree_sitter, TreeSitter.NIF, features: ["elixir"]

if Mix.env() == :test do
  config :tree_sitter, TreeSitter.NIF, features: ["elixir", "embedded-template", "html"]
end

defmodule TreeSitterTest do
  use ExUnit.Case

  test "creates a parser" do
    assert {:ok, %TreeSitter.Parser{}} = TreeSitter.Parser.new(:elixir)

    assert_raise ErlangError, fn ->
      TreeSitter.Parser.new(:invalid_language)
    end
  end

  test "parses an empty document" do
    assert {:ok, parser} = TreeSitter.Parser.new(:elixir)

    assert {:ok, %TreeSitter.Tree{}} = TreeSitter.Parser.parse(parser, "")
  end

  test "parses a single comment" do
    assert {:ok, parser} = TreeSitter.Parser.new(:elixir)

    assert {:ok, %TreeSitter.Tree{}} = TreeSitter.Parser.parse(parser, "# this is a comment")
  end

  test "parses a simple module" do
    assert {:ok, parser} = TreeSitter.Parser.new(:elixir)

    assert {:ok, %TreeSitter.Tree{}} = TreeSitter.Parser.parse(parser, "defmodule Foo do\n end")
  end

  test "gets the root node of a tree" do
    assert {:ok, parser} = TreeSitter.Parser.new(:elixir)

    assert {:ok, %TreeSitter.Tree{} = tree} =
             TreeSitter.Parser.parse(parser, "defmodule Foo do\n end")

    assert %TreeSitter.Node{} = TreeSitter.Tree.root(tree)
  end

  test "gets all the nodes in the tree in prefix order" do
    assert {:ok, parser} = TreeSitter.Parser.new(:elixir)

    assert {:ok, %TreeSitter.Tree{} = tree} =
             TreeSitter.Parser.parse(parser, "defmodule Foo do\n end")

    assert [%TreeSitter.Node{} | _] = TreeSitter.Tree.pre_walk(tree)
  end

  test "executes a query" do
    assert {:ok, parser} = TreeSitter.Parser.new(:elixir)

    assert {:ok, %TreeSitter.Tree{} = tree} =
             TreeSitter.Parser.parse(parser, "defmodule Foo do\n end")

    assert %{highlights: highlights} = TreeSitter.Language.queries(:elixir)

    assert [%TreeSitter.QueryMatch{} | _] = TreeSitter.Tree.query(tree, highlights)
  end
end

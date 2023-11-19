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

  test "accepts tree edits and reparses" do
    assert {:ok, parser} = TreeSitter.Parser.new(:elixir)

    old_text = "defmodule Foo do\nend"

    assert {:ok, %TreeSitter.Tree{} = tree} =
             TreeSitter.Parser.parse(parser, old_text)

    new_text = "defmodule Foo do\nend\n# comment"

    # NOTE: It feels really unpredictable what will and will not be shared
    # between trees after edits. We really need to understand this InputEdit
    # struct a lot better and possibly have lowered expectations.

    edit = %TreeSitter.InputEdit{
      start_byte: byte_size(old_text),
      old_end_byte: byte_size(old_text),
      new_end_byte: byte_size(new_text),
      start_position: %TreeSitter.Point{row: 1, column: 3},
      old_end_position: %TreeSitter.Point{row: 1, column: 3},
      new_end_position: %TreeSitter.Point{row: 2, column: 9}
    }

    old_mod_node = tree |> TreeSitter.Tree.pre_walk() |> Enum.find(&(&1.kind == "alias"))

    TreeSitter.Tree.edit(tree, edit)
    assert {:ok, new_tree} = TreeSitter.Parser.reparse(parser, tree, new_text)

    new_mod_node = new_tree |> TreeSitter.Tree.pre_walk() |> Enum.find(&(&1.kind == "alias"))
    assert old_mod_node.id == new_mod_node.id
  end
end

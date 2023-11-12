defmodule TreeSitter.QueryMatch do
  defstruct [:pattern_index, :captures]
end

defmodule TreeSitter.QueryCapture do
  defstruct [:node, :index, :capture_name]
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

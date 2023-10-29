defmodule TreeSitter.QueryMatch do
  defstruct [:pattern_index, :captures]
end

defmodule TreeSitter.QueryCapture do
  defstruct [:node, :index, :capture_name]
end

defmodule TreeSitter.Node do
  defstruct [
    :text,
    :range,
    :kind,
    :kind_id,
    :is_named,
    :is_extra
  ]
end

defmodule TreeSitter.Range do
  defstruct [:start_byte, :end_byte, :start_point, :end_point]
end

defmodule TreeSitter.Point do
  defstruct [:row, :column]
end

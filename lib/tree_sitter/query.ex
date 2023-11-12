defmodule TreeSitter.QueryMatch do
  defstruct [:pattern_index, :captures]
end

defmodule TreeSitter.QueryCapture do
  defstruct [:node, :index, :capture_name]
end

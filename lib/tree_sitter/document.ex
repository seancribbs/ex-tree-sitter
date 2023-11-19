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

defmodule TreeSitter.InputEdit do
  defstruct [
    :start_byte,
    :old_end_byte,
    :new_end_byte,
    :start_position,
    :old_end_position,
    :new_end_position
  ]
end

defmodule Day2 do
  def test_data do
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
  end

  @spec valid_report?(list(integer())) :: boolean()
  def valid_report?(levels) do
    increasing?(levels) || decreasing?(levels)
  end

  @spec increasing?(list(integer())) :: boolean()
  def increasing?(levels) do
    Enum.chunk_every(levels, 2, 1, :discard)
    |> Enum.all?(fn [a | [b | _]] -> b - a >= 1 && b - a <= 3 end)
  end

  @spec decreasing?(list(integer())) :: boolean()
  def decreasing?(levels) do
    Enum.chunk_every(levels, 2, 1, :discard)
    |> Enum.all?(fn [a | [b | _]] -> a - b >= 1 && a - b <= 3 end)
  end

  def q1(inputStr) do
    inputStr
    |> String.split("\n")
    |> Enum.map(&String.split/1)
    |> Enum.map(fn report ->
      Enum.map(report, fn level_str ->
        {level_num, ""} = Integer.parse(level_str)
        level_num
      end)
    end)
    |> Enum.count(&valid_report?/1)
  end

  def q2(inputStr) do
    inputStr
    |> String.split("\n")
    |> Enum.map(&String.split/1)
    |> Enum.map(fn report ->
      Enum.map(report, fn level_str ->
        {level_num, ""} = Integer.parse(level_str)
        level_num
      end)
    end)
    |> Enum.count(fn report -> Day2.valid_report?(report) || can_be_valid(report) end)
  end

  def can_be_valid(report) do
    # Every possible index of the report
    Enum.to_list(0..length(report))
    # remove it
    |> Enum.map(fn i -> List.delete_at(report, i) end)
    # valid once removed?
    |> Enum.any?(&Day2.valid_report?/1)
  end
end

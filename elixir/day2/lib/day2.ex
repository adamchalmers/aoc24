defmodule Day2 do
  def testData do
    "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
  end

  def validReport?(levels) do
    increasing(levels) || decreasing(levels)
  end

  def increasing(levels) do
    Enum.chunk_every(levels, 2, 1, :discard)
    |> Enum.all?(fn [a | [b | _]] -> b - a >= 1 && b - a <= 3 end)
  end

  def decreasing(levels) do
    Enum.chunk_every(levels, 2, 1, :discard)
    |> Enum.all?(fn [a | [b | _]] -> a - b >= 1 && a - b <= 3 end)
  end

  def q1(inputStr) do
    inputStr
    |> String.split("\n")
    |> Enum.map(&String.split/1)
    |> Enum.map(fn report ->
      Enum.map(report, fn levelStr ->
        {levelNum, ""} = Integer.parse(levelStr)
        levelNum
      end)
    end)
    |> Enum.filter(&validReport?/1)
    |> Enum.count()
  end

  def q2(inputStr) do
    inputStr
    |> String.split("\n")
    |> Enum.map(&String.split/1)
    |> Enum.map(fn report ->
      Enum.map(report, fn levelStr ->
        {levelNum, ""} = Integer.parse(levelStr)
        levelNum
      end)
    end)
    |> Enum.filter(fn report -> Day2.validReport?(report) || canBeValid(report) end)
    |> Enum.count()
  end

  def canBeValid(report) do
    n = length(report)

    # Every possible index of the report
    Enum.to_list(0..n)
    # remove it
    |> Enum.map(fn i -> List.delete_at(report, i) end)
    # valid once removed?
    |> Enum.any?(&Day2.validReport?/1)
  end
end

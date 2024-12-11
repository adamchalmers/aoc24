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
end

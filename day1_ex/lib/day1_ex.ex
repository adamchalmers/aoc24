defmodule Day1Ex do
  def testInput do
    "3   4
4   3
2   5
1   3
3   9
3   3"
  end

  def parseTwoLists(str) do
    lists =
      str
      |> String.split("\n")
      |> Enum.map(fn x -> String.split(x, "   ") end)
      |> Enum.zip()
      |> Enum.map(fn x -> Tuple.to_list(x) end)
      |> Enum.map(fn x -> Enum.map(x, fn y -> Integer.parse(y) |> elem(0) end) end)
      |> Enum.map(fn x -> Enum.sort(x) end)

    l = Enum.at(lists, 1)
    r = Enum.at(lists, 0)
    range = 0..(length(r) - 1)

    diffs =
      for i <- range,
          do: abs(Enum.at(l, i) - Enum.at(r, i))

    diffs |> Enum.sum()
  end

  def hello do
    IO.puts("hello")
    :world
  end
end

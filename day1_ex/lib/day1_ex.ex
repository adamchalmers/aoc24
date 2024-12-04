defmodule Day1 do
  def test_input do
    "3   4
4   3
2   5
1   3
3   9
3   3"
  end

  def q1(str) do
    str
    |> String.split("\n")
    |> Enum.map(fn x -> String.split(x, "   ") end)
    # Turn list of pairs into a pair of tuples
    |> Enum.zip()
    # zip returns tuples, but tuples aren't iterable. So make em lists instead.
    |> Enum.map(&Tuple.to_list/1)
    |> Enum.map(fn list -> Enum.map(list, &str_to_int/1) end)
    |> Enum.map(&Enum.sort/1)
    # Turn pair of lists into list of pairs.
    |> Enum.zip()
    # Find the absolute difference between each pair.
    |> Enum.map(fn pair -> abs(elem(pair, 0) - elem(pair, 1)) end)
    |> Enum.sum()
  end

  def str_to_int(str), do: Integer.parse(str) |> elem(0)
end

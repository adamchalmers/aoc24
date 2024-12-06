defmodule Day1 do
  @moduledoc """
  https://adventofcode.com/2024/day/1
  """
  def test_input do
    "3   4
4   3
2   5
1   3
3   9
3   3"
  end

  def pair_of_sorted_lists(str) do
    str
    |> String.split("\n")
    |> Enum.map(fn x -> String.split(x, "   ") end)
    # Turn list of pairs into a pair of tuples
    |> Enum.zip()
    # zip returns tuples, but tuples aren't iterable. So make em lists instead.
    |> Enum.map(&Tuple.to_list/1)
    |> Enum.map(fn list -> Enum.map(list, &str_to_int/1) end)
    |> Enum.map(&Enum.sort/1)
  end

  def q1(str) do
    pair_of_sorted_lists(str)
    # Turn pair of lists into list of pairs.
    |> Enum.zip()
    # Find the absolute difference between each pair.
    |> Enum.map(fn pair -> abs(elem(pair, 0) - elem(pair, 1)) end)
    |> Enum.sum()
  end

  def str_to_int(str), do: Integer.parse(str) |> elem(0)

  @doc """
  Maps list elements to their frequency. This deduplicates the list.
  """
  def count(list) do
    List.foldr(list, Map.new(), fn item, acc_map ->
      m = Map.put_new(acc_map, item, 0)
      Map.update!(m, item, fn x -> x + 1 end)
    end)
  end

  def q2(str) do
    [l | [r | []]] = pair_of_sorted_lists(str)
    r = count(r)

    Enum.map(l, fn item -> Map.get(r, item, 0) * item end)
    |> Enum.sum()
  end
end

f = File.read!("input")
IO.puts(Day1.q1(f))
IO.puts(Day1.q2(f))

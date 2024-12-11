defmodule Day2Test do
  use ExUnit.Case
  doctest Day2

  test "Q1 example" do
    assert Day2.q1(Day2.testData()) == 2
  end

  test "Q1 real" do
    {:ok, realData} = File.read("../../rust/input/2024/day2.txt")
    realData = String.trim(realData)
    assert Day2.q1(realData) == 230
  end
end

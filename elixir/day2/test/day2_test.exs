defmodule Day2Test do
  use ExUnit.Case
  doctest Day2

  test "Q1 example" do
    assert Day2.q1(Day2.test_data()) == 2
  end

  test "Q1 real" do
    {:ok, real_data} = File.read("../../rust/input/2024/day2.txt")
    real_data = String.trim(real_data)
    assert Day2.q1(real_data) == 230
  end

  test "Q2 real" do
    {:ok, real_data} = File.read("../../rust/input/2024/day2.txt")
    real_data = String.trim(real_data)
    assert Day2.q2(real_data) == 301
  end
end

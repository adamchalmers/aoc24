defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  test "Q1 on test input" do
    input = Day1.test_input()
    assert Day1.q1(input) == 11
  end

  test "Q1 on real input" do
    input = File.read!("input")
    assert Day1.q1(input) == 3_574_690
  end
end

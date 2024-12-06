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

  test "Q2 on test input" do
    input = Day1.test_input()
    assert Day1.q2(input) == 31
  end

  test "Q2 on real input" do
    input = File.read!("input")
    assert Day1.q2(input) == 22_565_391
  end
end

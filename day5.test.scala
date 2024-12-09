//> using toolkit 0.6.0
//> using file ./day5.scala

import io.cjf.aoc2024.day5._

val testInpRules = """
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13"""

val testRules = parseRules(testInpRules)

val testInpUpdates = """
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"""

val testUpdates = parseUpdates(testInpUpdates, testRules)

class Day5Test extends munit.FunSuite:
  test("mid") {
    assertEquals(testUpdates(0).mid, 61)
    assertEquals(testUpdates(1).mid, 53)
    assertEquals(testUpdates(2).mid, 29)
  }

  test("computing answer") {
    val result = testUpdates
      .filter(upd => upd.sorted(testRules).values == upd.values)
      .map(_.mid)
      .sum
    assertEquals(result, 143)
  }

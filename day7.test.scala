//> using toolkit 0.6.0
//> using file ./day7.scala

import io.cjf.aoc2024.day7._

class Day7Test extends munit.FunSuite:
  val inp =
    "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20"
  test("part 1") {
    val parsed = parseInputs(inp)
    val solved = sumPassed(parsed.map(p => (p.lhs, p.solve(0L))))
    assertEquals(solved, 3749L)
  }
  test("part 2") {
    val parsed = parseInputs(inp)
    val solved = sumPassed(parsed.map(p => (p.lhs, p.solveWithConcat(0L))))
    assertEquals(solved, 11387L)
  }

//> using toolkit 0.6.0
//> using file ./day8.scala

import io.cjf.aoc2024.day8._
import scala.io.Source

class Day8Test extends munit.FunSuite:
  val testInput = Source.fromFile("./inputs/day8.test.txt").mkString
  test("part2") {
    assertEquals(part2(testInput), 34)
  }

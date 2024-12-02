package io.cjf.aoc2024.day2

import scala.io.Source
import scala.util.boundary, scala.util.boundary.break
val inputs = Source.fromFile("./inputs/day2.txt").mkString

enum Direction:
  case Increasing
  case Decreasing

case class Report(val nums: Seq[Int]):
  def isSafe: Boolean =
    boundary:
      if nums.length < 2 then break(false)
      val direction =
        if nums.head < nums.last then Direction.Increasing
        else if nums.head > nums.last then Direction.Decreasing
        else break(false)
      nums.sliding(2).foreach { pair =>
        direction match
          case Direction.Decreasing =>
            if pair(0) <= pair(1) then break(false)
            else if pair(0) - pair(1) > 3 then break(false)
          case Direction.Increasing =>
            if pair(0) >= pair(1) then break(false)
            else if pair(1) - pair(0) > 3 then break(false)
      }
      true

  def removalIter: Iterable[Report] =
    (0 until nums.length).map { toRemove =>
      Report(
        nums.zipWithIndex
          .filter({ case (v, i) => i != toRemove })
          .map({ case (v, i) => v })
          .toSeq
      )
    }

  def isSafeWithProblemDampener: Boolean =
    isSafe || removalIter.exists(_.isSafe)

def parseInputs(inp: String): Seq[Report] =
  inp.linesIterator
    .map((line: String) =>
      Report(line.split(" ").filter(_.nonEmpty).map(_.toInt).toSeq)
    )
    .toSeq

def part1(inp: String): Int =
  parseInputs(inp).count(_.isSafe)

def part2(inp: String): Int =
  parseInputs(inp).count(_.isSafeWithProblemDampener)

@main def main() =
  println(s"Part 1: ${part1(inputs)}")
  println(s"Part 2: ${part2(inputs)}")

package io.cjf.aoc2024.day5

import scala.io.Source
import scala.math.floorDiv

val rules =
  parseRules(
    Source
      .fromFile("./inputs/day5_rules.txt")
      .mkString
  )

// Map from item -> prerequisites
def parseRules(inp: String): Map[Int, Seq[Int]] =
  inp.linesIterator
    .filter(!_.isEmpty)
    .map(_.split("""\|""").map(_.toInt))
    .toSeq
    .groupBy(_(1))
    .view
    .mapValues(_.map(_(0)))
    .toMap

case class Update(val values: Seq[Int]):
  def mid: Int =
    values(floorDiv(values.length, 2))

  def sorted(rules: Map[Int, Seq[Int]]): Update =
    Update(
      values.sortWith((a, b) =>
        rules.lift(b).map(_.contains(a)).getOrElse(false)
      )
    )

val updates =
  parseUpdates(
    Source
      .fromFile("./inputs/day5_updates.txt")
      .mkString,
    rules
  )

def parseUpdates(inp: String, rules: Map[Int, Seq[Int]]): Seq[Update] =
  inp.linesIterator
    .filter(!_.isEmpty)
    .map(l => l.split(",").map(_.toInt).toSeq)
    .filter(!_.isEmpty)
    .map(Update.apply)
    .toSeq

def part1(): Int =
  updates.filter(upd => upd.sorted(rules).values == upd.values).map(_.mid).sum

def part2() =
  updates
    .filter(upd => upd.sorted(rules).values != upd.values)
    .map(_.sorted(rules).mid)
    .sum

@main def main() =
  println(s"Part 1: ${part1()}")
  println(s"Part 2: ${part2()}")

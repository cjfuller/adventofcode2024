package io.cjf.aoc2024.day7

import scala.io.Source

val inputs = Source.fromFile("./inputs/day7.txt").mkString

enum Operator:
  case Add
  case Mul
  case Concat

enum CalResult:
  case Success(chain: Seq[Seq[Operator]])
  case Failure

  def chainWith(op: Operator): CalResult =
    this match
      case Failure     => Failure
      case Success(ch) => Success(ch.map(_ :+ op))

  def chainWithForward(op: Operator): CalResult =
    this match
      case Failure     => Failure
      case Success(ch) => Success(ch.map(op +: _))

  def combineWith(other: CalResult): CalResult =
    (this, other) match
      case (Failure, Failure)    => Failure
      case (Success(_), Failure) => this
      case (Failure, Success(_)) => other
      case (Success(lchain), Success(rchain)) =>
        Success(lchain ++ rchain)

case class CalibrationEquation(val lhs: Long, val rhs: Seq[Long]):
  def solve(running: Long): CalResult =
    if running > lhs then CalResult.Failure
    else if rhs.isEmpty && lhs == running then CalResult.Success(Seq(Seq()))
    else if rhs.isEmpty then CalResult.Failure
    else
      var rec = CalibrationEquation(lhs, rhs.tail)
        .solve(running + rhs.head)
        .chainWith(Operator.Add)

      rec = rec.combineWith(
        CalibrationEquation(lhs, rhs.tail)
          .solve(running * rhs.head)
          .chainWith(Operator.Mul)
      )
      rec

  def solveWithConcat(running: Long): CalResult =
    if running > lhs then CalResult.Failure
    else if rhs.isEmpty && lhs == running then CalResult.Success(Seq(Seq()))
    else if rhs.isEmpty then CalResult.Failure
    else
      var rec =
        CalibrationEquation(lhs, rhs.tail)
          .solveWithConcat(running + rhs.head)
          .chainWithForward(Operator.Add)

      val con = (running.toString() ++ rhs.head.toString()).toLong

      rec = rec
        .combineWith(
          CalibrationEquation(lhs, rhs.tail).solveWithConcat(con)
        )
        .chainWith(Operator.Concat)

      rec = rec.combineWith(
        CalibrationEquation(lhs, rhs.tail)
          .solveWithConcat(running * rhs.head)
          .chainWith(Operator.Mul)
      )
      rec

def parseInputs(inp: String): Seq[CalibrationEquation] =
  inp.linesIterator.map { l =>
    val parts = l.split(": ")
    val lhs = parts(0).toLong
    val rhs = parts(1).split(" ").map(_.toLong).toSeq
    CalibrationEquation(lhs, rhs)
  }.toSeq

def sumPassed(result: Iterable[(Long, CalResult)]): Long =
  result
    .filter((_, res) => res != CalResult.Failure)
    .map((lhs, _) => lhs)
    .sum

def part1(inp: String): Long =
  sumPassed(
    parseInputs(inp).map(c => (c.lhs, c.solve(0L)))
  )

def part2(inp: String) =
  sumPassed(
    parseInputs(inp)
      .map(cal => CalibrationEquation(cal.lhs, cal.rhs))
      .map(c => (c.lhs, c.solveWithConcat(0L)))
  )

@main def main() =
  println(s"Part 1: ${part1(inputs)}")
  println(s"Part 2: ${part2(inputs)}")

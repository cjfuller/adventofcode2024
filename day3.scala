//> using file ./parsers.scala

package io.cjf.aoc2024.day3
import io.cjf.aoc2024.parsers.{Parser, ParseResult}
import scala.io.Source

val inputs = Source.fromFile("./inputs/day3.txt").mkString

val Mul = (Parser.lit("mul(") >> Parser.num << Parser.lit(
  ","
)) :+ (Parser.num << Parser.lit(")"))

def part1(inp: String): Int =
  Mul(inp) match
    case ParseResult.Success((a, b), rest) =>
      a * b + (if rest.length > 0 then part1(rest) else 0)
    case ParseResult.Failure =>
      if inp.length > 0 then part1(inp.slice(1, inp.length)) else 0

enum ParseState:
  case Enabled
  case Disabled

def part2(inp: String, state: ParseState): Int =
  if inp.isEmpty then 0
  else
    state match
      case ParseState.Enabled =>
        (Parser.lit("don't()") | Mul)(inp) match
          case ParseResult.Failure => part2(inp.slice(1, inp.length), state)
          case ParseResult.Success(v, rest) =>
            v match
              case "don't()" => part2(rest, ParseState.Disabled)
              case (a, b)    => a * b + (part2(rest, state))
              case _         => throw new Error("Unreachable")
      case ParseState.Disabled =>
        Parser.lit("do()")(inp) match
          case ParseResult.Success(v, rest) => part2(rest, ParseState.Enabled)
          case ParseResult.Failure => part2(inp.slice(1, inp.length), state)

@main def main() =
  println(s"Part 1: ${part1(inputs)}")
  println(s"Part 2: ${part2(inputs, ParseState.Enabled)}")

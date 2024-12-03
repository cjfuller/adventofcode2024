package io.cjf.aoc2024.parsers

import scala.util.matching.Regex

enum ParseResult[+T]:
  case Success(value: T, rest: String)
  case Failure

trait Parser[+T]:
  def apply(target: String): ParseResult[T]

  def and_then[U](other: Parser[U]): Parser[(T, U)] =
    ParserSeq(this, other)

  infix def `:+`[U](other: Parser[U]): Parser[(T, U)] =
    and_then(other)

  infix def >>[U](other: Parser[U]): Parser[U] =
    (this :+ other).map { case (t, u) => u }

  infix def <<[U](other: Parser[U]): Parser[T] =
    (this :+ other).map { case (t, u) => t }

  infix def |[U](other: Parser[U]): Parser[T | U] =
    OrParser(this, other)

  def map[U](mapper: (T) => U): Parser[U] =
    MappedParser(this, mapper)

object Parser:
  def r(regex: Regex): Parser[String] = RegexParser(regex)
  def lit(s: String): Parser[String] = Lit(s)
  def num: Parser[Int] = Num

class MappedParser[T, U](orig: Parser[T], mapper: (T) => U) extends Parser[U]:
  def apply(target: String): ParseResult[U] =
    orig(target) match
      case ParseResult.Failure          => ParseResult.Failure
      case ParseResult.Success(t, rest) => ParseResult.Success(mapper(t), rest)

case class ParserSeq[T, U](val first: Parser[T], val second: Parser[U])
    extends Parser[(T, U)]:
  def apply(target: String): ParseResult[(T, U)] =
    first(target) match
      case ParseResult.Success(v1, rest1) => {
        second(rest1) match {
          case ParseResult.Success(v2, rest2) =>
            ParseResult.Success((v1, v2), rest2)
          case ParseResult.Failure => ParseResult.Failure
        }
      }
      case ParseResult.Failure => ParseResult.Failure

case class OrParser[T, U](val first: Parser[T], val second: Parser[U])
    extends Parser[T | U]:
  def apply(target: String): ParseResult[T | U] =
    first(target) match
      case ParseResult.Success(value, rest) => ParseResult.Success(value, rest)
      case ParseResult.Failure              => second(target)

class RegexParser(val re: Regex) extends Parser[String]:
  def apply(target: String): ParseResult[String] =
    val maybeMatch = re.findPrefixMatchOf(target)
    maybeMatch match
      case Some(m) =>
        ParseResult.Success(
          m.matched,
          target.slice(m.end, target.length)
        )
      case None => ParseResult.Failure

val Num = RegexParser("[0-9]+".r).map(_.toInt)

case class Lit(val s: String) extends Parser[String]:
  def apply(target: String): ParseResult[String] =
    if target.startsWith(s) then
      ParseResult.Success(s, target.slice(s.length, target.length))
    else ParseResult.Failure

//> using toolkit 0.6.0
//> using file ./parsers.scala

import io.cjf.aoc2024.parsers.{Parser, ParseResult}

class Tests extends munit.FunSuite:
  test("parsing num") {
    val parser = Parser.num
    val res = parser("104")
    assertEquals(res, ParseResult.Success(104, ""))
  }
  test("parsing lit") {
    val parser = Parser.lit("abcd")
    val res = parser("abcdefg")
    assertEquals(res, ParseResult.Success("abcd", "efg"))
  }
  test("chaining") {
    val parser = Parser.num :+ Parser.lit(" calling birds")
    val res = parser("4 calling birds")
    assertEquals(res, ParseResult.Success((4, " calling birds"), ""))
  }
  test("parsing complex") {
    val parser = (Parser.lit("mul(") >> Parser.num << Parser.lit(
      ","
    )) :+ (Parser.num << Parser.lit(")"))
    val res = parser("mul(32,3)")
    assertEquals(res, ParseResult.Success((32, 3), ""))
  }

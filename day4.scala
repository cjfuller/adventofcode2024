package io.cjf.aoc2024.day4

import scala.io.Source

val inputs = Source.fromFile("./inputs/day4.txt").mkString

enum SearchState:
  case Begin
  case X
  case M
  case A

enum SearchResult:
  case Found
  case NotFound

case class Coord(val row: Int, val col: Int):
  def ul: Coord = Coord(row - 1, col - 1)
  def ur: Coord = Coord(row - 1, col + 1)
  def ll: Coord = Coord(row + 1, col - 1)
  def lr: Coord = Coord(row + 1, col + 1)

class Grid(inp: String):
  val grid = inp.linesIterator.map(_.toCharArray().toSeq).toSeq

  def apply(c: Coord): Option[Char] =
    grid.lift(c.row).flatMap(_.lift(c.col))

  def search_from(
      state: SearchState,
      c: Coord,
      delta: Option[Coord]
  ): Seq[SearchResult] =
    import SearchState._
    val newState = (state, this(c)) match
      case (Begin, Some('X')) => X
      case (X, Some('M'))     => M
      case (M, Some('A'))     => A
      case (A, Some('S'))     => return Seq(SearchResult.Found)
      case _                  => return Seq(SearchResult.NotFound)

    delta match
      case Some(d) =>
        search_from(newState, Coord(c.row + d.row, c.col + d.col), Some(d))
      case None =>
        (-1 until 2).flatMap { row =>
          (-1 until 2).flatMap { col =>
            search_from(
              newState,
              Coord(c.row + row, c.col + col),
              Some(Coord(row, col))
            )
          }
        }

  def has_x_mas_at(c: Coord): Boolean =
    this(c) == Some('A') && (
      (this(c.ul) == Some('M') && this(c.ll) == Some('M') && this(c.ur) == Some(
        'S'
      ) && this(
        c.lr
      ) == Some('S')) ||
        (
          this(c.ul) == Some('M') && this(c.ur) == Some('M') && this(
            c.ll
          ) == Some('S') && this(c.lr) == Some('S')
        ) ||

        (this(c.ur) == Some('M') && this(c.lr) == Some('M') && this(
          c.ul
        ) == Some('S') && this(c.ll) == Some('S')) ||

        (this(c.ll) == Some('M') && this(c.lr) == Some('M') && this(
          c.ul
        ) == Some('S') && this(c.ur) == Some('S'))
    )

  def index_iter: Seq[Coord] =
    for
      row <- 0 until grid.length
      col <- 0 until grid(row).length
    yield Coord(row, col)

def part1(inp: String) =
  val grid = Grid(inp)
  grid.index_iter
    .flatMap { c => grid.search_from(SearchState.Begin, c, None) }
    .count(_ == SearchResult.Found)

def part2(inp: String) =
  val grid = Grid(inp)
  grid.index_iter
    .map(grid.has_x_mas_at)
    .count(c => c)

@main def main() =
  println(s"Part 1: ${part1(inputs)}")
  println(s"Part 2: ${part2(inputs)}")

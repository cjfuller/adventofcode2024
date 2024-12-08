package io.cjf.aoc2024.day8

import scala.collection.mutable
import scala.io.Source
import scala.util.boundary, scala.util.boundary.break

case class Coord(val row: Int, val col: Int):
  def -(other: Coord): CoordDiff =
    CoordDiff(row - other.row, col - other.col)

  def -(other: CoordDiff): Coord =
    this + (-other)

  def +(other: CoordDiff): Coord =
    Coord(row + other.rows, col + other.cols)

  def inBounds(dims: CoordDiff): Boolean =
    row < dims.rows && col < dims.cols && row >= 0 && col >= 0

case class CoordDiff(val rows: Int, val cols: Int):
  def unary_- : CoordDiff =
    CoordDiff(-rows, -cols)

case class Antenna(val pos: Coord, val freq: Char)

case class AntennaMap(
    val antennasByFreq: Map[Char, Seq[Antenna]],
    val dims: CoordDiff
)

case class Rule(
    val base: Coord,
    val diff: CoordDiff
):
  def isSatisfiedAt(c: Coord) =
    val trialDiff = c - base
    (trialDiff.rows % diff.rows == 0)
    && (trialDiff.cols % diff.cols == 0)
    && (trialDiff.rows / diff.rows == trialDiff.cols / diff.cols)

val inputs = Source.fromFile("./inputs/day8.txt").mkString

def parseInput(inp: String): AntennaMap =
  var sizeRows: Option[Int] = None
  var sizeCols: Option[Int] = None
  var antennas: mutable.Map[Char, Seq[Antenna]] = mutable.Map()
  inp.linesIterator.zipWithIndex.foreach { (row, ri) =>
    sizeRows = Some(ri + 1)
    row.zipWithIndex.foreach { (ch, ci) =>
      sizeCols = Some(ci + 1)
      if ch != '.' then
        if !antennas.contains(ch) then antennas(ch) = Seq()
        antennas(ch) = antennas(ch) :+ Antenna(Coord(ri, ci), ch)
    }
  }
  AntennaMap(antennas.toMap, CoordDiff(sizeRows.get, sizeCols.get))

def part1(inp: String): Int =
  val antennaMap = parseInput(inp)
  val nodes: mutable.Set[Coord] = mutable.Set()
  antennaMap.antennasByFreq.foreach { (freq, antennas) =>
    (0 until antennas.length).foreach { i =>
      (i + 1 until antennas.length).foreach { j =>
        val diff = antennas(j).pos - antennas(i).pos
        val firstNode = antennas(j).pos + diff
        val secondNode = antennas(i).pos - diff
        if firstNode.inBounds(antennaMap.dims) then nodes += firstNode
        if secondNode.inBounds(antennaMap.dims) then nodes += secondNode
      }
    }
  }
  nodes.size

def part2(inp: String): Int =
  val antennaMap = parseInput(inp)
  var rules: Seq[Rule] = Seq()
  val nodes: mutable.Set[Coord] = mutable.Set()
  antennaMap.antennasByFreq.foreach { (freq, antennas) =>
    (0 until antennas.length).foreach { i =>
      (i + 1 until antennas.length).foreach { j =>
        val diff = antennas(j).pos - antennas(i).pos
        rules = rules :+ Rule(antennas(i).pos, diff)
      }
    }
  }

  (0 until antennaMap.dims.rows).foreach { ri =>
    (0 until antennaMap.dims.cols).foreach { ci =>
      val c = Coord(ri, ci)
      if rules.exists(_.isSatisfiedAt(c))
      then nodes += c
    }
  }
  nodes.size

@main def main() =
  println(s"Part 1: ${part1(inputs)}")
  println(s"Part 2: ${part2(inputs)}")

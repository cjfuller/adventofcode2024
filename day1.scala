import scala.io.Source
import scala.collection.mutable.Buffer

val inputs = Source.fromFile("./inputs/day1.txt").mkString

def parseInputs(inp: String): (Buffer[Int], Buffer[Int]) =
  val first = Buffer[Int]()
  val second = Buffer[Int]()

  inputs.lines().map("""\s+""".r.split(_).filter(_.length > 0)).forEach { arr =>
    first.append(arr(0).toInt)
    second.append(arr(1).toInt)
  }
  (first, second)

def part1(): Int =
  var (first, second) = parseInputs(inputs)

  first = first.sorted()
  second = second.sorted()

  first.zip(second).map({ case (a, b) => (a - b).abs }).sum

def simScore(first: Seq[Int], second: Seq[Int]): Int =
  val counts = second.groupBy(x => x).view.mapValues(_.length)
  first.map(f => f * counts.applyOrElse(f, k => 0)).sum

def part2(): Int =
  var (first, second) = parseInputs(inputs)
  simScore(first.toSeq, second.toSeq)

@main def main() =
  println(s"Part 1: ${part1()}")
  println(s"Part 2: ${part2()}")

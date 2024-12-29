import scala.io.Source
import scala.collection.mutable

val inputs = Source.fromFile("./inputs/day23.txt").mkString

def buildGraph(inp: String): Set[(String, String)] =
  val outputs = mutable.Set[(String, String)]()
  inp.linesIterator.foreach { line =>
    val parts = line.split("-")
    outputs += (parts(0) -> parts(1))
    outputs += (parts(1) -> parts(0))
  }
  outputs.toSet

def connectedComponents(
    start: Set[String],
    nodes: Set[String],
    graph: Set[(String, String)],
    cache: mutable.Map[String, Set[String]]
): Set[String] =
  val key = start.toSeq.sorted.mkString(",")
  if cache.contains(key) then cache(key)
  else
    val result = nodes
      .filter(!start.contains(_))
      .toSeq
      .map { node =>
        if start.forall(curr => graph.contains((curr, node))) then
          connectedComponents(start + node, nodes, graph, cache)
        else start
      }
      .maxBy(_.size)
    cache(key) = result
    result

def part2(inp: String): String =
  val graph = buildGraph(inp)
  val nodes = graph.map(_(0)).toSet
  val cache = mutable.Map[String, Set[String]]()
  val solutions = connectedComponents(Set(), nodes, graph, cache)
  solutions.toSeq.sorted.mkString(",")

@main def main() =
  println(s"Part 2: ${part2(inputs)}")

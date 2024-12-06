import scala.collection.mutable
import scala.io.Source

val inputs = Source.fromFile("./inputs/day6.txt").mkString

enum Dir:
  case Up
  case Down
  case Left
  case Right

  def turnRight: Dir =
    this match
      case Up    => Right
      case Down  => Left
      case Left  => Up
      case Right => Down

case class Coord(val row: Int, val col: Int):
  def move(dir: Dir): Coord =
    import Dir._
    dir match
      case Up    => Coord(row - 1, col)
      case Down  => Coord(row + 1, col)
      case Left  => Coord(row, col - 1)
      case Right => Coord(row, col + 1)

  def inBounds(size: Coord): Boolean =
    row >= 0 && col >= 0 && row < size.row && col < size.col

case class State(
    val obstructions: Set[Coord],
    val size: Coord,
    val guard: Coord,
    val dir: Dir
):
  def nextState: Option[State] =
    val nextCoord = guard.move(dir)
    if nextCoord.inBounds(size) then
      if obstructions.contains(nextCoord) then
        Some(State(obstructions, size, guard, dir.turnRight))
      else Some(State(obstructions, size, nextCoord, dir))
    else None

def parseInputs(inp: String): State =
  var sizeRows = 0
  var sizeCols = 0
  var guardPos: Option[Coord] = None
  var guardDir: Option[Dir] = None
  val obs: mutable.Set[Coord] = mutable.Set()
  inp.linesIterator.zipWithIndex.foreach { (r, ri) =>
    sizeRows = ri + 1
    val chars = r.toCharArray
    sizeCols = r.length
    chars.zipWithIndex.foreach { (c, ci) =>
      if c == '#' then obs += Coord(ri, ci)
      else if c == '^' then
        guardPos = Some(Coord(ri, ci))
        guardDir = Some(Dir.Up)
      else if c == '>' then
        guardPos = Some(Coord(ri, ci))
        guardDir = Some(Dir.Right)
      else if c == '<' then
        guardPos = Some(Coord(ri, ci))
        guardDir = Some(Dir.Left)
      else if c == 'v' then
        guardPos = Some(Coord(ri, ci))
        guardDir = Some(Dir.Down)
    }
  }
  State(obs.toSet, Coord(sizeRows, sizeCols), guardPos.get, guardDir.get)

def part1(inp: String): Int =
  var state: Option[State] = Some(parseInputs(inp))
  var visited: mutable.Set[Coord] = mutable.Set()
  while state.isDefined do
    visited += state.get.guard
    state = state.get.nextState
  visited.size

def part2(inp: String) =
  val origState: State = parseInputs(inp)
  var obstructable: mutable.Set[Coord] = mutable.Set()
  (0 until origState.size.row).foreach { ro =>
    (0 until origState.size.col).foreach { co =>
      val newObs = Coord(ro, co)
      if origState.obstructions.contains(
          newObs
        ) || origState.guard == newObs
      then ()
      else
        var state: Option[State] = Some(
          State(
            origState.obstructions + newObs,
            origState.size,
            origState.guard,
            origState.dir
          )
        )
        val visited: mutable.Set[(Coord, Dir)] = mutable.Set()
        while state.isDefined do
          if visited.contains((state.get.guard, state.get.dir)) then
            obstructable += newObs
            state = None
          else
            visited += ((state.get.guard, state.get.dir))
            state = state.get.nextState

    }
  }
  obstructable.size

@main def main() =
  println(s"Part 1: ${part1(inputs)}")
  println(s"Part 2: ${part2(inputs)}")

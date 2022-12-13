package maze

import maze.Exploration.{Explored, Unexplored}
import maze.Maze.Branch

import scala.collection.mutable.ListBuffer

enum Exploration:
  case Explored, Unexplored

enum Maze:
  case Branch(label: String, left: Maze, right: Maze, var status: Exploration)
  case Leaf(label: String)

  def explore(): List[String] = this match
    case Leaf(l) => List(l)
    case branch@Branch(label, l, r, status) => status match
      case Explored => List(label)
      case Unexplored => {
        branch.status = Explored
        label :: l.explore() ++ r.explore()
      }
  def explore(trace: ListBuffer[String]): Unit = this match
    case Leaf(label) => trace+= label
    case branch@Branch(label, l, r, status) => status match
      case Explored => trace += label
      case Unexplored => {
        branch.status = Explored
        trace += label
        l.explore(trace)
        r.explore(trace)
      }

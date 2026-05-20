# Puzzle 3

We want to union any number of sets (`RangeSetBlaze<T>`). In OO terms, any collection that is an iterable of `RangeSetBlaze<T>` references should inherit this operation.

```mermaid
classDiagram
    direction TB

    class Iterable {
        <<abstract class>>
        +iterator()
    }

    class RangeSetCollection {
        <<abstract class>>
        +iterator() // from Iterable
        +union()
    }

    class VectorOfRangeSetRefs {
        <<concrete class>>
        +iterator()
        +union() // inherited
    }

    class ArrayOfRangeSetRefs {
        <<concrete class>>
        +iterator()
        +union() // inherited
    }

    class AnyOtherRangeSetCollection {
        <<concrete class>>
        +iterator()
        +union() // inherited
    }

    Iterable <|-- RangeSetCollection : is-a
    RangeSetCollection <|-- VectorOfRangeSetRefs : is-a
    RangeSetCollection <|-- ArrayOfRangeSetRefs : is-a
    RangeSetCollection <|-- AnyOtherRangeSetCollection : is-a
```

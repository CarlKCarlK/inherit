# Puzzle 5

Treat 15 integer-like types uniformly:
 `u8`, ...,  `usize`, ...`i128`, `char`, `IPv4`, `IPv6`
 
For example, with a `min`, `max`, and ability to add one (unchecked).

```mermaid
classDiagram
    direction TB

    class Integer {
        <<abstract class>>
        +add_one()
        +min_value()
        +max_value()
    }

    class NumericInteger {
        <<abstract class>>
        +add_one()
        +min_value()
        +max_value()
    }

    class IpInteger {
        <<abstract class>>
        +add_one()
        +min_value()
        +max_value()
    }

    class CharInteger {
        <<abstract class>>
        +add_one()
        +min_value()
        +max_value()
    }

    class u8 {
        <<concrete class>>
        +add_one() // inherited
        +min_value() // inherited
        +max_value() // inherited
    }

    class IPv4Type {
        <<concrete class>>
        +add_one() // inherited
        +min_value() // inherited
        +max_value() // inherited
    }

    class IPv6Type {
        <<concrete class>>
        +add_one() // inherited
        +min_value() // inherited
        +max_value() // inherited
    }

    class CharType {
        <<concrete class>>
        +add_one() // inherited
        +min_value() // inherited
        +max_value() // inherited
    }

    Integer <|-- NumericInteger : is-a
    Integer <|-- IpInteger : is-a
    Integer <|-- CharInteger : is-a

    NumericInteger <|-- u8 : is-a
    IpInteger <|-- IPv4Type : is-a
    IpInteger <|-- IPv6Type : is-a
    CharInteger <|-- CharType : is-a
```

# Puzzle 9

We want two levels of `FlashBlock`: a base level with `new` and `clear`, and a constrained level that adds `save` and `load` only when `T` satisfies both serialization and deserialization capabilities.

```mermaid
classDiagram
    direction TB

    class Serialize {
        <<superclass>>
        +serialize()
    }

    class Deserialize {
        <<superclass>>
        +deserialize()
    }

    class TConstrained["T: Serialize + Deserialize"] {
        <<concrete subclass>>
        +serialize() // inherited
        +deserialize() // inherited
    }

    class FlashBlock {
        <<superclass>>
        +new()
        +clear()
    }

    class FlashBlockForTSerializeDeserialize {
        <<subclass>>
        +new() // inherited
        +clear() // inherited
        +save(value, key)
        +load(key) T
    }

    Serialize <|-- TConstrained : is-a
    Deserialize <|-- TConstrained : is-a
    FlashBlock <|-- FlashBlockForTSerializeDeserialize : is-a
```

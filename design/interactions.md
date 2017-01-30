# Interactions

## Possible Triggers

- Person in same room
    - with status effects, e.g. too tired, overly stressed
    - with traits, e.g. explorer, ditcher
- Upcoming or current event
    - asked a question in class
    - need to leave for event
- Self status
    - Sleeping
    - On call and get a call
- Location
    - Walking in the plantations

## Implementation

- Pure 'tag' system is too general: we basically have to scan every possible interaction every time we want to generate one
- Split interactions into broad categories, then use tags/traits/etc to refine which are possible
    - Person-based
    - Event-based

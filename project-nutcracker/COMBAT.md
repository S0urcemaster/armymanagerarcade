# Combat Model

## Accepted foundation

Combat is a numerical arcade simulation built from persistent individual
soldiers. It advances in observable phases, with player decisions before combat
and between phases. Opponent information becomes more precise over time.

## Formation preparation

There are five army-size levels, including the initial 1–50 soldier level. Each
level has one predefined formation template with dedicated infantry or
spearmen, cavalry, and archer blocks. The player assigns recruited soldiers to
the blocks where their properties fit best.

The enemy uses a mirrored template. Espionage reveals an increasingly accurate
estimate of every enemy block before combat, allowing the player to prepare a
counter-formation without receiving complete information.

### Accepted formation levels

| Level | Army size | Formation | Placement |
| --- | ---: | --- | --- |
| 1 | 1–50 | One spearmen block | Individual soldiers |
| 2 | 51–500 | One spearmen, one archer, and one cavalry block; ten ranks per block; roughly 166 soldiers maximum per block | Detailed assignment |
| 3 | 501–5,000 | More blocks and soldiers; exact template open | Increasingly grouped |
| 4 | 5,001–50,000 | More blocks and soldiers; exact template open | Coarse group assignment |
| 5 | 50,001–500,000 | More blocks and soldiers; exact template open | Broad distribution |

Control uses progressive abstraction. The player can optimize individual
positions in small armies but distributes increasingly large groups at higher
levels.

### Strategic rank order

Soldier order inside a block is meaningful. Veterans can be placed in front
against a weak opponent to participate more and gain experience, or held in
later ranks against a strong opponent to improve their chance of surviving.
The formation editor must therefore support ordering goals rather than assuming
that the strongest-first arrangement is always best.

### Block strategies

The player assigns recruits to suitable troop-type blocks and configures a
strategy for each block. Strategies influence ordering and combat behavior,
providing strong benefits with explicit trade-offs. They may exaggerate
historical ideas to remain readable and enjoyable as arcade mechanics.

Initial examples:

| Block | Strategy | Benefit | Trade-off |
| --- | --- | --- | --- |
| Spearmen | Shield wall | Nearly negates incoming archer attacks | Gives opposing spearmen more room or opportunity on their following attack |
| Archers | Dispersion | Reduces the first cavalry attack | Not yet defined |

Block strategies also provide a way to automate detailed soldier ordering as
army size increases.

## Combat ticks

One combat tick resolves the fight between two currently opposing ranks. The
player advances to another tick with "Continue" when no intervening command is
needed. Across successive ticks, ranks inflict losses, advance, and are
gradually consumed.

## Troop types

- Spearmen
- Archers
- Cavalry

Each soldier has a combat value and modifiers for preferred and unfavorable
opponent types.

```text
Archers  > Spearmen
Spearmen > Cavalry
Cavalry  > Archers
```

## Round sequence

### Opening round

1. Every available archer attacks an opposing spearman.
2. When archers outnumber available targets in the exposed ranks, front-rank
   soldiers can receive multiple attacks.
3. The front rank of spearmen fights the opposing front rank.
4. Cavalry waits and does not attack.

### Following rounds

- Archers continue attacking opposing spearmen.
- Spearmen continue their front-rank fight.
- Cavalry begins fighting opposing cavalry from the second round.

## Breakthroughs

When a mirrored formation is defeated, its survivors break through and join
another fight. The currently unambiguous transition is:

```text
Victorious cavalry  -> opposing archers
```

The spearmen transition still needs clarification. The cyclic advantage points
to opposing cavalry, while the current design example places victorious
spearmen among the opposing archers. Archers currently attack spearmen directly
rather than fighting a mirrored archer front.

The battle therefore becomes a race to create a useful breakthrough before an
enemy front collapses.

## Still open

- Damage and injury calculation
- Definition of formation defeat or routing
- Exact interpretation of ranks and repeated archer targeting
- Commands available before and between phases
- Shapes and capacities of the five formation templates
- Rank width, overflow, advancement, and replacement rules
- Rules for soldier suitability within formation blocks
- Retreat and overall victory conditions
- Presentation of individual encounters and small personal events
- Interaction with terrain, equipment, belonging, and leaders

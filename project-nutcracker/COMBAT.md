# Combat Model

## Accepted foundation

Combat is a numerical arcade simulation built from persistent individual
soldiers. It advances in observable phases, with player decisions before combat
and between phases. Opponent information becomes more precise over time.

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
- Retreat and overall victory conditions
- Presentation of individual encounters and small personal events
- Interaction with terrain, equipment, belonging, and leaders

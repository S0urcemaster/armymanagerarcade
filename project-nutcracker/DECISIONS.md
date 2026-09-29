# Decision Log

## D-001 — Decision-guided project process

**Status:** Accepted

Use Project Nutcracker as a project companion that surfaces the most important
current concerns, asks the owner only for consequential choices, and records
their effects as the project develops.

### Rationale

The project owner should not need to know in advance which product details are
important. The process should expose those details when they become relevant.

### Consequences

- "What comes next?" is the standard entry point for resuming project work.
- Answers prioritize a short list instead of presenting an exhaustive backlog.
- The agent proceeds autonomously where accepted decisions already imply the
  answer.
- Product choices are recorded with their downstream consequences.

## D-002 — Core competition fantasy

**Status:** Partially superseded by D-008 and D-012

Treat war as a sport. The player develops continuously from one person into the
leader of an army and ultimately reaches first place. The original conventional
league structure was later replaced by the ranked realm map in D-008 and the
direct position system in D-012. A separate cup remains an unconfirmed concept.

### Rationale

The sports structure gives repeated battles an understandable competitive
framework, a visible path of advancement, and a definitive long-term goal.

### Consequences

- The campaign begins with exactly one person.
- Army growth is continuous rather than split into unrelated scenarios.
- Leagues communicate rank and progression.
- A cup provides a competition distinct from the league.
- Winning the first league is the primary victory condition.
- The maximum army size remains unresolved.

## D-003 — Four-topic decision rounds

**Status:** Accepted

Each owner decision round has two distinct selection steps. First, the agent
presents exactly four high-weight project topics and the owner selects one.
Only then does the agent present exactly four solution choices for the selected
topic.

### Consequences

- Topic selection remains strategic and avoids premature detail.
- Choosing a topic is distinct from choosing its solution.
- Solution choices may still be adapted or combined by the owner.

## D-004 — Long seasons and strength-class leagues

**Status:** Partially superseded by D-008

Originally defined as long, table-based seasons in army-strength classes. The
army-strength progression remains accepted, but D-008 replaces the scheduled
league and season structure with a ranked map of competing realms.

### Provisional scale example

- 1–50 soldiers
- 51–500 soldiers
- 501–5,000 soldiers
- 5,001–50,000 soldiers
- 50,001–500,000 soldiers

These ranges express the intended order-of-magnitude growth. Their exact bounds
and the number of leagues remain open to balancing and technical validation.

### Consequences

- Advancement requires both army development and competitive success in the
  realm ranking; the exact promotion rule must be revised for D-008.
- Army growth, training, and equipment are prerequisites for competing at the
  next scale.
- The interface and simulation must scale through nested military groups.
- The player must be able to develop or equip both individual soldiers and
  whole groups at appropriate scales.
- Group organization is a core mechanic rather than only a performance detail.

## D-005 — Individual soldiers inside a zoomable hierarchy

**Status:** Accepted

Represent the army as a nested hierarchy whose leaves are persistent individual
soldiers. The player can zoom from the largest formations down to groups and
individual people, selecting and managing any appropriate level.

In combat, individual soldiers still face individual opponents. Large battles
are primarily simulations of numbers and outcomes rather than visually rendered
mass combat.

### Design direction

- The setting is antiquity.
- Use only a small number of soldier classes and attributes to preserve the
  arcade character.
- Show many faces and allow small individual fates to create emergent stories.
- Recruits often arrive as existing social groups.
- Soldiers have a belonging attribute tied to their group history.
- Separating an established group reduces belonging.
- Surviving battles with a group can build belonging again.
- Leaders exist, but their roles and mechanics remain unresolved.
- Candidate combat concepts such as attack, defense, bonuses, and penalties are
  still provisional.

### Consequences

- Group totals must be derived from, or remain consistent with, their members.
- Commands such as training and equipping can target either a formation or an
  individual soldier.
- The interface needs semantic zoom rather than separate unrelated army views.
- The simulation must support large numbers of individual encounters without
  requiring all of them to be animated or displayed simultaneously.
- Combat rules must stay legible despite the underlying number of soldiers.

## D-006 — Observed combat phases with partial information

**Status:** Accepted

Combat advances in discrete phases. Before the first phase and between later
phases, the player makes a small number of command decisions and deliberately
continues the battle. Each phase then runs visibly for several seconds without
requiring constant input.

Information about the opposing army is incomplete before combat. Additional
strengths, weaknesses, composition details, or behavior are revealed as the
battle develops, so later commands respond to observed evidence rather than to
complete advance knowledge.

### Intended rhythm

1. Review the currently available information.
2. Issue a few commands.
3. Continue the battle.
4. Watch the numerical simulation unfold for several seconds.
5. Receive new observations and make the next decision.

### Consequences

- Combat is deliberate rather than dependent on reaction speed.
- Every phase should reveal or change enough information to justify another
  decision.
- The interface must distinguish known, estimated, and newly revealed facts.
- Computer opponents need recognizable styles and exploitable weaknesses.
- Opponent behavior must operate under clear rules suitable for deterministic
  testing, even when outcomes contain uncertainty.
- Phase length and total phase count remain open.

## D-007 — Time-based intelligence and reconnaissance advancement

**Status:** Accepted

Opponent information becomes more precise automatically as combat phases
advance. Reconnaissance technology can reveal some of that information before
combat, improving opponent selection and initial orders.

### Consequences

- Information revelation follows a predictable arcade-friendly progression.
- Reconnaissance progression has a direct and visible strategic benefit.
- Pre-battle information must use the same information layers as combat rather
  than forming a disconnected scouting system.
- Better intelligence reduces uncertainty but should not remove every surprise.

## D-008 — Ranked realms instead of a scheduled sports league

**Status:** Accepted

Replace the fixed sequence of league opponents with a map of competing ancient
realms. Cities, kingdoms, duchies, and larger empires fight one another and are
ordered in a shared ranking. Each army has a prominent, persistent NPC leader.

The sports analogy remains in the ranking, competitive advancement, and goal of
reaching the top, but no longer requires a conventional fixture schedule.

### Consequences

- The world map supplies opponents and context for conflicts.
- Opponent selection replaces a strictly scheduled sequence of matches.
- Realm strength and player army-strength class must both influence suitable
  opponents.
- NPC leaders can provide recognizable styles, weaknesses, and ongoing rivalries.
- The precise historical period and degree of historical simultaneity remain
  unresolved.

## D-009 — An all-star antiquity with army-size ranking

**Status:** Accepted

Use a deliberately anachronistic ancient world in which historical realms,
cities, cultures, and prominent people from different eras can meet, comparable
to the cross-era premise of a civilization game. Preserve an educational layer
by identifying their real dates, places, roles, and selected historical facts.

Rank armies primarily by their assigned number of soldiers. Do not attempt to
model or rank the real historical combat effectiveness of each army.

Every army has one persistent featured NPC leader. "Leader" is a game role and
may historically represent a ruler, elected magistrate, general, queen, king,
emperor, or another well-attested commander.

### Consequences

- Historical simultaneity is not required.
- Assigned army size is a gameplay value and must not be presented as a
  historical claim.
- Educational profiles distinguish documented facts from game fiction.
- Republics, dual monarchies, and confederations can still use one featured
  leader without claiming that the person ruled alone.
- Prefer attested people; any composite or fictional leader must be labeled.
- The geographical extent of the roster remains unresolved.

## D-010 — Global antiquity

**Status:** Accepted

Draw the roster from global antiquity rather than limiting it to the
Mediterranean or Eurasia. Include well-researched cultures from Africa, Asia,
Europe, Oceania, and the Americas where suitable named polities and featured
leaders are available.

### Consequences

- Geography and historical dates are educational metadata, not restrictions on
  possible matchups.
- Cultural coverage should be broad without inventing certainty where sources
  are limited.
- Initial roster size and distribution across army-size classes remain open.

## D-011 — Forty-eight persistent NPC armies

**Status:** Accepted

Use an initial roster of 48 persistent NPC armies between 51 and 500,000
soldiers, distributed evenly across four army-size classes.

| Army-size class | NPC armies |
| --- | ---: |
| 51–500 | 12 |
| 501–5,000 | 12 |
| 5,001–50,000 | 12 |
| 50,001–500,000 | 12 |

The five highest-ranked armies form the final competitive group at the top of
the world ranking. Exact realms, leaders, and assigned sizes remain provisional
until the roster is researched and reviewed.

### Consequences

- NPC armies persist and can develop histories and rivalries across encounters.
- Every size class must contain distinct regions, leaders, and opponent styles.
- The ranking should make recurring opponents meaningful rather than constantly
  replacing them with generated entries.
- The player is additional to the 48 NPC armies.

## D-012 — Direct rank exchange and challenge events

**Status:** Accepted

When the player defeats a higher-ranked army in a ranking-eligible battle, the
two armies exchange positions directly. NPC armies may also issue event-driven
challenges with special battle conditions.

### Consequences

- Rank movement is immediate and easy to explain.
- Access to much higher-ranked opponents may need limits to preserve progression.
- The result of a ranking-eligible battle matters more than accumulated points.
- Challenge events can restrict army size, formation, equipment, soldier class,
  terrain, available intelligence, or other battle rules.
- It must always be clear before acceptance whether a challenge affects rank.
- Challenge frequency, refusal rules, stakes, and NPC-versus-NPC rank exchanges
  remain unresolved.

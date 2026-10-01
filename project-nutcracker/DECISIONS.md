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
If the selected topic remains too broad for a useful decision, the agent
presents exactly four high-weight subtopics. This narrowing may repeat before
the agent presents four concrete solution choices.

### Consequences

- Topic selection remains strategic and avoids premature detail.
- Broad topics can be narrowed hierarchically without pretending they are
  already decision-sized.
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

**Status:** Partially superseded by D-016

This decision replaced the conventional league with ranked ancient realms and
persistent NPC leaders. D-016 later replaced the dynamic opponent structure
with a scripted sequence of stages and places.

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

**Status:** Partially superseded by D-016

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

## D-013 — Type fronts and breakthrough cycle

**Status:** Accepted

Build combat around three troop-type fronts: spearmen, archers, and cavalry.
Soldiers resolve individual attacks using a combat value modified by advantage
or disadvantage against the target type.

Use a cyclic matchup advantage:

- archers defeat spearmen;
- spearmen defeat cavalry;
- cavalry defeat archers.

Archers attack opposing spearmen at the start of a round. Every available
archer fires once; if archers outnumber the target formation, soldiers in its
front rows may be targeted repeatedly. Spearmen then fight opposing spearmen at
the front rank. Cavalry does not act in the opening round and begins fighting
opposing cavalry from the second round onward.

When a mirrored formation is defeated, the surviving force breaks through and
joins another fight. Victorious cavalry proceeds to the opposing archers. The
destination of victorious spearmen remains open: the cyclic advantage suggests
cavalry, while the current example has them reaching the opposing archers.

### Consequences

- Combat creates a race between breakthroughs on three connected fronts.
- Army composition affects both initial matchups and the consequences of a
  breakthrough.
- Large formations still resolve through individual soldier interactions.
- Front-rank and target-selection rules must be deterministic and visible.
- Damage, routing, formation defeat, commands, and overall victory conditions
  remain unresolved.
- The breakthrough destination for spearmen requires clarification when combat
  detail is revisited.

## D-014 — Gold, recruit intelligence, and espionage points

**Status:** Accepted

Use gold as the main purchasing currency. Recruits appear in a changing pool
and have different gold prices. Some recruits also bring intelligence; hiring
them awards espionage points in addition to adding them to the army.

Espionage points are a separate resource spent to investigate armies in the
world ranking. Investigation proceeds through the ranking and reveals
decision-relevant opponent information before combat.

### Consequences

- Recruit value combines combat usefulness, group membership, equipment,
  development potential, and possible intelligence value.
- A militarily weak recruit may still be strategically desirable.
- Intelligence has an opportunity cost because obtaining it can consume gold
  and roster capacity through recruitment.
- Espionage points bring forward information that would otherwise be revealed
  during combat phases.
- The exploration order, information costs, recruit dismissal rules, and
  safeguards against recruitment exploits remain unresolved.

## D-015 — Development, loot, and territory as reasons for war

**Status:** Accepted

Battles create lasting development and economic consequences:

- surviving soldiers gain experience;
- soldiers who actively fought gain more experience;
- soldiers who were wounded and survived gain an additional experience reward;
- killed soldiers leave behind their equipment and personal gold;
- equipment not needed by the army can be sold;
- victory can conquer territory, produce limited plunder, and add recurring
  income;
- the player begins with enough gold to enter the initial cycle of recruitment
  and combat.

Every soldier has a personal gold reserve distinct from the army treasury.

### Consequences

- War advances veteran soldiers while also risking their permanent loss.
- Battlefield casualties transfer or release material value into the post-battle
  economy.
- Territory connects combat success to longer-term income.
- Experience, loot, conquest, and rank can make the same opponent attractive for
  different reasons.
- Wounds must not become something the player intentionally farms without
  meaningful risk.
- Battlefield recovery, ownership after defeat, plunder limits, territorial
  control, taxes, and the relationship between personal and army gold remain
  unresolved.

## D-016 — Scripted progression through places

**Status:** Accepted

Do not use a league or a dynamic sequence of opponents. Progress through a
scripted order of stages, with one primary opponent after another. Each stage is
associated with a place, such as a settlement, city, or territory, and its NPC
army.

The player may skip opponents and challenge a later stage. On victory, the
player exchanges places with the defeated opponent. On defeat, the player falls
back to the previous place in the progression.

Places provide different recurring income and may have additional properties
that influence preparation for, or conditions in, the next battle.

### Consequences

- Progression is authored and can build a deliberate difficulty and historical
  learning curve.
- The 48 persistent NPC armies can populate 48 scripted stages rather than a
  simulated league.
- Skipping creates a risk-reward choice between faster advancement and a harder
  opponent.
- Place exchange connects victory, territory, income, and rank in one action.
- Places can create strategic variety without requiring a fully dynamic world
  simulation.
- The treatment of skipped opponents, defeat at the first place, NPC movement,
  and exact place modifiers remain unresolved.

## D-017 — Five mirrored formation templates

**Status:** Accepted

Each of the five army-size levels has one predefined formation template. Every
template contains dedicated blocks for infantry or spearmen, cavalry, and
archers. The player fills the current template by assigning recruited soldiers
to the positions where they fit best.

The opposing army uses a mirrored formation. Before battle, espionage provides
an approximate picture of each opposing block. The player uses this incomplete
information to design their own troop assignment for that opponent.

### Consequences

- Army preparation is a spatial assignment problem rather than only a comparison
  of total troop counts.
- Formation complexity can increase across the five army-size levels while the
  three troop types remain consistent.
- Soldier suitability must be visible when assigning recruits to blocks.
- Espionage information must map directly onto the same blocks shown in the
  formation editor.
- Mirrored blocks provide clear opposing relationships at battle start.
- Exact template shapes, capacities, suitability rules, and whether assignments
  may change during combat remain unresolved.

## D-018 — Progressive formation scale and row-based combat ticks

**Status:** Accepted

The first two formation levels are defined as follows:

1. **1–50 soldiers:** one infantry or spearmen block; soldiers can be placed
   individually.
2. **51–500 soldiers:** one block for each of the three troop types, with up to
   roughly 166 soldiers per block and ten ranks per block.

A combat tick resolves the encounter between two currently opposing ranks. If
no other decision is available or necessary, the player repeatedly presses
"Continue" and watches the ranks wear one another down over successive ticks.

Higher army levels add more blocks and more soldiers. Control becomes
progressively more abstract: early armies reward deliberate placement of every
soldier, while the largest armies are managed through coarse distribution of
large groups.

### Consequences

- The same formation concept scales from individual placement to aggregate
  command instead of replacing the interface with an unrelated system.
- Early individual optimization can create advantages and personal stories that
  persist into later progression.
- A block needs deterministic rules for rank width, overflow, advancement, and
  losses.
- "Continue" must remain fast and informative even when no command is required.
- The block counts, rank counts, and control granularity of levels three through
  five remain unresolved.

## D-019 — Strategic ordering inside blocks

**Status:** Accepted

The order of soldiers within a block is a strategic choice. In particular, the
player can place veterans in early ranks against a weak opposing block to give
them more combat participation and experience, or preserve them in later ranks
against a dangerous block so they are less exposed to early losses.

### Consequences

- There is no universally optimal strongest-first ordering.
- Espionage estimates influence not only troop counts but also internal block
  order.
- Experience incentives compete with survival and immediate battle strength.
- The formation editor should communicate the expected role and exposure of
  early and late ranks.
- Additional ordering choices should create comparable trade-offs rather than
  merely adding maintenance work.

## D-020 — Block assignment and stylized tactics

**Status:** Accepted

The player assigns recruits to troop-type blocks according to their suitability
and then configures strategies for each block. Block strategy controls relevant
ordering and combat behavior, allowing the same collection of soldiers to be
used differently against different opponents.

Tactics may be historically inspired but deliberately simplified or
exaggerated for arcade clarity. Every tactic should provide a strong readable
benefit while creating a meaningful vulnerability or opportunity cost.

Accepted examples:

- **Shield wall for spearmen:** nearly negates archer attacks but gives opposing
  spearmen more room or opportunity in their following attack.
- **Dispersed archers:** reduces the effect of the first cavalry attack but
  requires a corresponding trade-off that remains to be defined.

### Consequences

- Formation preparation has two layers: assign soldiers to suitable blocks,
  then choose block strategies.
- Block strategies can automate detailed ordering at large army scales.
- Espionage can reveal information that makes one strategy attractive without
  guaranteeing it is correct.
- Tactics need visible counters and must not become unconditional upgrades.
- Strategy availability may depend on troop type, leader, training, technology,
  equipment, or place, but those unlock rules remain open.

## D-021 — Three-step individual combat resolution

**Status:** Accepted

A block contains ten ranks, which are processed over successive combat ticks.
The width of the active attack line depends on the formation or army scale and
remains to be specified.

Each opposing soldier pair resolves combat in three steps:

1. **Initiative:** both soldiers roll for initiative, with experience improving
   the chance of winning. Only the initiative winner earns an attack attempt.
2. **Hit:** the winner rolls to hit. The chance combines soldier properties,
   suitability for the assigned position, daily form, and luck.
3. **Consequence:** a successful hit causes a wound or death depending primarily
   on weapon value against armor value. Weapons and armor use a small number of
   quality tiers.

A soldier who failed to act must win initiative in a later exchange before a
counterattack can occur.

### Consequences

- Experience creates an advantage before damage is rolled without guaranteeing
  a hit.
- Correct formation placement matters independently of equipment.
- Daily form and luck create variation between otherwise similar soldiers.
- Weapon and armor quality influence severity rather than initiative.
- The formulas, soldier properties, tie behavior, exchange frequency, quality
  tiers, and exact wound effects remain unresolved.

## D-022 — Wounds, replacement, and attack endurance

**Status:** Refined by D-025

A wounded soldier immediately leaves the current battle and the next available
soldier advances. Wounded survivors recover fully before the next battle.

An experienced soldier may defeat several successive opponents, but cannot
remain at the front indefinitely. After roughly five attack attempts, the
soldier reaches their endurance limit and moves to the back of the block, where
they may eventually cycle forward again.

### Consequences

- A strong or lucky individual can create a visible run of victories.
- Endurance prevents one soldier from defeating an arbitrarily large block.
- Block order changes dynamically as wounded and exhausted soldiers leave the
  active line.
- Wounds have tactical and experience consequences without requiring recovery
  management between battles.
- The exact endurance value, whether misses consume endurance, and how quickly a
  rotated soldier becomes available again remain provisional.

## D-023 — Minimal soldier properties and equipment-defined role

**Status:** Accepted

Alongside accumulated experience and carried equipment, every soldier has four
core personal properties:

- **Talent:** an innate value that does not change;
- **Fitness:** a condition value that can improve or decline;
- **Belonging:** how comfortable and attached the person feels as a soldier in
  their current social and military context;
- **Troop-type aptitude:** an innate suitability for one troop type.

A soldier is not permanently locked to the preferred troop type. Equipment
determines the actual battlefield role, so changing the weapon, armor, mount, or
other relevant loadout can assign a different task. Position suitability is
therefore based primarily on whether the soldier carries the correct equipment
for the assigned block rather than on another positional attribute.

### Consequences

- Talent creates lasting individual identity while fitness provides a changing
  development axis.
- Belonging connects group history to individual combat performance.
- Troop-type aptitude makes natural roles meaningful without forbidding
  retraining or emergency reassignment.
- Equipment affects both combat quality and which role a soldier can perform.
- Exact value ranges and how each property enters combat remain unresolved.

## D-024 — Combat cycles, automatic playback, and the fifty-percent limit

**Status:** Accepted

Each soldier attack is one combat cycle. An attack may fail to produce a hit or
casualty, so opposing soldiers can repeat cycles until one is removed or either
or both become exhausted and rotate out.

A small battle should normally resolve after roughly 10–20 manual "Continue"
advances. Longer battles emerge at higher army levels through more encounters,
failed attacks, exhaustion, and rotation.

The same simulation supports two playback modes:

- manual step-by-step advancement with "Continue";
- automatic advancement with "Start" and "Pause".

An army cannot be completely annihilated in normal battle. The first army to
reach fifty percent losses is defeated and the other army wins. The player may
also retreat earlier under certain conditions, accepting additional losses as
the cost of disengagement.

### Consequences

- Manual and automatic playback must produce equivalent simulation results.
- Automatic playback must pause when a meaningful decision or exceptional event
  occurs.
- Small battles have a short target interaction length while large battles can
  feel materially longer without changing the basic rules.
- Victory preserves survivors on both sides for persistent armies and future
  encounters.
- The definition of a counted loss, cycle parallelism at large scale, playback
  speed, exhaustion resolution, and retreat cost remain unresolved.

## D-025 — Rotation endurance and total exhaustion

**Status:** Accepted

A soldier rotates to the back of the block after roughly five consecutive
attack attempts, as established in D-022. Across repeated appearances, a soldier
can make roughly fifty attack attempts in total during one battle. Reaching that
total makes the soldier exhausted and no longer combat-ready for the remainder
of the battle.

Exhausted soldiers count as losses toward the fifty-percent army defeat
threshold, alongside wounded and killed soldiers.

### Consequences

- Front-line rotation and total battle endurance are separate limits.
- An army can lose through exhaustion without every affected soldier being
  wounded or killed.
- Defensive tactics can pursue victory by exhausting the opponent.
- Five and fifty are provisional balancing targets, not fixed simulation laws.
- Recovery after battle and whether failed attacks consume endurance remain
  unresolved details.

## D-026 — Portrait, edge-to-edge, swipe-based UI

**Status:** Accepted

Use an exclusively portrait mobile layout. On phones the game fills the
available width without outer side margins. Avoid nested indentation, including
in the future army tree, which should remain horizontally flat.

Use a dedicated splash/menu screen for the game title and primary entry points.
Do not repeat the title or a tab bar on every screen. Primary screens are
navigated by horizontal left and right swipes, with a minimal control for
returning to the menu.

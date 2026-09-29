# Process

## The recurring question

When asked **"What comes next?"** or an equivalent question, inspect the actual
project state and respond with exactly four highest-weight topics. These are
large project concerns, not detailed rule questions or an exhaustive backlog.

For each topic, state:

1. why it matters now;
2. whether it needs an owner decision, implementation, or validation;
3. the next concrete step.

Recommend one topic, then let the owner select the topic before presenting
solution choices.

## Ranking concerns

Rank concerns by their current ability to move or block the project:

- **Dependency:** Does other important work depend on it?
- **Uncertainty:** Would answering it remove a major assumption or risk?
- **User impact:** How strongly does it shape the intended experience?
- **Immediacy:** Is it necessary for the next playable increment?
- **Cost of delay:** Will postponing it cause rework or close useful options?

Use qualitative judgment rather than pretending the ranking is mathematically
exact. Explain close or surprising rankings briefly. A concern loses weight
when it is premature, safely reversible, or can be inferred from accepted
requirements.

## Decision cycle

When the recommended step needs owner judgment:

1. Present exactly four high-weight topics and let the owner select one.
2. Only after that selection, give the minimum context needed to decide.
3. Ask exactly one decision question about the selected topic.
4. Present exactly four substantially different solution choices.
5. Describe the effect and main trade-off of each choice.
6. Make a recommendation when the available evidence supports one.
7. Allow the owner to adapt or combine choices instead of treating them as
   a rigid multiple-choice form.
8. Record the result and its consequences after the owner chooses.

Options should describe outcomes or experiences, not obscure implementation
details. Defer questions whose answers would not affect current work.

## Agent autonomy

The agent may make reversible implementation choices, provisional balancing
choices, internal architecture choices, and details already implied by accepted
requirements. Mark consequential assumptions clearly.

Ask the owner about product experience, scope, risk, irreversible direction,
conflicting goals, or decisions that substantially constrain later choices.

## Decision states

- **Provisional:** sufficient for the next experiment, but expected to change.
- **Accepted:** the current project direction until evidence challenges it.
- **Validated:** supported by a playable test or other relevant evidence.

Changing a decision is allowed. Record what changed and which requirements,
decisions, or implementation areas need review.

## Keep the process lightweight

- Keep `NEXT.md` short and current.
- Keep frontier topics strategic; do not promote a detail to the main decision
  while larger unresolved systems still block the project.
- Record conclusions, not entire conversations.
- Avoid speculative detail that does not unlock work.
- Prefer a playable experiment when discussion alone cannot resolve a question.
- Build playable increments as parts of the final game, not as disposable
  prototypes, unless a later decision explicitly justifies an exception.
- Never turn every implementation detail into an owner decision.

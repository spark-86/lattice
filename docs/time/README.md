# Lattice::time::README

## Why time is the answer

Time is something we take for granted. We assume the clock is just how it is and not something we can change. But the whole concept of the Temporal Lattice challenges that idea, that if we have a stable, _calculable_ way to perceive time, we can then use that as the coordinate in the universe to store things.

Everything we know exists in time.

- Actions
- Ideas
- People
- Places
- Things

All have starts and ends. Something within defined boudaries.

## So why not use civil time?

Civil time is broadcast. A handful of operations manage and tell us what time it is. This creates the potential for lying, injection, and general manipulation from the trusted source.

If we move to a **sidereal day**, or what we call a `Turn` to divorce ourselves from the civil calendar, we find we now have a way to calculate time ourselves. We can use instrumentation connected directly to the accepting device so there is no _convincing_ the device what time it is.

Instead now we turn anyone with a telescope into a time source that we can argue about and decide acceptable deltas. Integrating those into the quorum signatures guarantees that the witness of **when** becomes incorruptable. Now that we have when, we can argue who came first, and who is right.

## A Turn explained

A `Turn` is a complete rotation of the planet relative to distant starts. This is ≈ 86,164.0905 seconds, or roughly 23 hours and 56 minutes. This difference of 4 minutes means durning the course of a year, the solar calendar and a sidereal one are off by a full Turn.

Unfortunately as much as us humans like small numbers, computers still kinda suck at floating point math. So in order to keep a Turn as an monotonic integer we need to divide the Turns into 1/1,000,000,00ths, what we call a `Micromark`. Each record in the Temporal Lattice contains the Micromark time from the genesis record (TBD).

> Math to be put here plz and thx.

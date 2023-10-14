# Elevator Optimization

Recently I was in the office talking about fun project ideas with some coworkers, and this idea came up as a fun one that could get complex quickly depending on the approach.  This is also my attempt to kill two birds with one stone and attempt to learn a new language while also tackling a fun problem.

## Problem statement

Essentially, the idea is to identify the optimal resting floor for an elevator platform based on inflow and outflow of a building with potentially many elevators and many floors.  The elevator can be optimized along differing dimensions as well, whether that is energy efficiency, wait time, or perhaps some other dimension we haven't yet considered.

## General problem model

We have the notion of a `Building` which may contain multiple `Floor` and `Elevator` components.  We also have the notion of a `Person` who enters a `Building` at the first floor with a certain probability.  The `Person` then chooses an `Elevator` with a certain probability.  Perhaps we also introduce the notion of a `Door` through which a person may enter a `Building`, and we define a distance between it and an `Elevator`; this presumably would influence the decision made by the `Person` of which `Elevator` to use.  Upon accessing an `Elevator`, a `Person` then travels to a `Floor` with a certain probability.  The `Floor` components of the `Building` then store the `Person` over time, and at each time iteration there is some probability that the `Person` travels to another `Floor`, potentially the first `Floor`, at which person the `Person` may even leave the `Building` with some probability.  Each time the `Elevator` travels from floor to floor, it uses some energy which may vary based on whether it is traveling up or down.  Each time a `Person` requests the `Elevator`, their wait time should be monitored as well.  Energy and wait time are the main concepts that we intend to optimize against for now.
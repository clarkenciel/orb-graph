# orb-graph

This is a tool to help me generate valid layouts for an orb performance.
A valid layout is a collection of performers, each with positions and headings,
that can be made into a fully-connected graph where each node has an indegree and outdegree
of 2. The connection logic is somewhat complicated by the fact that creation of an edge
between two nodes depends on whether the sink node has an available "input" that can be reached
(according to the orientations of the two performers and the arcs of their ears and mouths)
by the source node and whether the source node has a free output.

Generally, a performer can speak at a 60° arc in the direction of their heading and hear at a 60°
arc perpendicular to their heading.

Invalid edges:

1. Sink node cannot "hear" from behind.
```
⮉
↑
⮉
```

2. Sink node cannot "hear" from directly ahead.
```
⮋
↑
⮉
```

Valid edges:

1. Sink node can "hear" from right:
```
⮊ → ⮉
```

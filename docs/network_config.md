### Network Graph Overview

Processes running in Shadow do not have access to the internet; instead, processes running on Shadow virtual hosts utilize an internal routing module to communicate with other processes running on other virtual hosts in the simulation. The routing module is used to position virtual hosts within a network topology, to compute communication paths between virtual hosts, and to enforce network path characteristics like latency and packet loss.

Importantly, the routing module is currently used to _model_ the performance characteristics of internet paths; we do not _simulate_ the behavior of network routers (we do not run routing protocols like [BGP](https://en.wikipedia.org/wiki/Border_Gateway_Protocol)).

This page describes the routing module and how it can be configured.

#### Graph

Shadow represents a network topology over which processes can communicate using a [weighted graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics)). The graph contains _vertices_ that abstractly represent network locations, and _edges_ representing network paths between those locations. Shadow requires that the graph is [connected](https://en.wikipedia.org/wiki/Connectivity_(graph_theory)) such that there exists at least one _path_ (a series of one or more edges) between every pair of vertices.

#### Behavior

The graph encodes network positioning and path characteristics as attributes on the vertices and edges. Shadow uses the connectivity graph along with the information encoded in vertex and edge attributes to:

  - attach virtual hosts to specific vertices (i.e., locations) in the network topology;
  - assign the bandwidth allowed for each attached virtual host;
  - compute the shortest path (weighted by edge `latency`) between two virtual hosts using [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm); and
  - compute the end-to-end latency and packet loss for the shortest path.

The bandwidth of the virtual hosts and the end-to-end latency and packet loss for a shortest path between two virtual hosts are then enforced for all network communication.

#### Important Notes

  - The network graph may be directed or undirected, as long as the graph is structured such that every vertex can reach every other vertex through a series of edges.
  - If the network graph is a [complete graph](https://en.wikipedia.org/wiki/Complete_graph) (there exists a single unique edge between every pair of vertices), then we can avoid running the shortest path algorithm as a performance optimization by setting the [use_shortest_path option](shadow_config.md#networkuse_shortest_path) to `False`.

### Network Graph Attributes

We encode attributes on the vertices and edges that allow for configuring the simulated network characteristics. The attributes and their effect on the simulated network are described in more detail (alongside a simple example graph) on [the network graph attributes page](network_graph_attributes.md).

### Using an Existing Graph

We created a large network graph representing worldwide latencies and bandwidths as of 2018 using the [RIPE Atlas measurement platform](https://atlas.ripe.net). The graph contains network bandwidths and latencies in and between major cities around the world, and is suitable for general usage for most types of Shadow simualtions. The graph is [available for download as a research artifact](https://tmodel-ccs2018.github.io/data/shadow/network/atlas-lossless.201801.shadow113.graphml.xml.xz) and more details about the measurement methodology is available on [the research artifacts site](https://tmodel-ccs2018.github.io).

Note: [the scripts we used to create the graph](http://github.com/shadow/atlas) are also available, but are not recommended for general use. The scripts require advanced knowledge of RIPE Atlas and also require that you possess RIPE Atlas credits to conduct the measurements needed to create a new graph. We recommend using our existing graph linked above instead, which we may periodically update.

### Creating Your Own Graph

The python module [networkx](http://networkx.github.io/) can be used to create and manipulate more complicated graphs.
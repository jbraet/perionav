# Changelog

## v0.1.3

### Added
- strongly connected subcomponents algorithms

## Changed
- move routingoptions to routing::options
- refactored edge implementation
- rename algorithmOptions to RoutingAlgorithmOptions
- replace node index by usize instead of i32

## v0.1.2

### Added
- Use a kdtree to snap nodes
- Create a result as wkt
- make a difference between tower nodes and shape nodes, only use tower nodes for routing

## Changed
- Vehicle permissions changed

## v0.1.1

### Added
- Make Graph a trait so that we can easily replace it with different implementations
- Initial implementation to read an osm file

## v0.1.0

### Added
- Initial implementation
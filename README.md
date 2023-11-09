# joeboids
simple boids implementation in Rust using rayon for multithreading. thoeretically runs anywhere you can define a triangle drawing function.

### joe's TODOs:
- restrict their vision

### issues
- On smaller screens, pingponging happens. This requires a balance of edge avoidance. Too much edge avoidance and it looks unnatural. Too little and boids are able to venture far off screen, building up to max speed and flying off the opposite side. This probably happens less with large screens and more boids. Also pingponging can be broken up with spacebar.

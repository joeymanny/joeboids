

Simple implementation of [boids](https://en.wikipedia.org/wiki/Boids).


If you used just the library from this project it theoretically could run anywhere you can draw triangles. However I couldn't be bother to actually test that out.


Use the U/J keys to decrease/increase how long one `step` takes, use M to set it to None and simulate as fast as possible.

Uses the highly sophisticated method of calling sleep at the end of the step function if it's early so it blocks all other IO during that period. Genius.


This is a silly little project that I used to get more comfortable with Rust, and it has certainly done that.


Use `-h` for help with options (naturally).


Also you can have tiny boids, they're adorable.


The visualize_neighbors feature will choose one boid and draw a line from it to al its neighbors.

The print_timings feature will print how early/late the step function was in relation to its schedule (defaults to 120 fps for some reason--oh well)


# h


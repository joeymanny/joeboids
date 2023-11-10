Simple implementation of [boids](https://en.wikipedia.org/wiki/Boids).


Thoeretically the library could run anywhere the BoidCanvas trait is implemented (anywhere you can draw triangles), however this is just a binary crate with a silly demo.


Use the U/J keys to decrease/increase how long one step if schedules to take, use M to set it to None (using the highly sophisticated method of calling sleep at the end of the step function so it blocks all other IO during that period).


This is a silly little project that I used to get more comfortable with Rust, and it has certainly done that.

# h

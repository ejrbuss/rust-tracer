# rust-tracer
![sample](/screenshots/week1/step16.png)
A simple ray tracer written in Rust based off [Peter Shirley's books.](http://in1weekend.blogspot.ca/2016/01/ray-tracing-in-one-weekend.html).

## Description
This project is a place for me to re-explore some of the topics I learned about in [Uvic's CSC 305](https://web.uvic.ca/calendar2018-01/CDs/CSC/305.html) as well as learn a new language. This is the first coding project I have attempted in [Rust](https://www.rust-lang.org/en-US/).

## Week 1
During my first week of working on this project I implemented the ray tracing features found in *Ray Tracing in One Weekend*. While the ray tracing features were straightforward enough, learning my way around Rust was not without challenges. In particular:
 - Integrating [rayon]() in order to trace in parallel opened a whole can of worms with `[type] cannot be shared safely between threads`
 - Deciding whether to implement Materials and Geometry as `traits`, `closures` or `enums` was a pain point. I started with traits but settles on enums for their simplicity.
 - The camera always has a bug.

 ### Update

 After looking closer at the images being produced by my ray tracer it was clear that there was something wrong in my implementation. The error was in my implementation of random unit sphere vectors.

 At some point I rewrote `vec3.rs` and didn't think carefully about what this function actually needed to do. That implementation looked like this:

 ```rust
 pub fn rand() -> Self {
     loop {
         let v = Vec3::new(rand(), rand(), rand())l
         if v.mag() < 1.0 {
             return v;
         }
     }
 }
 ```

 Which suffers the obvious problem of only producing random vectors in 1/8th of the unit sphere. The new (and hopefully correct) implementation is:

 ```rust
pub fn rand() -> Self {
    loop {
        let v = 2.0 * Vec3::new(rand(), rand(), rand()) - Vec3::ones();
        if v.mag() < 1.0 {
            return v;
        }
    }
}
 ```

 The effect produced by this bug was still interesting, and may be worth reinvestigating as a filter of some kind.
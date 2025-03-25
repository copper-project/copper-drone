## Tutorial: Copper on a Drone Series

This GitHub project supports the video tutorial series on how to use [Copper](https://github.com/copper-project/copper-rs),  
the Rust-based robotics runtime.

From zero to flight, the series walks through building a simple robotics stack—including hardware integration, computer vision, control, communication, and deployment—on a twitchy 3.5" freestyle drone.

The goal is twofold: to build a complete reference project using Copper, and to develop reusable Copper components along the way (e.g., GStreamer, AprilTags).

📺 The full playlist is [here](https://www.youtube.com/playlist?list=PLQ5cuUFIqMOOd1d39ewMUizYaQq0IlohV).  
So far, it contains 9 parts, with matching integration branches in this repository.

| Part   | Title | Branch |
|--------|-------|--------|
| Part 0 | [Building an Autonomous Drone](https://www.youtube.com/watch?v=rjqe8zYpuPQ) | |
| Part 1 | [Cross Compiling Rust to Arm 64bits with musl](https://www.youtube.com/watch?v=YKXkN_sPebQ) | |
| Part 2 | [Generating a base Copper Project](https://www.youtube.com/watch?v=gZtRhV9XIfs) | `git checkout part2` |
| Part 3 | [Copper GStreamer Source + Rerun visualization](https://www.youtube.com/watch?v=KJhfmBhYm7c) | `git checkout part3` |
| Part 4 | [Adaptive Threshold & April Tag Detection](https://www.youtube.com/watch?v=woXVinzIMZ8) | `git checkout part4` |
| Part 5 | [Linux Perf tool & Calibration and relative Pose](https://www.youtube.com/watch?v=tSuZMfMz2tE) | |
| Part 6 | [Controlling a drone running Betaflight with Rust](https://www.youtube.com/watch?v=an8z5nyPkA0) | |
| Part 7 | [Establishing a Control Baseline](https://www.youtube.com/watch?v=jdSA5PJSuPI) | |
| Part 8 | [Integration and maiden flight!](https://www.youtube.com/watch?v=OaVXLcNbNKE) | `git checkout part8` |

💬 Questions about this tutorial or Copper in general?  
Join us on [Discord](https://discord.gg/VkCG7Sb9Kw)!

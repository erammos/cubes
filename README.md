# Cubes

**Cubes** is my first Rust project! This program demonstrates rendering two cubes, where one cube rotates around another using a `bad` **scene graph** structure. The cubes are organized hierarchically, with the second cube attached as a child of the first. This project aims to simulate Unity's approach to scene management, including **MonoBehaviour**-like behavior through Rust's trait system.

---

## Features

- **Scene Graph Hierarchy**
- **Unity-inspired MonoBehaviour Traits**: Implements `Object` traits as a way to define behavior on objects in the scene graph, similar to Unity's `MonoBehaviour`.
- **Basic SDL2 Integration**

---

## Requirements

- **Rust** (version 1.70 or newer)
- **SDL2** (installed on your system)

---

## Installation

### Install SDL2

1. **Linux:**
   ```bash
   sudo apt-get install libsdl2-dev

2. **MacOS**
   ```bash
   brew install sdl2


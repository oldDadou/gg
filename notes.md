# What the game is about

## Technical details
This game will be a sort of advance wars; turn by turn based. I don't know yet
if the turn will be for a character (ff tactics, disgaea, Dofus)
or for the whole team (FE, advance Wars).

The game will be view from top (like zelda). I want to give to the gameplay
an aspect of base building; My first idea is to have neutral building that will
be captured by the player. But we could imagine a way to construct building.  

**Ideally I would have a pvp mode using networks connections.**

# What does an engine need ?

## What can be generic

I don't want to over engineer the game; but I think
that some part of the code might be use-full in a large
set of games.

* The rendering of the map.
* The loading of the scene.

* The game loop, with a smooth time handling.
* The events (Keyboard etc).
* The network.

## What cannot be generic

* The logic of the game

## Engine:
My first Idea is to make the engine be a class that (can) holds a Scene;
If no scene is loaded, then it means that the gamer is in the main menu.

Once a scene is ~~loaded~~ selected, the Engine will do some actions
in the following order:

- Initialize the scene
- While the scene is active
  - Render the logic
  - Draw the scene
- Discard the scene

### A Scene must
  * Know how to draw itself.
  * Know all entities, and how they update.

## A Scene can load a level 

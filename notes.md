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

# SDL2 binding

I can't use it because I think the api os very difficult to understand. To go more in details, my issue
was to that I could not create some kind of game engine. I was not able to use some
pieces of the API. Indeed the textures need to be bounded to a texture creator. It leads to huge lifetime issue.
I decided to stop to use this API till I get more rust skill or the API is more understandable.

# Piston

Oh piston seems rather great, but it is splitted into a lot of submodules which make it difficult to understand in the first
try.

## Sprite
I tried to use the https://github.com/PistonDevelopers/sprite, but this does not looks like
exactly what I need. Event if the code is organized into an abstraction that I might want to have to,
(Sprite; Scene). I don't really like it because It is difficult to create a game based on an ECS.
In addition, I noticed that if I want to render a tiled map (which is one of my goals); I need to defined a sprite
by case, which I don't think it is what I want. It may cost a lot in memory.

So what ? I think I will use piston but I will need to write my own framework.
So the next step is to define what I want my framework to be.


Stanbying question:

For my current situation, I have a world map (which is built from software tiled map). My map is currently an entity with an array of tiles; a texture (the tileset)/.
But need this map in a lot of systems like in the camera_motion_system, (to make sure I do not overflow when I scroll). I then need it to my action_system which needs to find out

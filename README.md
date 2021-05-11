# TODO / Roadmap

## Loading Assets
Add a loading screen and centralized way to load assets and fetch them by name. This
ensures the game doesn't begin before assets are loaded.
Here is sample of asset loading in bevy:

https://bevy-cheatbook.github.io/cookbook/assets-ready.html

## Adding a generic weapon and bullet component
This is what I'm currently working on

Allow component to be added to an entity, uses entities position as starting place and angle of bullets

## Learn Bevy "States"
Setup states to transition from Main Menu/Loading/Game/Game Over

## Enemies
Currently only a simple follow enemy exists
Desire enemies that shoot at player.
Also welcome creative enemies. Teleport enemies, gravity well enemies, etc...

## Rooms / Map
Add concept of a Room and Map to the State machine to determine when to respawn new enemies
and determine player progression
 
# Resources

## Bevy Game Engine
https://bevyengine.org/

## Rapier Physics
For this we use the bevy_rapier_2d plugin that attemps to make the rapier physics library more compatible with bevy.

https://rapier.rs/

## Bevy Cheatsheet
https://bevy-cheatbook.github.io/


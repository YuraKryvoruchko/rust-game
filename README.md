<h1>Table of Contents</h1>

- [1. About](#1-about)
- [2. Project goals](#2-project-goals)
- [3. Style](#3-style)
  - [3.1. Main menu](#31-main-menu)
  - [3.2. Gameplay scene](#32-gameplay-scene)
  - [3.3. References](#33-references)
- [4. Mechanics](#4-mechanics)
  - [4.1. Key binding](#41-key-binding)
  - [4.2. Game characteristic values](#42-game-characteristic-values)
  - [4.3. Player movement](#43-player-movement)
  - [4.4. Asteroid movement](#44-asteroid-movement)
  - [4.5. Asteroid spawning](#45-asteroid-spawning)
  - [4.6. Shooting](#46-shooting)
  - [4.7. Collision](#47-collision)
  - [4.8. Health system](#48-health-system)
  - [4.9. Score counting](#49-score-counting)
  - [4.10. Saving of best result](#410-saving-of-best-result)
- [5. Used resources](#5-used-resources)
- [6. Results](#6-results)

# 1. About
- Program type: game;
- Genre: space-shooter;
- Engine: [Bevy](https://bevy.org/)
- Platform: Windows.

# 2. Project goals
1. Gain knowledge and practice in the Rust language and the use of the ECS architecture pattern in game development.
2. Develop a simple 2D space shooter game.

# 3. Style
It's 2D game in casual style. The game has a main menu and a gameplay scene.
1. **Player** - a spacecraft that can shoot lasers;
2. **Asteroid** - an enemy of the player. The player have to destroy it with his laser.

## 3.1. Main menu
In the main menu there are a button **"Start game"** which you can press to go to the **Gameplay scene** and a button **"Exit"** which you can press to exit the game.<br>
There is also a best score text label that displays your best score in the game.

## 3.2. Gameplay scene
In the gameplay, the player is at the bottom of the screen, and the asteroids are moving at us from top to bottom.<br>
The upper left corner of the screen displays the amount of health of the player. In the upper right corner is the current score.

## 3.3. References

![](images/example1.png)

---

![](images/example2.png)

# 4. Mechanics
1. - [x] Player movement
2. - [x] Asteroid movement
3. - [x] Asteroid spawning
4. - [x] Shooting
5. - [ ] Collision
6. - [ ] Health system
7. - [x] Score counting
8. - [ ] Saving of best result

## 4.1. Key binding
| Action     |  Key  |
| :--------- | :---: |
| Move left  |   A   |
| Move right |   D   |
| Shoot      | Space |

## 4.2. Game characteristic values
| Characteristic        | Value |
| :-------------------- | :---: |
| Player speed          |   1   |
| Asteroid speed        |   1   |
| Laser speed           |   1   |
| Amount of health      |   3   |
| Damage of asteroid    |   1   |
| Shooting rate         |  0.2  |
| Asteroid spawn rate   |   3   |
| Score by one asteroid |   5   |

## 4.3. Player movement
When player click **[Move left]** or **[Move right]** then the player moves in the corresponding direction with the speed ***[Player speed]***.

## 4.4. Asteroid movement
The asteroid moves from top to bottom at a constant speed ***[Asteroid speed]***.

## 4.5. Asteroid spawning
Asteroids appear at the top of the screen with a ***[Asteroid spawn rate]***.

## 4.6. Shooting
When the player press **[Shoot]** action then a laser is created at the end of the ship and moves at a constant speed ***[Laser speed]***. The player can shoot with a certain time interval ***[Shooting rate]***.

## 4.7. Collision
The player, lasers and asteroids must collide with each other as well as with the edges of the screen.

## 4.8. Health system
When an asteroid collides with the player's ship or the bottom of the screen, the player is damaged and loses health by ***[Damage of asteroid]***.<br>
When the health is 0, then the game is over.

## 4.9. Score counting
When the player destroy an asteroid by laser his get ***[Score by one asteroid]***.

## 4.10. Saving of best result
When the game ends, the result is saved if it is a record.

# 5. Used resources
1. [Game engine Bavy](https://bevy.org/)
2. [Space Shooter Redux by KenneY](https://kenney.nl/assets/space-shooter-redux)

# 6. Results
Work in progress.

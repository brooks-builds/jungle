# jungle

![Rust](https://github.com/brooks-builds/jungle/workflows/Rust/badge.svg)

This game is inspired by [Pitfall]() from the Atari 2600.

## Design Doc

- Single Player
- Start screen
  - Instructions
    - Controls
      - left
      - right
      - jump
      - climb ladder
      - descend ladder
  - Options
    - set resolution
    - fullscreen or not
    - freeplay mode on/off
- Pause screen
  - Show instructions
  - Show relics collected
- Gameplay
  - Timer (20:00)
  - life system
    - 3 lives
  - Score
  - overworld
    - Can move left or right
    - empty (start)
    - tar pit
    - barrels
    - rope swing
    - lake
    - alligators
    - snake
    - fire
    - relic
    - pit
    - ladder
  - caves
    - snakes (animated)
    - scorpion (animated, moves)
    - stalagmites/ stalactites (static)
    - caves should end at some point
- Game over screen
  - Summary of gameplay
    - how did the player die?
    - how many points?
      1000 points per relic
      1000 points per second of time left
    - what relics were collected?
  - High score screen
    - top 10 scores
    - Adding in name for high score

## Questions

- Should we support freeplay (no timer?)
  - yes, but no score
- Do we want a internet leaderboard?
  - no if ggez
  - yes if web based but also have a tab for same machine
- How many relics do we want to have?
  - at least 10
  - at most 20
- What should the relics be?
  1. Golf course logos
  2. Rust logo
  3. Xilbë
  4. German Shepherd dog
  5. Wildwood sign
  6. heart
  7. sword
- Do we want to be able to gain lives?
  - no
- What devices is the game going to be played on?
  - Windows Surface tablet
    - can run ggez games?
  - iPhone browser
    - support web assembly and canvas?
  - Rasberry Pi
- What technology to use?
  - Rust + GGEZ
  - Rust + Web assembly
  - P5.js
  - Vanilla.js + Canvas
  - Rust + Bevy
- Should I buy a keyboad / controller to send with the game?
  - [Nintendo classic controller](https://www.amazon.com/Controller-suily-Joystick-RetroPie-Emulators/dp/B07M7SYX11/ref=pd_sbs_63_2/146-0131105-8454120?_encoding=UTF8&pd_rd_i=B07M7SYX11&pd_rd_r=16c80660-78d2-4276-bd9c-6f2f2e41932d&pd_rd_w=oHoCO&pd_rd_wg=x0IvG&pf_rd_p=cc0adad9-73a2-470d-acda-37a71f8758ba&pf_rd_r=1KKRQ5XD16ERJC5ES6HH&psc=1&refRID=1KKRQ5XD16ERJC5ES6HH)
- Do we need a certificate if we are building for Windows?
  - no, but it would be nice

## Architecture

- Component based architecture
- Game design file JSON / RON format
  - speed of player
  - jump speed/gravity
  - ## map

```json
{
  "resolution": {
    "width": 1920.0,
    "height": 1080.0
  },
  "player": {
    "speed": 5.0,
    "actions": {
      "jump": {
        "controlls": {
          "keyboard": 76,
          "controller": 1
        },
        "jump_velocity": -10.0,
        "sprite": {
          "type": "image",
          "filename": "player-jumping.png",
          "name": "player_jumping"
        },
        "sound": "player_jumping.ogg"
      },
      "run_right": {
        "controlls": {
          "keyboard": 72,
          "controller": 14
        },
        "velocity": 15.0,
        "sprite": {
          "type": "spritesheet",
          "filename": "player-run-right.png",
          "name": "player_run_right",
          "count": 3
        }
      }
    }
  },
  "map": [
    {
      "overworld": ["barrel"],
      "cave": ["end-right", "stairs"]
    },
    {
      "overworld": ["barrels-moving"],
      "cave": ["end-left"]
    }
  ],
  "assets_folder": "assets",
  "items": {
    "barrel": {
      "sprite": {
        "type": "image",
        "filename": "barrel.png",
        "name": "barrel"
      },
      "location": {
        "x": 1800.0,
        "y": 500.0
      },
      "type": "enemy"
    },
    "barrels-moving": {
      "sprite": {
        "type": "spritesheet",
        "filename": "barrel-moving.png",
        "name": "barrel_moving",
        "count": 2
      },
      "location": {
        "x": 1940.0,
        "y": 500.0
      },
      "velocity": {
        "x": -5.0,
        "y": 0.0
      },
      "type": "enemy"
    },
    "end-right": {
      "location": {
        "x": 800.0,
        "y": 900.0
      },
      "type": "background"
    },
    "stairs": {
      "sprite": {
        "type": "image",
        "filename": "stairs.png",
        "name": "stairs"
      },
      "location": {
        "x": 400.0,
        "y": 600.0
      },
      "type": "background"
    }
  }
}``
```

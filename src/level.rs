use bevy::math::*;
use bevy::prelude::*;

use crate::proc::*;
use crate::room::*;

pub fn new() -> LevelPrototype {
    let r1a = RoomPrototype {
        name: "Bedroom".to_string(),
        description: "I should get some soy milk.".to_string(),
        color: Color::rgb(1.0, 1.0, 1.0),
        width: 12.0,
        height: 2.0,
        depth: 8.0,
        doors: vec![Door::East].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 1,
            from: Door::East,
            to: Door::West,
        }],
        props: vec![
            PropPrototype {
                name: "bed".to_string(),
                position: Vec2::new(4.5, -2.5),
                rotation: 0.0,
            },
            PropPrototype {
                name: "desk".to_string(),
                position: Vec2::new(-1.5, -3.5),
                rotation: 270.0_f32.to_radians(),
            },
            PropPrototype {
                name: "chair".to_string(),
                position: Vec2::new(-1.3, -2.4),
                rotation: 0.0,
            },
        ],
    };
    let r2 = RoomPrototype {
        name: "Corridor".to_string(),
        description: "Shoes, keys, wallet, phone, got everything.".to_string(),
        color: Color::rgb(1.0, 1.0, 1.0),
        width: 2.0,
        height: 2.0,
        depth: 8.0,
        doors: vec![Door::North].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 2,
            from: Door::North,
            to: Door::West,
        }],
        props: vec![],
    };
    let r1b = RoomPrototype {
        name: r1a.name.clone(),
        description: "I should get some soy milk.".to_string(),
        color: Color::rgb(1.0, 1.0, 1.0),
        width: r1a.width,
        height: r1a.height,
        depth: r1a.depth,
        doors: vec![Door::East].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 3,
            from: Door::East,
            to: Door::West,
        }],
        props: r1a.props.clone(),
    };
    let r3a = RoomPrototype {
        name: "Abjection".to_string(),
        description: "Shoes, keys... This isn't my corridor.".to_string(),
        color: Color::rgb(0.9, 1.0, 1.0),
        width: 4.0,
        height: 2.0,
        depth: 4.0,
        doors: vec![Door::East, Door::South].into_iter().collect(),
        edges: vec![
            EdgePrototype {
                index: 4,
                from: Door::East,
                to: Door::North,
            },
            EdgePrototype {
                index: 5,
                from: Door::South,
                to: Door::North,
            },
        ],
        props: vec![PropPrototype {
            name: "lamp".to_string(),
            position: Vec2::new(-1.8, -1.8),
            rotation: 0.0,
        }],
    };
    let r3b = RoomPrototype {
        name: r3a.name.clone(),
        description: "This still isn't my corridor.".to_string(),
        color: Color::rgb(0.9, 1.0, 1.0),
        width: r3a.width,
        height: r3a.height,
        depth: r3a.depth,
        doors: vec![Door::East].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 3,
            from: Door::East,
            to: Door::North,
        }],
        props: r3a.props.clone(),
    };
    let r4a = RoomPrototype {
        name: "Gloom".to_string(),
        description: "Huh, nice flower.".to_string(),
        color: Color::rgb(0.9, 1.0, 1.0),
        width: 8.0,
        height: 2.0,
        depth: 8.0,
        doors: vec![Door::East, Door::West].into_iter().collect(),
        edges: vec![
            EdgePrototype {
                index: 3,
                from: Door::East,
                to: Door::West,
            },
            EdgePrototype {
                index: 6,
                from: Door::West,
                to: Door::North,
            },
        ],
        props: vec![PropPrototype {
            name: "flower_table".to_string(),
            position: Vec2::zero(),
            rotation: 0.0,
        }],
    };
    let r4b = RoomPrototype {
        name: r4a.name.clone(),
        description: "Huh, lovely flower.".to_string(),
        color: Color::rgb(0.9, 1.0, 1.0),
        width: r4a.width,
        height: r4a.height,
        depth: r4a.depth,
        doors: vec![Door::East, Door::West].into_iter().collect(),
        edges: vec![
            EdgePrototype {
                index: 3,
                from: Door::East,
                to: Door::West,
            },
            EdgePrototype {
                index: 7,
                from: Door::West,
                to: Door::North,
            },
        ],
        props: r4a.props.clone(),
    };
    let r4c = RoomPrototype {
        name: r4a.name.clone(),
        description: "Huh, pretty flower.".to_string(),
        color: Color::rgb(0.9, 1.0, 1.0),
        width: r4a.width,
        height: r4a.height,
        depth: r4a.depth,
        doors: vec![Door::East, Door::West].into_iter().collect(),
        edges: vec![
            EdgePrototype {
                index: 3,
                from: Door::East,
                to: Door::West,
            },
            EdgePrototype {
                index: 8,
                from: Door::West,
                to: Door::North,
            },
        ],
        props: r4a.props.clone(),
    };
    let r4d = RoomPrototype {
        name: r4a.name.clone(),
        description: "Huh, flower.".to_string(),
        color: Color::rgb(0.9, 1.0, 1.0),
        width: r4a.width,
        height: r4a.height,
        depth: r4a.depth,
        doors: vec![Door::East, Door::West].into_iter().collect(),
        edges: vec![
            EdgePrototype {
                index: 12,
                from: Door::East,
                to: Door::West,
            },
            EdgePrototype {
                index: 9,
                from: Door::West,
                to: Door::North,
            },
        ],
        props: r4a.props.clone(),
    };
    let r4e = RoomPrototype {
        name: r4a.name.clone(),
        description: "Huh, ugly flower.".to_string(),
        color: Color::rgb(0.9, 1.0, 1.0),
        width: r4a.width,
        height: r4a.height,
        depth: r4a.depth,
        doors: vec![Door::East, Door::West].into_iter().collect(),
        edges: vec![
            EdgePrototype {
                index: 3,
                from: Door::East,
                to: Door::West,
            },
            EdgePrototype {
                index: 10,
                from: Door::West,
                to: Door::North,
            },
        ],
        props: r4a.props.clone(),
    };
    let r4f = RoomPrototype {
        name: r4a.name.clone(),
        description: "Huh, terrible flower.".to_string(),
        color: Color::rgb(0.9, 1.0, 1.0),
        width: r4a.width,
        height: r4a.height,
        depth: r4a.depth,
        doors: vec![Door::East, Door::West].into_iter().collect(),
        edges: vec![
            EdgePrototype {
                index: 3,
                from: Door::East,
                to: Door::West,
            },
            EdgePrototype {
                index: 11,
                from: Door::West,
                to: Door::North,
            },
        ],
        props: r4a.props.clone(),
    };
    let r4g = RoomPrototype {
        name: r4a.name.clone(),
        description: "Huh, florpy flower.".to_string(),
        color: Color::rgb(0.9, 1.0, 1.0),
        width: r4a.width,
        height: r4a.height,
        depth: r4a.depth,
        doors: vec![Door::East, Door::West].into_iter().collect(),
        edges: vec![
            EdgePrototype {
                index: 3,
                from: Door::East,
                to: Door::West,
            },
            EdgePrototype {
                index: 5,
                from: Door::West,
                to: Door::North,
            },
        ],
        props: r4a.props.clone(),
    };
    let r5a = RoomPrototype {
        name: "Betterment".to_string(),
        description: "Now where was I? Soy milk.".to_string(),
        color: Color::rgb(0.775, 1.0, 1.0),
        width: 32.0,
        height: 2.0,
        depth: 3.0,
        doors: vec![Door::East].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 13,
            from: Door::East,
            to: Door::North,
        }],
        props: vec![],
    };
    let r6 = RoomPrototype {
        name: "Turning".to_string(),
        description: "Is this the store?.".to_string(),
        color: Color::rgb(0.775, 1.0, 1.0),
        width: 6.0,
        height: 2.0,
        depth: 6.0,
        doors: vec![Door::West].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 14,
            from: Door::West,
            to: Door::East,
        }],
        props: vec![PropPrototype {
            name: "a".to_string(),
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
        }],
    };
    let r7 = RoomPrototype {
        name: "Turning".to_string(),
        description: "Is this the store?.".to_string(),
        color: Color::rgb(0.775, 1.0, 1.0),
        width: 6.0,
        height: 2.0,
        depth: 6.0,
        doors: vec![Door::North].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 15,
            from: Door::North,
            to: Door::South,
        }],
        props: vec![PropPrototype {
            name: "b".to_string(),
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
        }],
    };
    let r8 = RoomPrototype {
        name: "Turning".to_string(),
        description: "Is this the store?.".to_string(),
        color: Color::rgb(0.775, 1.0, 1.0),
        width: 6.0,
        height: 2.0,
        depth: 6.0,
        doors: vec![Door::East].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 16,
            from: Door::East,
            to: Door::West,
        }],
        props: vec![PropPrototype {
            name: "c".to_string(),
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
        }],
    };
    let r9 = RoomPrototype {
        name: "Turning".to_string(),
        description: "Is this the store?.".to_string(),
        color: Color::rgb(0.775, 1.0, 1.0),
        width: 6.0,
        height: 2.0,
        depth: 6.0,
        doors: vec![Door::South].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 17,
            from: Door::South,
            to: Door::North,
        }],
        props: vec![],
    };
    let r10 = RoomPrototype {
        name: "Turning".to_string(),
        description: "Is this the store?.".to_string(),
        color: Color::rgb(0.775, 1.0, 1.0),
        width: 6.0,
        height: 2.0,
        depth: 6.0,
        doors: vec![Door::West].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 18,
            from: Door::West,
            to: Door::East,
        }],
        props: vec![PropPrototype {
            name: "e".to_string(),
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
        }],
    };
    let r5b = RoomPrototype {
        name: r5a.name.clone(),
        description: r5a.description.clone(),
        color: Color::rgb(0.775, 1.0, 1.0),
        width: r5a.width,
        height: r5a.height,
        depth: r5a.depth,
        doors: vec![Door::West].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 19,
            from: Door::West,
            to: Door::North,
        }],
        props: r5a.props.clone(),
    };
    let r11 = RoomPrototype {
        name: "Mistakes".to_string(),
        description: "I keep making the same mistakes.".to_string(),
        color: Color::rgb(0.44, 1.0, 1.0),
        width: 12.0,
        height: 2.0,
        depth: 12.0,
        doors: vec![Door::South].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 20,
            from: Door::South,
            to: Door::East,
        }],
        props: vec![PropPrototype {
            name: "rev_chair".to_string(),
            position: Vec2::zero(),
            rotation: 0.0,
        }],
    };
    let r90a = RoomPrototype {
        name: "Repetition".to_string(),
        description: "Every time.".to_string(),
        color: Color::rgb(0.44, 1.0, 1.0),
        width: 12.0,
        height: 2.0,
        depth: 8.0,
        doors: vec![Door::West].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 21,
            from: Door::West,
            to: Door::North,
        }],
        props: vec![PropPrototype {
            name: "mobius".to_string(),
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
        }],
    };
    let r12 = RoomPrototype {
        name: "Thoughts".to_string(),
        description: "But it doesn't matter.".to_string(),
        color: Color::rgb(0.44, 1.0, 1.0),
        width: 10.0,
        height: 2.0,
        depth: 10.0,
        doors: vec![Door::South].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 22,
            from: Door::South,
            to: Door::North,
        }],
        props: vec![],
    };
    let r13 = RoomPrototype {
        name: "Memories".to_string(),
        description: "Because it gets better.".to_string(),
        color: Color::rgb(0.44, 1.0, 1.0),
        width: 10.0,
        height: 2.0,
        depth: 10.0,
        doors: vec![Door::South].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 23,
            from: Door::South,
            to: Door::North,
        }],
        props: vec![PropPrototype {
            name: "room".to_string(),
            position: Vec2::zero(),
            rotation: 0.0,
        }],
    };
    let r90b = RoomPrototype {
        name: r90a.name.clone(),
        description: r90a.description.clone(),
        color: Color::rgb(0.44, 1.0, 1.0),
        width: r90a.width,
        height: r90a.height,
        depth: r90a.depth,
        doors: vec![Door::South].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 24,
            from: Door::South,
            to: Door::East,
        }],
        props: r90a.props.clone(),
    };
    let r14 = RoomPrototype {
        name: "Content".to_string(),
        description: "I am not happy, but I am content.".to_string(),
        color: Color::rgb(0.0, 1.0, 1.0),
        width: r1a.width,
        height: r1a.height,
        depth: r1a.depth,
        doors: vec![].into_iter().collect(),
        edges: vec![],
        props: r1a.props.clone(),
    };
    LevelPrototype {
        start: 0,
        rooms: vec![
            r1a, r2, r1b, r3a, r3b, r4a, r4b, r4c, r4d, r4e, r4f, r4g, r5a, r6, r7, r8, r9, r10,
            r5b, r11, r90a, r12, r13, r90b, r14,
        ],
    }
}

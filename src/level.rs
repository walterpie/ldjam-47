use bevy::math::*;
use bevy::prelude::*;

use crate::proc::*;
use crate::room::*;

pub fn new() -> LevelPrototype {
    let r1a = RoomPrototype {
        name: "Bedroom".to_string(),
        color: Color::rgb(1.0, 1.0, 1.0),
        width: 12.0,
        height: 2.0,
        depth: 6.0,
        doors: vec![Door::East].into_iter().collect(),
        edges: vec![EdgePrototype {
            index: 1,
            from: Door::East,
            to: Door::West,
        }],
        props: vec![PropPrototype {
            name: "bed".to_string(),
            position: Vec2::new(4.5, -1.5),
            rotation: 0.0,
        }],
    };
    let r2 = RoomPrototype {
        name: "Corridor".to_string(),
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
        props: vec![],
    };
    let r3b = RoomPrototype {
        name: r3a.name.clone(),
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
        color: Color::rgb(0.775, 1.0, 1.0),
        width: 64.0,
        height: 2.0,
        depth: 4.0,
        doors: vec![].into_iter().collect(),
        edges: vec![],
        props: vec![],
    };
    LevelPrototype {
        start: 0,
        rooms: vec![
            r1a, r2, r1b, r3a, r3b, r4a, r4b, r4c, r4d, r4e, r4f, r4g, r5a,
        ],
    }
}

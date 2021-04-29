use self::VaultRoom::*;
use crate::constants::FIFTEEN_BIT_MODULO;

const VAULT_DOOR_VAL: u16 = 30;
const VAULT_WIDTH: usize = 4;
const VAULT_MAP: [VaultRoom; 16] = [
    MulOp,
    Val(8),
    SubOp,
    End(1),
    Val(4),
    MulOp,
    Val(11),
    MulOp,
    AddOp,
    Val(4),
    SubOp,
    Val(18),
    Start(22),
    SubOp,
    Val(9),
    MulOp,
];

#[derive(Clone, Copy, Debug)]
enum VaultRoom {
    Start(u16),
    End(u16),
    Val(u16),
    AddOp,
    SubOp,
    MulOp,
}

impl VaultRoom {
    fn apply(&self, a: u16, b: u16) -> u16 {
        match self {
            AddOp => a.wrapping_add(b) % FIFTEEN_BIT_MODULO,
            SubOp => a.wrapping_sub(b) % FIFTEEN_BIT_MODULO,
            MulOp => a.wrapping_mul(b) % FIFTEEN_BIT_MODULO,
            _ => panic!("Room supplied was not an operator room!: {:?}", self),
        }
    }

    fn from_id(id: usize) -> Option<VaultRoom> {
        VAULT_MAP.get(id).cloned()
    }
}

#[derive(Clone, Debug)]
pub struct TravelLog {
    orb_val: u16,
    visited: Vec<usize>,
    path: Vec<String>,
}

impl TravelLog {
    fn new() -> Self {
        TravelLog {
            orb_val: 22,
            visited: vec![12],
            path: vec![],
        }
    }

    fn nth_room(&self, i: usize) -> Option<VaultRoom> {
        self.visited.get(i).map(|&i| VAULT_MAP[i])
    }

    fn current_room(&self) -> Option<VaultRoom> {
        if self.visited.len() == 0 {
            VAULT_MAP.get(12).cloned()
        } else {
            self.nth_room(self.visited.len().saturating_sub(1))
        }
    }

    fn prev_room(&self) -> Option<VaultRoom> {
        if self.visited.len() < 2 {
            None
        } else {
            self.nth_room(self.visited.len().saturating_sub(2))
        }
    }

    fn print_path(&self) {
        let path = self
            .path
            .iter()
            .fold("Start".to_string(), |acc, x| format!("{}\n-> {}", acc, x));

        println!("{}", path);
    }
}

pub fn solve_vault(max_depth: usize) {
    let log = TravelLog::new();

    if let Some(successful_log) = pathfind(log, max_depth) {
        successful_log.print_path();
    } else {
        println!("Unable to solve vault with current configuration");
    }
}

pub fn pathfind(mut log: TravelLog, depth: usize) -> Option<TravelLog> {
    if depth == 0 {
        return None;
    }

    if let Some(End(x)) = log.current_room() {
        let op = log.prev_room().expect("No previous room when at End?");
        log.orb_val = op.apply(log.orb_val, x);

        if log.orb_val == VAULT_DOOR_VAL {
            return Some(log);
        }
    }

    let room_id = log.visited[log.visited.len() - 1];
    let rooms_to_visit: Vec<(String, usize)> = [
        north_of(room_id),
        east_of(room_id),
        south_of(room_id),
        west_of(room_id),
    ]
    .iter()
    .filter_map(|x| x.clone())
    .collect::<Vec<_>>();

    rooms_to_visit
        .into_iter()
        .fold(None, |acc, (direction, id)| {
            acc.or_else(|| visit_room(&log, depth, id, &direction))
        })
}

fn north_of(room_id: usize) -> Option<(String, usize)> {
    room_id
        .checked_sub(VAULT_WIDTH)
        .map(|id| ("north".into(), id))
}

fn east_of(room_id: usize) -> Option<(String, usize)> {
    if room_id % VAULT_WIDTH != VAULT_WIDTH - 1 {
        room_id.checked_add(1).map(|id| ("east".into(), id))
    } else {
        None
    }
}

fn south_of(room_id: usize) -> Option<(String, usize)> {
    room_id
        .checked_add(VAULT_WIDTH)
        .map(|id| ("south".into(), id))
}

fn west_of(room_id: usize) -> Option<(String, usize)> {
    if room_id % VAULT_WIDTH != 0 {
        room_id.checked_sub(1).map(|id| ("west".into(), id))
    } else {
        None
    }
}

fn visit_room(log: &TravelLog, depth: usize, room_id: usize, direction: &str) -> Option<TravelLog> {
    let mut log = log.clone();
    log.visited.push(room_id);
    log.path.push(direction.into());

    VaultRoom::from_id(room_id)
        .map(|room| match room {
            Start(_) => None,
            End(x) => {
                let op = log.prev_room().expect("No previous room when at End?");
                log.orb_val = op.apply(log.orb_val, x);

                if log.orb_val == VAULT_DOOR_VAL {
                    Some(log)
                } else {
                    None
                }
            }
            Val(x) => {
                let op = log.prev_room().expect("No previous room when at Val?");
                log.orb_val = op.apply(log.orb_val, x);

                pathfind(log, depth - 1)
            }
            _ => pathfind(log, depth - 1),
        })
        .flatten()
}

// -------------------------------------------------------------------------------------------------

// use std::{collections::HashMap, sync::Arc};

// type RoomValue = u16;
// type RoomId = usize;

// #[derive(Clone, Hash, PartialEq, Eq)]
// enum VaultRoom {
//     Start(u16),
//     End(u16),
//     Value(u16),
//     AddOperator,
//     SubOperator,
//     MulOperator,
// }

// impl VaultRoom {
//     fn record_new_log(self, log: &TravelLog) -> TravelLog {
//         use self::VaultRoom::*;

//         if let Start(x) = self {
//             return TravelLog {
//                 orb_value: x,
//                 prev_room: None,
//                 rooms_traveled: 0,
//             };
//         }

//         if let End(x) = self {}

//         if let Value(v) = self {
//             let new_orb_val = match log.prev_room {
//                 Some(AddOperator) => log.orb_value + v,
//                 Some(SubOperator) => log.orb_value - v,
//                 Some(MulOperator) => log.orb_value * v,
//                 _ => log.orb_value,
//             };
//             return TravelLog {
//                 orb_value: new_orb_val,
//                 prev_room: Some(self.clone()),
//                 rooms_traveled: log.rooms_traveled + 1,
//             };
//         }

//         return TravelLog {
//             orb_value: log.orb_value,
//             prev_room: Some(self.clone()),
//             rooms_traveled: log.rooms_traveled + 1,
//         };
//     }
// }

// struct TravelLog {
//     orb_value: u16,
//     prev_room: Option<VaultRoom>,
//     rooms_traveled: usize,
// }

// struct VaultConfiguration {
//     target: u16,
//     kinds: Vec<VaultRoom>,
//     values: Vec<RoomValue>,
// }

// pub fn solve_vault() -> String {
//     use self::VaultRoom::*;

//     "".into()
// }

// -------------------------------------------------------------------------------------------------

// #[derive(Clone)]
// struct TravelLog {
//     path: String,
//     orb_value: u16,
//     nodes_traveled: usize,
//     current_node: NodeValue,
//     access_delta: u16,
//     explored: bool,
// }
//
// struct VaultConfiguration {
//     node_map: HashMap<(u16, NodeKind), VaultNode>,
//     travel_map: Vec<TravelLog>,
// }
//
// pub fn solve_vault() -> String {
//     unimplemented!()
// }
//
// fn find_shortest_path(vault: VaultConfiguration) -> Option<String> {
//     loop {
//         vault
//             .travel_map
//             .iter()
//             .filter(|x| !x.explored)
//             .fold(
//                 None,
//                 |acc, log| {
//                     if acc.is_none() {
//                         Some(log)
//                     } else {
//                         None
//                     }
//                 },
//             )
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::{NodeKind::*, *};
//
//     #[test]
//     fn finds_only_possible_route() {
//         let configuration = VaultConfiguration {
//             node_map: [
//                 (
//                     (3, Start),
//                     VaultNode {
//                         kind: Start,
//                         value: 3,
//                         transitions: vec![(2, Arc::new(|x, y| (x + 2, format!("{} + 2", y))))],
//                     },
//                 ),
//                 (
//                     (2, Adjustor),
//                     VaultNode {
//                         kind: Adjustor,
//                         value: 2,
//                         transitions: vec![(1, Arc::new(|x, y| (x - 1, format!("{} - 1", y))))],
//                     },
//                 ),
//                 (
//                     (1, End),
//                     VaultNode {
//                         kind: End,
//                         value: 1,
//                         transitions: vec![],
//                     },
//                 ),
//             ]
//             .iter()
//             .cloned()
//             .collect(),
//
//             travel_map: [TravelLog {
//                 path: "3".into(),
//                 orb_value: 3,
//                 nodes_traveled: 0,
//                 current_node: 3,
//                 access_delta: 0,
//                 explored: false,
//             }]
//             .iter()
//             .cloned()
//             .collect(),
//         };
//
//         assert_eq!(
//             find_shortest_path(configuration),
//             Some("3 + 2 - 1".to_string())
//         );
//     }
// }

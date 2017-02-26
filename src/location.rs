/* Copyright 2017 David Li.

 * This file is part of BigRedRPG.

 * BigRedRPG is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.

 * BigRedRPG is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.

 * You should have received a copy of the GNU Affero General Public License
 * along with BigRedRPG.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::collections::{BinaryHeap, HashMap, HashSet};

use student::{Student, StudentId};

pub struct Location {
    pub name: String,
    pub id: LocationId,

    pub students: Vec<Student>,
}

#[derive(Copy,Clone,Debug,Eq,Hash,PartialEq)]
pub struct LocationId(i32);

impl Location {
    pub fn new<S: Into<String>>(name: S, id: LocationId) -> Location {
        Location {
            name: name.into(),
            id: id,

            students: Vec::new(),
        }
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn remove_student_by_id(&mut self, id: StudentId) -> Student {
        let mut idx = None;
        for (index, student) in self.students.iter().enumerate() {
            if student.id == id {
                idx = Some(index);
            }
        }

        self.students.remove(idx.expect(&format!("Could not remove student {}", id)))
    }
}

pub struct LocationIdGenerator(i32);

impl LocationIdGenerator {
    pub fn new_from_index(index: i32) -> LocationIdGenerator {
        LocationIdGenerator(index)
    }

    pub fn new_id(&mut self) -> LocationId {
        let id = LocationId(self.0);
        self.0 += 1;

        id
    }
}

pub struct Map {
    locations: HashMap<LocationId, Location>,
    junctions: HashMap<LocationId, Vec<LocationId>>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            locations: HashMap::new(),
            junctions: HashMap::new(),
        }
    }

    pub fn add(&mut self, location: Location) -> LocationId {
        let id = location.id;
        self.locations.insert(id, location);
        self.junctions.insert(id, Vec::new());

        id
    }

    pub fn add_junction(&mut self, location1: LocationId, location2: LocationId) {
        self.junctions.get_mut(&location1).unwrap().push(location2);
        self.junctions.get_mut(&location2).unwrap().push(location1);
    }

    pub fn get(&self, id: LocationId) -> Option<&Location> {
        self.locations.get(&id)
    }

    pub fn get_mut(&mut self, id: LocationId) -> Option<&mut Location> {
        self.locations.get_mut(&id)
    }

    pub fn iter(&self) -> ::std::collections::hash_map::Iter<LocationId, Location> {
        self.locations.iter()
    }

    pub fn iter_mut(&mut self) -> ::std::collections::hash_map::IterMut<LocationId, Location> {
        self.locations.iter_mut()
    }

    pub fn find_path(&self, start: LocationId, end: LocationId) -> Option<Vec<LocationId>> {
        // https://en.wikipedia.org/wiki/Dijkstra's_algorithm#Pseudocode
        use std::cmp::Ordering;

        #[derive(Clone,Copy,Eq,PartialEq)]
        struct State {
            location: LocationId,
            cost: usize,
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &State) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &State) -> Ordering {
                // Reverse the comparison order in order to use the
                // stdlib max-heap as a min-heap
                other.cost.cmp(&self.cost)
            }
        }

        let inf = self.locations.len() + 1;

        let mut dist = HashMap::new();
        let mut prev = HashMap::new();
        let mut seen = HashSet::new();
        dist.insert(start, inf);

        let mut heap = BinaryHeap::new();
        heap.push(State { location: start, cost: 0 });

        while let Some(State { location, cost }) = heap.pop() {
            if seen.contains(&location) {
                continue;
            }
            seen.insert(location);

            if location == end {
                let mut path = vec![location];
                let mut cur = location;
                while let Some(node) = prev.get(&cur) {
                    if *node != start {
                        path.push(*node);
                    }
                    cur = *node;
                }
                path.reverse();
                return Some(path)
            }

            // If we've found a better way already, skip
            if cost > *dist.get(&location).unwrap_or(&inf) {
                continue;
            }

            for adjacent in self.junctions[&location].iter() {
                if seen.contains(adjacent) {
                    continue;
                }

                let next = State { location: *adjacent, cost: cost + 1 };

                if next.cost < *dist.get(&adjacent).unwrap_or(&inf) {
                    heap.push(next);
                    dist.insert(*adjacent, next.cost);
                    prev.insert(*adjacent, location);
                }
            }
        }

        None
    }
}

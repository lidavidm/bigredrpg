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

use std::collections::HashMap;

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
    junctions: HashMap<LocationId, LocationId>,
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
        self.locations.insert(location.id, location);

        id
    }

    pub fn get_mut(&mut self, id: LocationId) -> Option<&mut Location> {
        self.locations.get_mut(&id)
    }

    pub fn iter_mut(&mut self) -> ::std::collections::hash_map::IterMut<LocationId, Location> {
        self.locations.iter_mut()
    }
}

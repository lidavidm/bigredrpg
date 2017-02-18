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

use student::Student;

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

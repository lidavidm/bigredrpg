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

use std::ops;

use rand::{self, Rng};

use interactiondb::InteractionDb;
use location::{Location, LocationId, Map};
use student::Student;
use util::rng;

/// Time represents the current game time, stored as the number of
/// minutes since game start.
#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub struct Time(u32);

pub const TIME_STEP: Time = Time(15);

impl ops::Add for Time {
    type Output = Time;

    fn add(self, rhs: Time) -> Time {
        Time(self.0 + rhs.0)
    }
}

impl ops::AddAssign for Time {
    fn add_assign(&mut self, rhs: Time) {
        self.0 += rhs.0;
    }
}

pub struct Cornell {
    map: Map,

    time: Time,
}

impl Cornell {
    pub fn new() -> Cornell {
        Cornell {
            map: Map::new(),

            time: Time(0),
        }
    }

    pub fn add_location(&mut self, location: Location) -> LocationId {
        self.map.add(location)
    }

    pub fn add_student(&mut self, student: Student, location: LocationId) {
        if let Some(loc) = self.map.get_mut(location) {
            loc.students.push(student);
        }
        else {
            panic!("LocationId not found: {:?}", location);
        }
    }

    pub fn step(&mut self, interactions: &InteractionDb) {
        self.time += TIME_STEP;

        let mut rng = rand::thread_rng();
        for (location_id, location) in self.map.iter_mut() {
            for student in location.students.iter_mut() {
                let possible_interactions = interactions.search(&student, *location_id);

                if let Some(interaction) = rng.choose(&possible_interactions) {
                    let mut choices = Vec::new();
                    for &(ref item, mut chance, ref dispositions) in interaction.choices.iter() {
                        for disposition in dispositions.iter() {
                            chance += disposition.value(student);
                        }
                        choices.push((item, chance));
                    }
                    if let Some(choice) = rng::weighted_random(choices.iter().cloned(), &mut rng) {
                        for effect in choice.effects.iter() {
                            effect.apply(student);
                        }
                    }
                }
            }
        }
    }
}

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

use location::{LocationId, Map};
use student::{StatusModifier, Student, StudentId};

pub enum SideEffect {
    // student, current, destination
    Move(StudentId, LocationId, LocationId),
}

impl SideEffect {
    pub fn apply(&self, map: &mut Map) {
        use self::SideEffect::*;

        match self {
            &Move(student, current, destination) => {
                let student = {
                    let current = map.get_mut(current)
                        .expect(&format!("location {:?} does not exist!", current));
                    current.remove_student_by_id(student)
                };
                let destination = map.get_mut(destination)
                    .expect(&format!("location {:?} does not exist!", destination));
                destination.add_student(student);
            }
        }
    }
}

pub enum EffectAction {
    Status(StatusModifier),
    Move(LocationId),
}

impl EffectAction {
    pub fn apply(&self, location: LocationId, target: &mut Student, side_effects: &mut Vec<SideEffect>) {
        use self::EffectAction::*;

        match self {
            &Status(ref modifier) => {
                target.apply_modifier(modifier.clone());
            }
            &Move(id) => {
                side_effects.push(SideEffect::Move(target.id, location, id));
            }
        }
    }
}

pub enum EffectTarget {
    Initiator,
    Participant,
    Both,
    None,
}

pub struct Effect {
    pub target: EffectTarget,
    pub action: EffectAction,
}

impl Effect {
    pub fn apply(&self, location: LocationId, initiator: &mut Student, side_effects: &mut Vec<SideEffect>) {
        self.action.apply(location, initiator, side_effects);
    }
}

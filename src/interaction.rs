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

use chance::{Chance, Disposition};
use effect::Effect;
use location::{LocationId, Map};
use student::{StatusKind, StatusModifier, Student, StudentId};

pub enum Trigger {
    Location(LocationId),
    Status(StatusKind, u32, u32),
    All(Vec<Trigger>),
    Any(Vec<Trigger>),
}

impl Trigger {
    pub fn evaluate(&self, student: &Student, location: LocationId) -> bool {
        use self::Trigger::*;
        match self {
            &Location(id) => id == location,
            &Status(kind, min, max) => {
                let status = student.get_status(kind).value();

                status >= min && status <= max
            },
            &All(ref statuses) => statuses.iter().all(|x| x.evaluate(student, location)),
            &Any(ref statuses) => statuses.iter().any(|x| x.evaluate(student, location)),
        }
    }
}

pub struct Choice {
    pub description: String,
    pub effects: Vec<Effect>,
}

pub struct Interaction {
    pub text: String,
    pub choices: Vec<(Choice, Chance, Vec<Disposition>)>,
    pub trigger: Trigger,
}

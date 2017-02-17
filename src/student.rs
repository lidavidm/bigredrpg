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

use location::LocationId;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum StatusKind {
    Stress,
    Boredom,
    Exhaustion,
    Grades,
}

#[derive(Clone)]
pub struct StatusModifier {
    description: String,
    modifiers: Vec<(StatusKind, i32)>,
}

pub struct Status {
    base: u32,
    range: (u32, u32),
    current: i32,
}

impl Status {
    pub fn new(base: u32) -> Status {
        if base > 100 {
            panic!("Base status value outside range (0, 100): {}", base);
        }
        Status {
            base: base,
            range: (0, 100),
            current: base as i32,
        }
    }
}

pub struct Student {
    id: u32,
    name: String,
    major: String,
    dorm: LocationId,

    stress: Status,
    boredom: Status,
    exhaustion: Status,
    grades: Status,

    modifiers: Vec<StatusModifier>,
}

impl Student {
    pub fn new<S: Into<String>>(id: u32, name: S, major: S, dorm: LocationId) -> Student {
        Student {
            id: id,
            name: name.into(),
            major: major.into(),
            dorm: dorm,

            stress: Status::new(0),
            boredom: Status::new(0),
            exhaustion: Status::new(0),
            grades: Status::new(100),

            modifiers: Vec::new(),
        }
    }

    pub fn apply_modifier(&mut self, modifier: StatusModifier) {
        for &(status, amount) in modifier.modifiers.iter() {
            let mut stat = match status {
                StatusKind::Stress => &mut self.stress,
                StatusKind::Boredom => &mut self.boredom,
                StatusKind::Exhaustion => &mut self.exhaustion,
                StatusKind::Grades => &mut self.grades,
            };
            stat.current += amount;
        }

        self.modifiers.push(modifier);
    }
}

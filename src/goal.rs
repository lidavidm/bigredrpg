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

use effect::{Effect, EffectAction, EffectTarget};
use location::LocationId;
use student::Student;

/// A Goal is used to define essentially a probabilistic state machine
/// for a particular student.
pub enum Goal {
    Location(LocationId),
}

impl Goal {
    /// If a goal is selected, then take a step towards fulfilling the
    /// goal.
    pub fn apply(&self) -> Vec<Effect> {
        use self::Goal::*;

        match self {
            &Location(target) => vec![Effect {
                target: EffectTarget::Initiator,
                action: EffectAction::Move(target),
            }],
        }
    }

    pub fn is_fulfilled(&self, student: &Student, location: LocationId) -> bool {
        use self::Goal::*;

        match self {
            &Location(target) => target == location,
        }
    }
}

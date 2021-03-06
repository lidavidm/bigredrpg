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

use interaction::Interaction;
use location::LocationId;
use student::Student;

pub struct InteractionDb {
    interactions: Vec<Interaction>,
}

impl InteractionDb {
    pub fn new() -> InteractionDb {
        InteractionDb {
            interactions: Vec::new(),
        }
    }

    pub fn add(&mut self, interaction: Interaction) {
        self.interactions.push(interaction);
    }

    pub fn search(&self, student: &Student, location: LocationId) -> Vec<&Interaction> {
        let mut result = Vec::new();

        for interaction in self.interactions.iter() {
            if interaction.trigger.evaluate(student, location) {
                result.push(interaction);
            }
        }

        result
    }
}

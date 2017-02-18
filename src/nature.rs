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

use student::StatusModifier;

pub struct Nature {
    pub name: String,
    pub description: String,

    pub modifier: StatusModifier,
}

impl ::std::hash::Hash for Nature {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        ::std::hash::Hash::hash(&self.name, state);
    }
}

impl ::std::cmp::PartialEq for Nature {
    fn eq(&self, other: &Nature) -> bool {
        self.name == other.name
    }
}

impl ::std::cmp::Eq for Nature {}

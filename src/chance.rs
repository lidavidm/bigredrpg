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

use std::cmp;
use std::ops;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
pub struct Chance(pub u32);

impl ops::Add<i32> for Chance {
    type Output = Chance;

    fn add(self, other: i32) -> Chance {
        let val = self.0 as i32 + other;
        if val < 0 {
            Chance(0)
        }
        else {
            Chance(val as u32)
        }
    }
}

impl ops::AddAssign<i32> for Chance {
    fn add_assign(&mut self, other: i32) {
        let val = self.0 as i32 + other;
        if val < 0 {
            self.0 = 0;
        }
        else {
            self.0 = val as u32;
        }
    }
}

impl cmp::PartialEq<u32> for Chance {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl cmp::PartialOrd<u32> for Chance {
    fn partial_cmp(&self, other: &u32) -> Option<cmp::Ordering> {
        cmp::PartialOrd::partial_cmp(&self.0, other)
    }
}

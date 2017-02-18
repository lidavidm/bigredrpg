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

use rand::Rng;

use ::chance::Chance;

pub fn weighted_random<'a, I, T, R>(items: I, rng: &mut R) -> Option<&'a T>
    where I: Iterator<Item=(&'a T, Chance)>,
          R: Rng
{
    let mut num = rng.gen_range(0, 100);

    for (item, chance) in items {
        if chance > num {
            return Some(&item);
        }
        else {
            num -= chance.0;
        }
    }

    None
}

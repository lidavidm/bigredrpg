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

use ::student::Student;
use ::chance::{Chance, Disposition};

pub fn convert_disposition_list<'a, T>(items: &'a [(T, Chance, Vec<Disposition>)], student: &Student)
                                       -> Vec<(&'a T, Chance)> {
    let mut choices = Vec::new();
    for &(ref item, mut chance, ref dispositions) in items.iter() {
        for disposition in dispositions.iter() {
            chance += disposition.value(student);
        }
        choices.push((item, chance));
    }

    choices
}

pub fn weighted_random<'a, I, T, R>(items: I, rng: &mut R) -> Option<(usize, &'a T)>
    where I: Iterator<Item=(&'a T, Chance)>,
          R: Rng
{
    let mut num = rng.gen_range(0, 100);

    for (index, (item, chance)) in items.enumerate() {
        if chance > num {
            return Some((index, &item));
        }
        else {
            num -= chance.0;
        }
    }

    None
}

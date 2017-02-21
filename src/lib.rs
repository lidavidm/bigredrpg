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

extern crate rand;

pub mod chance;
pub mod cornell;
pub mod effect;
pub mod goal;
pub mod interaction;
pub mod interactiondb;
pub mod location;
pub mod nature;
pub mod student;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn pathfinding() {
        let mut map = super::location::Map::new();
        let mut locgen = super::location::LocationIdGenerator::new_from_index(0);

        let l1 = map.add(super::location::Location::new("l1", locgen.new_id()));
        let l2 = map.add(super::location::Location::new("l2", locgen.new_id()));
        let l3 = map.add(super::location::Location::new("l3", locgen.new_id()));
        let l4 = map.add(super::location::Location::new("l4", locgen.new_id()));
        let l5 = map.add(super::location::Location::new("l5", locgen.new_id()));

        map.add_junction(l1, l2);
        map.add_junction(l1, l3);
        map.add_junction(l2, l4);
        map.add_junction(l4, l5);
        map.add_junction(l3, l2);

        let path = map.find_path(l1, l5);
        if let Some(path) = path {
            for part in path {
                print!("{:?} ", part);
            }
            println!();
        }
        else {
            println!("FAIL");
        }
    }

    #[test]
    fn it_works() {
        let mut school = super::cornell::Cornell::new();

        let mut locgen = super::location::LocationIdGenerator::new_from_index(0);

        let lr6 = school.add_location(super::location::Location::new("Low Rise 6", locgen.new_id()));
        let lr7 = school.add_location(super::location::Location::new("Low Rise 7", locgen.new_id()));

        let s1 = super::student::Student::new(0, "Test Student", "Computer Science", lr6);
        let s2 = super::student::Student::new(1, "Testing Student", "Computer Science", lr6);

        school.add_student(s1, lr6);
        school.add_student(s2, lr7);

        let mut db = super::interactiondb::InteractionDb::new();

        db.add(super::interaction::Interaction {
            text: "".into(),
            choices: vec![
                (super::interaction::Choice {
                    description: "".into(),
                    effects: vec![
                        super::effect::Effect {
                            target: super::effect::EffectTarget::Initiator,
                            action: super::effect::EffectAction::Status(super::student::StatusModifier {
                                description: "".into(),
                                modifiers: vec![
                                    (super::student::StatusKind::Stress, -10),
                                ],
                            }),
                        },
                    ]
                }, super::chance::Chance(40), vec![]),
                (super::interaction::Choice {
                    description: "".into(),
                    effects: vec![
                        super::effect::Effect {
                            target: super::effect::EffectTarget::Initiator,
                            action: super::effect::EffectAction::Status(super::student::StatusModifier {
                                description: "".into(),
                                modifiers: vec![
                                    (super::student::StatusKind::Stress, 10),
                                ],
                            }),
                        },
                    ]
                }, super::chance::Chance(60), vec![]),
            ],
            trigger: super::interaction::Trigger::Location(lr7),
        });

        db.add(super::interaction::Interaction {
            text: "".into(),
            choices: vec![
                (super::interaction::Choice {
                    description: "".into(),
                    effects: vec![
                        super::effect::Effect {
                            target: super::effect::EffectTarget::Initiator,
                            action: super::effect::EffectAction::Move(lr7),
                        },
                    ]
                }, super::chance::Chance(100), vec![]),
            ],
            trigger: super::interaction::Trigger::Location(lr6),
        });

        school.step(&db);

        assert_eq!(school.get_map().get(lr6).unwrap().students.len(), 0);
        assert_eq!(school.get_map().get(lr7).unwrap().students.len(), 2);
    }
}

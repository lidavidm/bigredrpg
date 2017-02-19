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
                        super::interaction::Effect {
                            target: super::interaction::EffectTarget::Initiator,
                            action: super::interaction::EffectAction::Status(super::student::StatusModifier {
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
                        super::interaction::Effect {
                            target: super::interaction::EffectTarget::Initiator,
                            action: super::interaction::EffectAction::Status(super::student::StatusModifier {
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
                        super::interaction::Effect {
                            target: super::interaction::EffectTarget::Initiator,
                            action: super::interaction::EffectAction::Move(lr7),
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

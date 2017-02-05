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

import Cornell from "./cornell";
import Interaction from "./interaction";
import {InteractionDb} from "./interactiondb";
import Location from "./location";
import Student from "./student";

function main() {
    let interactions = new InteractionDb();
    let i1 = new Interaction();
    i1.text = "Test interaction 1";
    i1.choices = [];
    i1.trigger = {
        kind: "location",
        location: "Low Rise 7",
    };
    interactions.add(i1);
    let i2 = new Interaction();
    i2.text = "Test interaction 2";
    i2.choices = [];
    i2.trigger = {
        kind: "location",
        location: "Low Rise 7",
    };
    interactions.add(i2);

    let cornell = new Cornell();

    let lr6 = new Location("Low Rise 6");
    let lr7 = new Location("Low Rise 7");

    cornell.addLocation(lr6);
    cornell.addLocation(lr7);

    let s1 = new Student(Student.newId(), "Student 1", "Computer Science", "Low Rise 6");
    let s2 = new Student(Student.newId(), "Student 2", "Computer Science", "Low Rise 6");
    let s3 = new Student(Student.newId(), "Student 3", "Computer Science", "Low Rise 7");

    cornell.addStudent(s1, "Low Rise 6");
    cornell.addStudent(s2, "Low Rise 6");
    cornell.addStudent(s3, "Low Rise 7");

    cornell.setTime(0, 7, 0);

    console.log("Cornell initialized.");

    cornell.step(interactions);
}

main();

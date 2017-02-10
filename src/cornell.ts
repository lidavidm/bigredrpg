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

import { Grid, Position } from "./grid";
import Interaction from "./interaction";
import { Choice } from "./interaction";
import { InteractionDb } from "./interactiondb";
import Location from "./location";
import Student from "./student";

import * as random from "./random";

/** Minimum possible time step, in minutes. */
const TIME_DELTA = 5;
const DELTAS_PER_HOUR = 60 / TIME_DELTA;
const DELTAS_PER_DAY = 24 * DELTAS_PER_HOUR;

const INTERACTION_CHANCE = 0.6;

export default class Cornell {
    grid: Grid;
    locations: Map<string, Location>;
    students: Student[];
    time: number;

    constructor() {
        this.grid = new Grid();
        this.locations = new Map();
        this.students = [];
        this.time = 0;
    }

    addLocation(location: Location, position: Position) {
        if (!this.locations.has(location.name)) {
            this.locations.set(location.name, location);
            this.grid.addLocation(location, position);
        }
        else {
            throw {
                description: "Location already present: " + location.name,
            };
        }
    }

    addStudent(student: Student, location: string) {
        let loc = this.locations.get(location);
        if (!loc) {
            throw {
                description: "Could not find location: " + location,
            };
        }
        loc.addStudent(student);
        this.students.push(student);
    }

    setTime(day: number, hour: number, minute: number) {
        if (minute % TIME_DELTA !== 0) {
            throw {
                description: "Minute must be a multiple of TIME_DELTA",
            }
        }

        this.time = day * DELTAS_PER_DAY + hour * DELTAS_PER_HOUR + minute / TIME_DELTA;
    }

    step(interactions: InteractionDb) {
        this.time += TIME_DELTA;

        let processing: [Location, Student, Interaction][] = [];

        for (let location of this.locations.values()) {
            for (let student of location.students) {
                let potentialInteractions = interactions.search(student, location);
                console.log("Found", student, location.name, potentialInteractions);

                if (potentialInteractions.length > 0 && Math.random() < INTERACTION_CHANCE) {
                    console.log("Interaction triggered");
                    processing.push([location, student, random.choice(potentialInteractions)]);
                }
            }
        }

        for (let [location, student, interaction] of processing) {
            console.log("Processing", student);

            let weightedChoices: [Choice, number][] = [];
            for (let choice of interaction.choices) {
                weightedChoices.push([choice, 1.0 / interaction.choices.length]);
            }
            let choice = random.weightedChoice(weightedChoices);

            console.log("Chose:", choice);

            for (let effect of choice.effects) {
                switch (effect.kind) {
                case "pass_time":
                    break;
                case "status":
                    console.log("Applying status modifier", effect.modifier);
                    student.applyStatusModifier(effect.status, effect.modifier);
                    console.log(student.status);
                    break;
                }
            }
        }
    }
}

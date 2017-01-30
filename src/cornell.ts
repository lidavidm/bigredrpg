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

import Location from "./location";
import Student from "./student";

/** Minimum possible time step, in minutes. */
const TIME_DELTA = 5;
const DELTAS_PER_HOUR = 60 / TIME_DELTA;
const DELTAS_PER_DAY = 24 * DELTAS_PER_HOUR;

export default class Cornell {
    locations: Map<string, Location>;
    students: Student[];
    time: number;

    constructor() {
        this.locations = new Map();
        this.students = [];
        this.time = 0;
    }

    addLocation(location: Location) {
        if (!this.locations.has(location.name)) {
            this.locations.set(location.name, location);
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

    step() {
        this.time += TIME_DELTA;
    }
}

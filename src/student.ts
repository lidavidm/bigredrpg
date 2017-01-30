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

export enum StatusType {
    Stress,
    Grades,
    Boredom,
    Exhaustion,
}

export interface StudentStatus {
    stress: [number, number],
    grades: [number, number],
    boredom: [number, number],
    exhaustion: [number, number],
}

export default class Student {
    id: number;
    name: string;
    major: string;
    dorm: string;

    status: StudentStatus;

    static maxId = 0;
    static newId(): number {
        Student.maxId += 1;
        return Student.maxId;
    }

    constructor(id: number, name: string, major: string, dorm: string) {
        this.id = id;
        this.name = name;
        this.status = {
            stress: [0, 100],
            grades: [100, 100],
            boredom: [0, 100],
            exhaustion: [0, 100],
        };
        this.major = major;
        this.dorm = dorm;
    }
}

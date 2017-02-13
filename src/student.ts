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

export interface StatusModifier {
    modifiers: [StatusType, number][],
    description: string,
}

export interface Status {
    base: number,
    min: number
    max: number,
    current: number,
}

function makeStatus(base: number, max: number): Status {
    return {
        base: base,
        min: 0,
        max: max,
        current: base,
    }
}

export interface StudentStatus {
    stress: Status,
    grades: Status,
    boredom: Status,
    exhaustion: Status,

    modifiers: StatusModifier[],
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
            stress: makeStatus(0, 100),
            grades: makeStatus(100, 100),
            boredom: makeStatus(0, 100),
            exhaustion: makeStatus(0, 100),

            modifiers: [],
        };
        this.major = major;
        this.dorm = dorm;
    }

    applyStatusModifier(modifier: StatusModifier) {
        this.status.modifiers.push(modifier);

        for (let [status, value] of modifier.modifiers) {
            let target = null;
            switch (status) {
            case StatusType.Stress:
                target = this.status.stress;
                break;
            case StatusType.Grades:
                target = this.status.grades;
                break;
            case StatusType.Boredom:
                target = this.status.boredom;
                break;
            case StatusType.Exhaustion:
                target = this.status.exhaustion;
                break;
            }

            if (target) {
                target.current += value;
            }
        }
    }
}

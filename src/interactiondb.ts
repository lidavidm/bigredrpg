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

import Interaction from "./interaction";
import { Trigger } from "./interaction";
import Location from "./location";
import Student from "./student";
import { fail } from "./util";

export interface Criterion {

}

function evaluateTrigger(trigger: Trigger, location: Location): boolean {
    switch (trigger.kind) {
    case "location":
        return trigger.location === location.name;
    case "person":
        return false;
        // TODO:
    case "status":
        return false;
        // TODO:
    case "any":
        for (let subtrigger of trigger.triggers) {
            if (evaluateTrigger(subtrigger, location)) {
                return true;
            }
        }
        return false;
    case "all":
        for (let subtrigger of trigger.triggers) {
            if (!evaluateTrigger(subtrigger, location)) {
                return false;
            }
        }
        return true;
    default:
        return fail(trigger);
    }
}

export class InteractionDb {
    interactions: Interaction[];

    constructor() {
        this.interactions = [];
    }

    add(interaction: Interaction) {
        this.interactions.push(interaction);
    }

    search(student: Student, location: Location): Interaction[] {
        let result = [];

        for (let interaction of this.interactions) {
            if (evaluateTrigger(interaction.trigger, location)) {
                result.push(interaction);
            }
        }

        return result;
    }
}

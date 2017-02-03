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
import Location from "./location";
import Student from "./student";

export interface Criterion {

}

export class InteractionDb {
    interactions: Interaction[];

    constructor() {
        this.interactions = [];
    }

    addInteraction(interaction: Interaction) {
        this.interactions.push(interaction);
    }

    search(student: Student, location: Location): Interaction[] {
        let result = [];

        for (let interaction of this.interactions) {
            switch (interaction.trigger.kind) {
            case "location":
                if (interaction.trigger.location === location.name) {
                    result.push(interaction);
                }
                break;
            case "person":
                // TODO:
                break;
            }
        }

        return result;
    }
}

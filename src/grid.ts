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

export interface Position {
    x: number,
    y: number,
}

/**
 * Determines the relative positions of locations.
 */
export class Grid {
    locations: Map<string, {
        location: Location,
        position: Position,
    }>;

    constructor() {
        this.locations = new Map();
    }

    addLocation(location: Location, position: Position) {
        this.locations.set(location.name, {
            location: location,
            position: position,
        });
    }

    distanceBetween(l1: string, l2: string): number {
        let loc1 = this.locations.get(l1);
        if (!loc1) throw {
            description: `Location ${l1} does not exist.`,
        };
        let pos1 = loc1.position;
        let loc2 = this.locations.get(l2);
        if (!loc2) throw {
            description: `Location ${l2} does not exist.`,
        };
        let pos2 = loc2.position;
        return Math.sqrt(Math.pow(pos1.x - pos2.x, 2) + Math.pow(pos1.y - pos2.y, 2));
    }
}

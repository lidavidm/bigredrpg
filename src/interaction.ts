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

import { Effect } from "./effect";
import { StatusType } from "./student";

export interface LocationTrigger {
    kind: "location",
    location: string,
}

export interface PersonTrigger {
    kind: "person",
    people: any[],
}

export interface StatusTrigger {
    kind: "status",
    status: StatusType,
}

export interface AnyTrigger {
    kind: "any",
    triggers: Trigger[],
}

export interface AllTrigger {
    kind: "all",
    triggers: Trigger[],
}

export type Trigger = LocationTrigger | PersonTrigger | StatusTrigger | AnyTrigger | AllTrigger;

export interface Choice {
    description: string,
    effects: Effect[],
}

export default class Interaction {
    text: string;
    choices: Choice[];
    trigger: Trigger;
}

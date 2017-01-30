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

import { StatusType } from "./student";

export interface PlayerTarget {
    kind: "player",
}

export interface NPCTarget {
    kind: "npc",
}

export interface NoTarget {
    kind: "none",
}

export interface EffectBase {
    target: PlayerTarget | NPCTarget,
}

export interface Status extends EffectBase {
    kind: "status",
    status: StatusType,
    modifier: number,
    description: string,
}

/** Suspend this actor for the specified number of time steps. */
export interface PassTime extends EffectBase {
    kind: "pass_time",
    steps: number,
}

export type Effect = Status | PassTime;

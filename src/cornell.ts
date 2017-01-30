/** Minimum possible time step, in minutes. */
const TIME_DELTA = 5;

export default class Cornell {
    locations: Map<string, any>;
    students: any[];
    time: number;

    constructor() {
        this.locations = new Map();
        this.students = [];
        this.time = 0;
    }

    step() {
        this.time += TIME_DELTA;
    }
}

export interface StudentStatus {
    stress: [number, number],
    grades: [number, number],
    boredom: [number, number],
    exhaustion: [number, number],
}

export default class Student {
    name: string;
    major: string;
    dorm: string;

    status: StudentStatus;

    constructor(name, major, dorm) {
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

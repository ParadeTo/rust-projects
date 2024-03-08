class Singleton {
    private static instance: Singleton;
    private data: string;

    private constructor(data: string) {
        this.data = data
    }

    public static getInstance(data: string): Singleton {
        if (!Singleton.instance) {
            Singleton.instance = new Singleton(data);
        }
        return Singleton.instance;
    }
    
    public getData(): string {
        return this.data
    }
}

const instance1 = Singleton.getInstance('hello');
const instance2 = Singleton.getInstance('world');

console.log(instance1 === instance2); // true
console.log(instance1.getData()); // hello
console.log(instance2.getData()); // hello

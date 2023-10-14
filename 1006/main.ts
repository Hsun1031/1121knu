main();

function getRandomInt(min: number, max: number): number {
    min = Math.ceil(min);
    max = Math.floor(max);
    return Math.floor(Math.random() * (max - min) + min);
}

function getRandomIntList(min: number, max: number, len: number): number[]  {
    let list:number[] = [];
    for(let i: number = 0; i < len; i++) {
        list.push(getRandomInt(min, max));
    }
    return list;
}

function to_hex(list: number[]): number {
    let num:number = 0;

    for(let i: number = 0; i < list.length; i++) {
        if(list[i] === 0) { continue; }
        num += Math.pow(2, list.length - 1 - i);
    }

    return num;
}

function main(): void {
    const rand_list:number[] = getRandomIntList(0, 2, 24);

    console.log("rand_list:", rand_list);

    let true_list:number[] = [];

    for(let i: number = 0; i < 24; i+=8) {
        true_list.push(to_hex(rand_list.slice(i, i + 8)));
    }
    console.log("true_list:", true_list)

    let list:number[] = [];

    for(let i: number = 0; i < 3; i++) {
        let num: number = true_list[i] - 128;
        list.push((num < -100)? -100 : (num > 100)? 100 : num);
    }
    console.log("list:", list)
}

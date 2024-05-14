const fs = require('fs');

// const list = [1, 2, 3];
// const result = list.map((item) => item + 1);
// console.log(result);

// read file
// fs.readFileSync('lines', 'utf8')
//     .toString()
//     .split('\n')
//     .filter((_: string, i: number) => i % 2 === 0)
//     .filter((_: string, i: number) => i > 1 && i < 4)
//     .forEach((line: string) => console.log(line));

// enum Color {
//     Red,
//     Blue,
//     Green,
//     Yellow,
// }

// function printColor(color: Color) {
//     switch (color) {
//         case Color.Red:
//             console.log('Red');
//             break;
//         case Color.Blue:
//             console.log('Blue');
//             break;
//         case Color.Green:
//             console.log('Green');
//             break;
//     }
// }

// printColor(Color.Yellow);

// type Custom = {
//     age: number,
//     name: string,
// }

// type Item = number | string | Custom;

// function append(items: Item[]) {
//     items.push("Hello Fem!");
// }
// const items: Item[] = [];
// console.log(items);
// append(items);
// console.log(items);

// const numbers: number[] = [];
// append(numbers);

// function practice(value: number | undefined): number | undefined {
//     return value === undefined ? undefined : value * 5;
// }

// function practice(values: number[], index: number): number {
//     return (values[index] ?? index) * 5;
// }


const args = process.argv[2];

try {
    fs.readFileSync(args, 'utf8')
        .toString()
        .split('\n')
        .forEach((line: string) => {
            const print = parseInt(line);
            if (isNaN(print)) {
                console.log('Line not a number');
            } else {
                console.log(print);
            }
        });
} catch (error) {
    console.error('File not found');
}
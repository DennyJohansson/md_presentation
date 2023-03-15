function example() {
  let hello = "Hello";

  let helloWorld = appendWorld(hello);
  let helloWorld2 = appendWorld(hello);

  console.log(helloWorld, helloWorld2);
}


function appendWorld(str: string) {
  return str + " World";
}


function example2() {
  const item = [1, 2, 3];

  printOutItem(item);
  printOutItem(item);
}


function printOutItem(item: number[]) {
  for (let i = 0; i < item.length; i++) {
    console.log(item[i]);
  }
}

function example3() {
  const items = [1, 2, 3];

  const lastItem = items[items.length - 1];

  items.pop()
  
  console.log(lastItem)
}

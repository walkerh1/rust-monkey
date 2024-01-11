# rust-monkey
*Rust* interpreter and compiler for the *Monkey* programming language, based on the books by Thorsten Ball.

## Additional language features
In addition to the langauge features of vanilla *Monkey*, I have added the following:
* logical operators `&&` and `||`
* `while` loop
* assignment (e.g. `x = x + 1;`)

With assignment it is possible to create closures that can also maintain state between function calls, such as `counter` below:
```
let makeCounter = fn() {
    let count = 0;
    return fn() {
        count = count + 1;
        return count;
    };
}
let counter = makeCounter();
counter(); // 1
counter(); // 2
counter(); // 3
```

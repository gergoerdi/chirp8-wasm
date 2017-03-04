var keystate = {};
window.onkeyup = e => { keystate[e.code] = false; };
window.onkeydown = e => { keystate[e.code] = true; };
window.addEventListener('blur',() => { keystate = {}; });

const keys = ["KeyX",   "Digit1", "Digit2", "Digit3",
              "KeyQ",   "KeyW",   "KeyE",   "KeyA",
              "KeyS",   "KeyD",   "KeyZ",   "KeyC",
              "Digit4", "KeyR",   "KeyF",   "KeyV"];


function read_keys() {
    let i = 1;
    let r = 0;

    for (key of keys) {
        if (keystate[key]) {
            r |= i;
        }
        i <<= 1;
    }

    return r;
}

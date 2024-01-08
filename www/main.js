import * as wasm from "../pkg/chirp8_wasm.js";

const SCALE = 8;
const WIDTH = 64 * SCALE;
const HEIGHT = 32 * SCALE;
const FPS = 60;

const screen = document.getElementById("screen");
const screen_ctx = screen.getContext("2d");

screen.width = WIDTH;
screen.height = HEIGHT;

const pixbuf = new ArrayBuffer(WIDTH * HEIGHT * 4);

// screen_ctx.font = "8px C64 Pro Mono Local";
// screen_ctx.fillStyle = "white";
// screen_ctx.textAlign = "center";
// screen_ctx.fillText("Click here", screen.width / 2, screen.height / 2 - 8);
// screen_ctx.fillText("to start", screen.width / 2, screen.height / 2 + 8);
// screen.onclick = start;

// async function start()
{
    // screen.onclick = () => {};

    wasm.default().then(() => {
        const app_ctx = wasm.setup();

        let step = () => {
            const keys = read_keys();
            wasm.step(app_ctx, keys);
        };
        setInterval(step, 1000 / FPS);
        
        let update_screen = () => {
            wasm.render_image(app_ctx, new Uint32Array(pixbuf));
            
            screen_ctx.putImageData(new ImageData(new Uint8ClampedArray(pixbuf), WIDTH, HEIGHT), 0, 0);
            
            window.requestAnimationFrame(update_screen);
        };
        window.requestAnimationFrame(update_screen);
    });
}

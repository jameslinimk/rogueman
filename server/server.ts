import express from "express";
import { resolve } from "path";

const port = 8604;
const app = express();

app.use("/assets", express.static(resolve("../assets")));
app.get("/game.wasm", (_, res) => {
    res.sendFile(
        resolve("../target/wasm32-unknown-unknown/release/rust-game.wasm")
    );
});

app.get("/", (_, res) => {
    res.sendFile(resolve("static/index.html"));
});

app.listen(port, () => {
    console.log(`Listening at http://localhost:${port}`);
});

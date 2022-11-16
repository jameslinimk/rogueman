import express from "express"

const port = 8604
const app = express()

app.use(express.static("./static"))
app.use("/assets", express.static("../assets"))
app.get("/game.wasm", (_, res) => {
    res.sendFile("../target/wasm32-unknown-unknown/release/rogueman.wasm")
})

app.get("/", (_, res) => {
    res.sendFile("./index.html")
})

app.listen(port, () => {
    console.log(`Listening at http://localhost:${port}`)
})

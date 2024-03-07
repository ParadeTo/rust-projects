import fs from 'fs'
import path from 'path'
import express from 'express'
import React from 'react'
import ReactDOMServer from 'react-dom/server'
import {App} from './src/App'
import {render, WasmReact} from '../rsrender/pkg/rsrender'

WasmReact.useReact(React)
// const wasmBuffer = fs.readFileSync(
//     path.resolve(__dirname, '../rsrender/pkg/rsrender_bg.wasm')
// )
//
// const importObject = {
//     imports: {},
// };
// let render
// WebAssembly.instantiate(wasmBuffer, importObject).then((wasmModule) => {
//     // Exported function live under instance.exports
//     const {add: r} = wasmModule.instance.exports
//     render = r
// })
//
// import('')

const app = express()

app.get('/rsrender', (req, res) => {
    const string = render()
    console.log(string)
    const s = ReactDOMServer.renderToString(string)
    console.log(s)

    res.end(s)
})

app.get('/jsrender', (req, res) => {
    const string = ReactDOMServer.renderToString(<App/>)
    res.end(string)
})

app.listen(3002, () => {
    console.log('App is running on http://localhost:3002')
})

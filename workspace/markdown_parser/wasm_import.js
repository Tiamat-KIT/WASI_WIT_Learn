const fs = require("fs")

const wasm = process.argv[2]

const run = async() => {
    const module = await WebAssembly.compile(fs.readFileSync(wasm))
    const imports = WebAssembly.Module.imports(module)

    console.log(JSON.stringify(imports,null,2))

}

run().catch(err => console.log(err))
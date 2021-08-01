addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
    await wasm_bindgen(wasm)
    return new Response(await wasm_bindgen.run(request), {
      headers: { 'Content-Type': 'application/json;charset=UTF-8' }
    })
}

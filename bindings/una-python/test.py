import asyncio
import una

config = dict({
    "url": "https://127.0.0.1:8081",
    "macaroon": "a1b2c3",
    "certificate": "a1b2c3"
})

invoice = dict({
    "amount": 100,
    "description": "test bindings"
})

async def main():
    node = una.Node("LndRest", config)
    info = await node.get_info()
    print(info.get('version'))
    payreq = await node.create_invoice(invoice)
    print(payreq)

asyncio.run(main())


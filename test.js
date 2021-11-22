const { encode, decode, verify } = require(".");
const secret = "12312313"
encode({
    secret: secret,
    exp: parseInt(new Date() / 1000),
    // exp: new Date() / 1000 + 24 * 60 * 60,
    algorithm: "ES256"
}).then(async (token) => {
    try {
        console.time("a")
        const result = await decode(token, secret);
        console.timeEnd("a")
        console.log(result);
    } catch (e) {
        console.log(e)
    }
}).catch((e) => {
    console.log(e)
})
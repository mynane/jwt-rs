const natice = require("./index.node");

function encode(payload) {
  return new Promise((resolve, reject) => {
    const result = natice.encode(payload);

    if (result.error) {
      reject(result.message)
    } else {
      resolve(result.token)
    }
  })
}

function decode(token, secret) {
  return new Promise((resolve, reject) => {
    const result = natice.decode(token, secret);

    if (token.error) {
      reject(result.message)
    } else {
      resolve(result.data)
    }
  })
}

function verify(token, secret) {
  return new Promise((resolve, reject) => {
    const { data, error } = natice.decode(token, secret);

    if (error) {
      reject(error)
    } else {
      resolve(data.exp > (new Date() / 1000))
    }
  })
}

exports.encode = encode;
exports.decode = decode;
exports.verify = verify;

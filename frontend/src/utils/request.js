import m from 'mithril'

if (process.env.NODE_ENV == 'production') {
    var url = "http://signin-api.nickendo.com"
} else {
    var url = "http://localhost:8000"
}

function request(options) {
    options.url = url + options.url

    return m.request(options)
}

function catcherPromise(e) {
    return new Promise((resolve, reject) => {
        if (e !== null) {
            if (e.hasOwnProperty('error')) {
                console.log("error", e.error)
            } else if (e.hasOwnProperty('message')) {
                console.log("message", e.error)
            }
        }
        reject(e)
    })
}

export default request
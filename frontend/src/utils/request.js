import m from 'mithril'

if (process.env.NODE_ENV == 'production') {
    var url = "http://signin-api.nickendo.com"
} else {
    var url = "http://localhost:8000"
}

function deserialize(data) {
    try {return data !== "" ? JSON.parse(data) : null}
    catch (e) {throw new Error(data)}
}

function request(options) {
    options.url = url + options.url

    //options.extract = xhr => ({
            //status: xhr.status,
            //body: deserialize(xhr.responseText)
    //})

    return m.request(options)
        //.catch(catcherPromise)
        //.then(regularize)
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

function regularize(result) {
    return result.body
}

export default request
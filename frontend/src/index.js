import m from 'mithril'
import Member from './models/Member'
import Signin from './models/Signin'
import Admin from './views/Admin'
import Regular from './views/Regular'
import "tachyons"
import "./index.css"

m.route.prefix("")

m.route(document.body, "/", {
    '/': {
        render() {
            return m(Regular)
        }
    },
    '/admin': {
        render() {
            return m(Admin)
        }
    }
})
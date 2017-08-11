import r from '../utils/request'
import Signin from './Signin'
import stream from 'mithril/stream'

const Member = {
    list: new Map(),
    current: {
        id: stream(),
        name: stream(),
        team: stream(),
        last_pizza: stream(),
        signin: stream(),
        pizza: stream(),
        signedin: stream(),
        clear() {
            Member.current.id("")
            Member.current.name("")
            Member.current.team("")
            Member.current.last_pizza("")
            Member.current.pizza("")
            Member.current.signin("")
            Member.current.signedin("")
        }
    },
    setCurrent(id) {
        let i = Member.list.get(+id)
        Member.current.id(i.id)
        Member.current.name(i.name)
        Member.current.team(i.team)
        Member.current.pizza(i.last_pizza)
        Member.current.last_pizza(i.last_pizza)
        let signin = Signin.signin(i.id)
        Member.current.signin(signin)
        Member.current.signedin(signin !== undefined)
    },
    signin() {
        return r({
            url: `/members/${Member.current.id()}/signin`,
            method: 'POST',
            data: { pizza: Member.current.pizza() }
        })
    },
    signout() {
        return r({
            url: `/members/${Member.current.id()}/signout`,
            method: 'GET'
        })
    },
    action() {
        if (Member.current.signedin() !== undefined) {
            return Member.signout()
        } else {
            return Member.signin()
        }
    },
    add() {
        let data = {
            name: Member.current.name(),
            team: +Member.current.team(),
            last_pizza: "None"
        }

        return r({
            url: "/members",
            method: 'POST',
            data: data
        })
    },
    all() {
        return r({
            url: '/members',
            method: 'GET'
        }).then(data => {
            Member.list = new Map()
            data.members.forEach(member => Member.list.set(member.id, member))
        })
    }
}

export default Member
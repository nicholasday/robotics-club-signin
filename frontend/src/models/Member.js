import r from '../utils/request'
import m from 'mithril'
import Signin from './Signin'
import stream from 'mithril/stream'

const Member = {
    list: new Map(),
    search: '',
    filtered_members: [],
    setSearch(value) { 
        Member.search = value
        if (Member.search.trim().length == 0) {
            Member.current.clear()
        }
        Member.filtered_members = Array.from(Member.list.values()).filter(member => member.name.toUpperCase().indexOf(Member.search.toUpperCase()) > -1)
        if (Member.filtered_members.length == 1) {
            console.log(Member.filtered_members[0].id)
            Member.setCurrent(Member.filtered_members[0].id)
        } else {
            Member.current.clear()
        }
    },
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
    signining_in: false,
    check_shown: false,
    signin() {
        Member.signining_in = true
        return r({
            url: `/members/${Member.current.id()}/signin`,
            method: 'POST',
            data: { pizza: Member.current.pizza() }
        }).then(() => {
            Member.signining_in = false
            Member.check_shown = true
            Member.setSearch('')
            setTimeout(() => {
                Member.check_shown = false
                m.redraw()
            }, 1000)
        })
    },
    signout() {
        Member.signining_in = true
        return r({
            url: `/members/${Member.current.id()}/signout`,
            method: 'GET'
        }).then(() => {
            Member.signining_in = false
            Member.check_shown = true
            Member.setSearch('')
            setTimeout(() => {
                Member.check_shown = false
                m.redraw()
            }, 1000)
            Member.setSearch('')
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
import m from 'mithril'
import Member from '../models/Member'
import Signin from '../models/Signin'
import stream from 'mithril/stream'

function radio(name) {
    return m('div', [
        m(`input.mr2[type='radio'][name='pizza'][value='${name}']`,
            { 
                checked: Member.current.pizza() == name,
                onclick: m.withAttr('value', Member.current.pizza)
            }
        ),
        m(`label.lh-copy[for='${name}']`, name)
    ])
}

function member_radio(member) {
    return m('div.flex.items-center.mb2', [
        m(`input.mr2[type='radio'][name='data'][value='${member.id}']`,
            { 
                checked: Member.current.id() == member.id,
                onclick: m.withAttr('value', Member.setCurrent)
            }
        ),
        m(`label.lh-copy[for='${member.id}']`, member.name)
    ])
}

function signin(e) {
    console.log(e)
    e.preventDefault()
    if (Member.current.signedin()) {
        Member.signout().then(() => {
            Member.current.clear()
            document.getElementById('search').focus()
            Member.all()
            Signin.all()
        })
    } else {
        Member.signin().then(() => {
            Member.current.clear()
            Member.all()
            Signin.all()
        })
    }
}

function search_created(vnode) {
    vnode.dom.focus()
}

const Regular = {
    oninit() {
        Member.all().then(() => {
            Member.setSearch('')
            m.redraw()
        })
        Signin.all().then(m.redraw)
    },
    view() {
        let action = Member.current.signedin() ? "Signout" : "Signin"
        let signedinclasses = ""
        let current_signin = Member.current.signin()
        console.log("current signin", current_signin)
        console.log("signedin", Member.current.signedin())
        if (current_signin != undefined && typeof current_signin !== "string") {
            if (current_signin.date_out !== null) {
                signedinclasses = ".noclick.dimmed"
            }
        }
        let status = ""
        if (Member.signining_in) {
            status = "✈"
        } else if (Member.check_shown) {
            status = "✔"
        }
        let button_text = action
        return m('div.bg-orange.h-100.pa1.pb5', 
            m('h2.white.center.tc.br2', "ECG Robotics Signin"),
            m("form.center.measure.pa4.bg-white.br1[autocomplete='off']", 
                { onsubmit: signin },
                m('div.cf', [
                    m.fragment({ oncreate: search_created}, [
                        m('input.fl.pa2.lh-solid.ba.w-60.input-reset#search', { oninput: m.withAttr('value', Member.setSearch), value: Member.search})
                    ]),
                    m(`input.fl.w-25.button-reset.bg-orange.white.bn.pv2.f6.dim.br2.br--right.pointer.b${signedinclasses}[type='submit'][value='${button_text}']`),
                    m("div.fl.pa2", status)
                ]),
                m('div.pb2.pt2.mv3.bb.bw2.border--orange.flex.justify-around', [
                    radio("Bacon"),
                    radio("Cheese"),
                    radio("Peppers"),
                    radio("None")
                ]),
                Member.filtered_members.map(member => member_radio(member))
            )
        )
    }
}

export default Regular
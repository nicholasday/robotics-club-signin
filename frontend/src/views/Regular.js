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

const Regular = {
    oninit() {
        Member.all().then(m.redraw)
        Signin.all().then(m.redraw)
    },
    view() {
        let action = Member.current.signedin() ? "Signout" : "Signin"
        console.log(Member.current.signin())
        let signedinclasses = ""
        let current_signin = Member.current.signin()
        if (current_signin != undefined) {
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
        let button_text = action + " " + status
        console.log(action, status)
        return m('div.bg-orange.h-100.pa1.pb5', 
            m('h2.white.center.tc.br2', "ECG Robotics Signin"),
            m('form.center.measure.pa4.bg-white', 
                { onsubmit: signin },
                Array.from(Member.list.values()).map(member => member_radio(member)),
                m('div.pb2.pt2.mt3.bt.bw2.border--orange.flex.justify-around', [
                    radio("Bacon"),
                    radio("Cheese"),
                    radio("Peppers"),
                    radio("None")
                ]),
                m(`input.button-reset.bg-orange.white.bn.mt2.pv2.ph3.f6.dim.br2.pointer.b${signedinclasses}[type='submit'][value='${button_text}']`)
            )
        )
    }
}

export default Regular
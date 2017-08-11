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
        return m('div.bg-orange.h-100.pa1.pb5', 
            m('h2.white.center.tc', "ECG Robotics Signin"),
            m('form.center.measure.pa4.bg-white', 
                { onsubmit: signin },
                Array.from(Member.list.values()).map(member => member_radio(member)),
                m('div.pb2.pt2.mt3.bt.bw2.border--orange.flex.justify-around', [
                    radio("Bacon"),
                    radio("Cheese"),
                    radio("Peppers"),
                    radio("None")
                ]),
                Member.current.signedin()
                    ? m("input.input-reset.bg-orange.white.bn.mt2.pv2.ph3.f6.pointer.b[type='submit'][value='Signout']")
                    : m("input.input-reset.bg-orange.white.bn.mt2.pv2.ph3.f6.pointer.b[type='submit'][value='Signin']")
            )
        )
    }
}

export default Regular
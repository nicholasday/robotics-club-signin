import m from "mithril";
import Member from "../models/Member";
import Signin from "../models/Signin";
import stream from "mithril/stream";

function radio(name) {
  return m("div", [
    m(`input.mr2[type='radio'][name='pizza'][value='${name}']`, {
      checked: Member.current.pizza() == name,
      onclick: m.withAttr("value", Member.current.pizza)
    }),
    m(`label.lh-copy[for='${name}']`, name)
  ]);
}

function member_radio(member) {
  return m("div.flex.items-center.mb2", [
    m(`input.mr2[type='radio'][name='data'][value='${member.id}']`, {
      checked: Member.current.id() == member.id,
      onclick: m.withAttr("value", Member.setCurrent)
    }),
    m(`label.lh-copy[for='${member.id}']`, member.name)
  ]);
}

function signin(e) {
  console.log(e);
  e.preventDefault();
  if (Member.current.signedin()) {
    Member.signout().then(() => {
      Member.current.clear();
      document.getElementById("search").focus();
      Member.all();
      Signin.all();
    });
  } else {
    Member.signin().then(() => {
      Member.current.clear();
      document.getElementById("search").focus();
      Member.all();
      Signin.all();
    });
  }
}

function search_created(vnode) {
  vnode.dom.focus();
}

const Regular = {
  oninit() {
    Signin.getPizzaList();
    Member.all().then(() => {
      Member.setSearch("");
      m.redraw();
    });
    Signin.all().then(() => {
      Member.setSearch("");
      m.redraw();
    });
  },
  view() {
    let action = Member.current.signedin() ? "Signout" : "Signin";
    let signedinclasses = "";
    let current_signin = Member.current.signin();
    console.log("current signin", current_signin);
    console.log("signedin", Member.current.signedin());
    if (current_signin != undefined && typeof current_signin !== "string") {
      if (current_signin.date_out !== null) {
        signedinclasses = ".noclick.dimmed";
      }
    }
    let status = "";
    if (Member.signining_in) {
      status = "✈";
    } else if (Member.check_shown) {
      status = "✔";
    }
    let button_text = action;
    return m(
      "div.bg-orange.h-100.pv1.ph1-ns.pb5",
      m("h2.white.center.tc.br2", "ECG Robotics Signin"),
      m(
        "form.center.w-100.measure-ns.pa3.pa4-ns.bg-white.br1-ns[autocomplete='off']",
        { onsubmit: signin },
        m("div.cf.ma0.pa0.w-100", [
          m.fragment({ oncreate: search_created }, [
            m("input.input-reset.fl.lh-solid.ba.br0.pa2.w-100.w-60-ns#search", {
              oninput: m.withAttr("value", Member.setSearch),
              value: Member.search
            })
          ]),
          m(
            `input.button-reset.fl.w-100.w-25-ns.bg-orange.white.bn.pv2.f6.dim.br0.br2-ns.br--right-ns.pointer.b${signedinclasses}[type='submit'][value='${button_text}']`
          ),
          m("div.fl.pa2", status)
        ]),
        m(
          "div.pb2.pt2.mb2.mt1.bb.bw2.border--orange.flex.justify-around",
          Signin.pizzaList.map(function(pizza) {
            return radio(pizza);
          })
        ),
        Member.filtered_members.map(member => member_radio(member))
      )
    );
  }
};

export default Regular;

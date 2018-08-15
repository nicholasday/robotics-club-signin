import m from "mithril";
import Member from "../models/Member";
import Signin from "../models/Signin";

function addMember(e) {
  e.preventDefault();
  Member.add()
    .then(Member.current.clear)
    .then(Member.all);
}

var date = "";

function setDate(d) {
  date = d;
}

function goToDate(e) {
  e.preventDefault();
  Signin.getDate(date).then(m.redraw);
}

var pizza = "";

function setPizza(p) {
  pizza = p;
}

function addPizza(e) {
  e.preventDefault();
  Signin.addPizza(pizza);
}

function deletePizza(name) {
  return function(e) {
    e.preventDefault();
    Signin.removePizza(name);
  };
}

function deleteMember(id) {
  return function(event) {
    event.preventDefault();
    Member.delete(id)
      .then(Member.current.clear)
      .then(Member.all);
  };
}

function radio(name) {
  return [
    m(`input[type='radio'][name='data'][value='${name}']`, {
      checked: Member.current.team() == name,
      onclick: m.withAttr("value", Member.current.team)
    }),
    m(`label[for='${name}']`, name)
  ];
}

const Admin = {
  oninit() {
    Member.all().then(m.redraw);
    Signin.all().then(m.redraw);
    Signin.getPizzaList();
  },
  view() {
    return m("div.pa4", [
      m(
        "div",
        m("h2", "Pizza Signups"),
        Object.entries(Signin.pizzas).map(function(entry) {
          return m("p", entry[0] + ": " + entry[1]);
        })
      ),
      m(
        "div",
        m("h2", "Pizza Types List:"),
        Signin.pizzaList.map(function(pizza) {
          return m(
            "p",
            pizza + " ",
            m("a.underline", { onclick: deletePizza(pizza) }, "x")
          );
        })
      ),
      m(
        "div",
        { onsubmit: goToDate },
        m("form.center.measure.measure-narrow-ns.pa2", [
          m(
            "input.input-reset.w-100.pa2.ba.b--black-20[name='name'][placeholder='08-15-2018']",
            {
              oninput: m.withAttr("value", setDate),
              value: date
            }
          ),
          m(
            "input.input-reset.lh-copy.white.b.ba.br3.bw0.bg-orange.ph3.pv2.f6.pointer[type='submit'][value='Go to date']"
          )
        ])
      ),
      m(
        "div",
        { onsubmit: addPizza },
        m("form.center.measure.measure-narrow-ns.pa2", [
          m(
            "input.input-reset.w-100.pa2.ba.b--black-20[name='name'][placeholder='Pizza']",
            {
              oninput: m.withAttr("value", setPizza),
              value: pizza
            }
          ),
          m(
            "input.input-reset.lh-copy.white.b.ba.br3.bw0.bg-orange.ph3.pv2.f6.pointer[type='submit'][value='Add Pizza']"
          )
        ])
      ),
      m(
        "div",
        { onsubmit: addMember },
        m("form.center.measure.measure-narrow-ns.pa2", [
          m(
            "input.input-reset.w-100.pa2.ba.b--black-20[name='name'][placeholder='Name']",
            {
              oninput: m.withAttr("value", Member.current.name),
              value: Member.current.name()
            }
          ),
          m("div.mv3.flex.justify-around", [
            radio("731"),
            radio("5795"),
            radio("10195"),
            radio("6183"),
            radio("1533")
          ]),
          m(
            "input.input-reset.lh-copy.white.b.ba.br3.bw0.bg-orange.ph3.pv2.f6.pointer[type='submit'][value='Add Member']"
          )
        ])
      ),
      m("table.w-100.center.mb4", [
        m(
          "thead",
          m("tr.stripe-dark", [
            m("th", "ID"),
            m("th", "X"),
            m("th", "Name"),
            m("th", "Team"),
            m("th", "Last Pizza")
          ])
        ),
        m(
          "tbody",
          Array.from(Member.list.values()).map(member => {
            return m("tr.stripe-dark", [
              m("td.pa2", member.id),
              m("td.pa2", m("a", { onclick: deleteMember(member.id) }, "X")),
              m("td.pa2", member.name),
              m("td.pa2", member.team),
              m("td.pa2", member.last_pizza)
            ]);
          })
        )
      ]),
      m("h2", "Total Signins this Date: " + Signin.list.total),
      m("table.w-100.center", [
        m(
          "thead",
          m("tr.stripe-dark", [
            m("th", "ID"),
            m("th", "Name"),
            m("th", "Team"),
            m("th", "Pizza"),
            m("th", "DateTime in"),
            m("th", "DateTime out")
          ])
        ),
        m(
          "tbody",
          Object.values(Signin.list).map(signin => {
            let member = Member.list.get(signin.member_id);
            if (member !== undefined) {
              return m("tr.stripe-dark", [
                m("td.pa2", signin.id),
                m("td.pa2", member.name),
                m("td.pa2", member.team),
                m("td.pa2", signin.pizza),
                m("td.pa2", signin.date_in),
                m("td.pa2", signin.date_out)
              ]);
            } else {
              return null;
            }
          })
        )
      ])
    ]);
  }
};

export default Admin;

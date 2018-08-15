import r from "../utils/request";

const Signin = {
  list: { total: 0 },
  pizzaList: [],
  pizzas: {},
  clear() {
    Signin.pizzas = {};
    Signin.list = { total: 0 };
  },
  addPizza(pizza) {
    Signin.pizzaList.push(pizza);
    Signin.setPizzaList();
  },
  removePizza(pizza) {
    const index = Signin.pizzaList.indexOf(pizza);
    if (index > -1) {
      Signin.pizzaList.splice(index, 1);
    }
    Signin.setPizzaList();
  },
  getPizzaList() {
    return r({ url: "/pizzalist" }).then(data => {
      Signin.pizzaList = data.result.pizzas;
    });
  },
  setPizzaList() {
    return r({
      url: "/pizzalist",
      method: "POST",
      data: { pizzas: Signin.pizzaList }
    });
  },
  getDate(date) {
    const s = date.split("-");
    const month = s[0];
    const day = s[1];
    const year = s[2];
    Signin.clear();
    return r({
      url: "/signins/" + year + "/" + month + "/" + day,
      method: "GET"
    }).then(data => {
      data.signins.forEach(signin => {
        Signin.list[signin.id] = signin;
        Signin.list.total++;
        if (Signin.pizzas.hasOwnProperty(signin.pizza)) {
          Signin.pizzas[signin.pizza]++;
        } else {
          Signin.pizzas[signin.pizza] = 1;
        }
      });
    });
  },
  all() {
    Signin.clear();
    return r({
      url: "/signins",
      method: "GET"
    }).then(data => {
      data.signins.forEach(signin => {
        Signin.list[signin.id] = signin;
        Signin.list.total++;
        if (Signin.pizzas.hasOwnProperty(signin.pizza)) {
          Signin.pizzas[signin.pizza]++;
        } else {
          Signin.pizzas[signin.pizza] = 1;
        }
      });
    });
  },
  signin(id) {
    return Object.values(Signin.list).find(signin => signin.member_id == id);
  }
};

export default Signin;
